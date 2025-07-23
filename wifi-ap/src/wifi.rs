use core::{net::Ipv4Addr, str::FromStr};

use crate::led::LogLed;
use crate::mk_static;
use anyhow::anyhow;
use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Runner, Stack, StackResources, StaticConfigV4};
use embassy_time::{Duration, Timer};
use esp_hal::rng::Rng;
use esp_println::println;
use esp_wifi::{
    wifi::{self, WifiController, WifiDevice, WifiEvent, WifiState},
    EspWifiController,
};

// Unlike Station mode, You can give any IP range(private) that you like
// IP Address/Subnet mask eg: STATIC_IP=192.168.13.37/24
const STATIC_IP: &str = "192.168.2.1/24";
// Gateway IP eg: GATEWAY_IP="192.168.13.37"
const GATEWAY_IP: &str = "192.168.2.1";

const PASSWORD: &str = "1QAZ2wsx";
const SSID: &str = "esp32-wifi";

pub async fn start_wifi(
    esp_wifi_cntrl: &'static EspWifiController<'static>, // a panel on the wall
    wifi: esp_hal::peripherals::WIFI<'static>,           //antena
    mut rng: Rng,                                        //a dice
    spawner: &Spawner, //a guy giving task through its walkie-talkie
    mut led: LogLed,
) -> anyhow::Result<Stack<'static>> {
    //put the chip-wifi in the panel device
    led.boot().await;
    info!("setting up controller and interface");
    let (controller, interface) = esp_wifi::wifi::new(&esp_wifi_cntrl, wifi).unwrap();

    // set the panel device as a hostpot
    led.event().await;
    info!("setting up wifi interface");
    let wifi_interface = interface.ap;

    // roll two dices, important
    led.event().await;
    info!("setting up net seeds");
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // Parse STATIC_IP
    led.event().await;
    info!("setting up ip gateway");
    let ip_addr =
        Ipv4Cidr::from_str(STATIC_IP).map_err(|_| anyhow!("Invalid GATEWAY_IP: {STATIC_IP}"))?;

    // Parse GATEWAY_IP
    led.event().await;
    info!("setting up ip address");
    let gateway =
        Ipv4Addr::from_str(GATEWAY_IP).map_err(|_| anyhow!("Invalid GATEWAY_IP: {GATEWAY_IP}"))?;

    // Create a Network config with IP details
    led.event().await;
    info!("net config initiated");
    let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: ip_addr,
        gateway: Some(gateway),
        dns_servers: Default::default(),
    });

    // Init network stack
    led.info().await;
    info!("Initializing network stack");
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    led.event().await;
    led.info().await;
    info!("Connection task started");
    spawner.spawn(connection_task(controller)).ok();

    led.event().await;
    led.info().await;
    info!("Net task started");
    spawner.spawn(net_task(runner)).ok();

    led.event().await;
    led.info().await;
    info!("waiting for connections");
    wait_for_connection(stack).await;

    led.success().await;
    Ok(stack)
}

async fn wait_for_connection(stack: Stack<'_>) {
    info!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            info!("link up");
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Connect to the AP `esp-wifi` and point your browser to http://192.168.13.37/24/");
    while !stack.is_config_up() {
        Timer::after(Duration::from_millis(100)).await
    }
    info!("stack granted");
    stack
        .config_v4()
        .inspect(|c| println!("ipv4 config: {c:?}"));
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async fn connection_task(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::ApStarted => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::ApStop).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = wifi::Configuration::AccessPoint(wifi::AccessPointConfiguration {
                ssid: SSID.try_into().unwrap(),
                password: PASSWORD.try_into().unwrap(),
                auth_method: esp_wifi::wifi::AuthMethod::WPA2Personal,
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
    }
}
