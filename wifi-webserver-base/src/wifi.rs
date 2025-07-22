use core::net::Ipv4Addr;
use core::str::FromStr;

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_net::{Runner, Stack, StackResources, StaticConfigV4};
use embassy_time::{Duration, Timer};
use esp_hal::rng::Rng;
use esp_println::println;
use esp_wifi::wifi::{self, WifiController, WifiDevice, WifiEvent, WifiState};
use esp_wifi::EspWifiController;
//new imports
use smoltcp::wire::Ipv4Cidr;
use heapless::Vec;

use crate::mk_static;

const SSID: &str = "SSID";
const PASSWORD: &str = "PASSWORD";
// IP Address/Subnet mask eg: STATIC_IP=192.168.0.50/24
const STATIC_IP: &str = "192.168.0.50/24";
const GATEWAY_IP: &str = "192.168.0.1";

pub async fn start_wifi(
    esp_wifi_ctrl: &'static EspWifiController<'static>,
    wifi: esp_hal::peripherals::WIFI<'static>,
    mut rng: Rng,
    spawner: &Spawner,
) -> Stack<'static> {
    let (wifi_controller, interfaces) =
        esp_wifi::wifi::new(&esp_wifi_ctrl, wifi).expect("Failed to initialize WIFI controller");

    let wifi_interface = interfaces.sta;
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // let dhcp_config = DhcpConfig::default();
    let Ok(ip_addr) = Ipv4Cidr::from_str(STATIC_IP) else {
        println!("Invalid STATIC_IP");
        loop {}
    };
    let Ok(gateway) = Ipv4Addr::from_str(GATEWAY_IP) else {
        println!("Invalid GATEWAY_IP");
        loop {}
    };
    let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: ip_addr,
        gateway: Some(gateway),
        dns_servers: Vec::new(),
    });

    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection_task(wifi_controller)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();

    wait_for_connection(stack).await;

    stack
}

async fn wait_for_connection(stack: Stack<'static>) {
    info!("Waiting for link to be up");
    loop {
        match stack.is_link_up() {
            true => break,
            false => Timer::after(Duration::from_millis(500)).await,
        }
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP:");
            println!("{}", config.address);
            break;
        } else {
            Timer::after(Duration::from_millis(500)).await
        }
    }
}

#[embassy_executor::task]
async fn connection_task(mut controller: WifiController<'static>) {
    info!("Start connection task");
    info!("Device capabilities:");
    println!("{:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaDisconnected => {
                // wait until w're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(500)).await;
            }
            _ => info!("WiFi connected"),
        }

        if !matches!(controller.is_connected(), Ok(true)) {
            let client_config = wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: SSID.try_into().unwrap(),
                password: PASSWORD.try_into().unwrap(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            info!("Starting WiFi...");
            controller.start_async().await.unwrap();
            info!("Wifi started")
        }

        info!("About to connect...");
        match controller.connect_async().await {
            Ok(_) => info!("WiFi connected."),
            Err(e) => {
                error!("Failed to connect to wifi:");
                println!("{:?}", e);
                Timer::after(Duration::from_millis(500)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
