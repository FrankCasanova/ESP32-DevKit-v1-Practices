#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_net::dns::DnsSocket;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{DhcpConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use embedded_io_async::Read;
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_wifi::wifi::{self, WifiController, WifiDevice, WifiEvent, WifiState};
use esp_wifi::EspWifiController;
use reqwless::client::{HttpClient, TlsConfig};
use reqwless::request::RequestBuilder;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// If you are okay with using a nightly compiler, you can use the macro provided by the static_cell crate: https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

esp_bootloader_esp_idf::esp_app_desc!();
const SSID: &str = "SSID";
const PASSWORD: &str = "PASSWORD";

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    // let _init = esp_wifi::init(
    //     timer1.timer0,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    // )
    // .unwrap();
    let mut rng = Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng).unwrap()
    );

    let (controller, interfaces) = esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();
    let wifi_interface = interfaces.sta;

    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);
    let tls_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let dhcp_config = DhcpConfig::default();
    // dhcp_config.hostname = Some(String::from_str("implRust").unwrap());

    let config = embassy_net::Config::dhcpv4(dhcp_config);
    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    wait_for_connection(stack).await;

    access_website(stack, tls_seed).await
}

async fn wait_for_connection(stack: Stack<'_>) {
    println!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: SSID.try_into().unwrap(),
                password: PASSWORD.try_into().unwrap(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
        println!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

async fn access_website(stack: Stack<'_>, tls_seed: u64) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let dns = DnsSocket::new(stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(stack, &tcp_state);

    let tls = TlsConfig::new(
        tls_seed,
        &mut rx_buffer,
        &mut tx_buffer,
        reqwless::client::TlsVerify::None,
    );
    info!("TLS config created");
    
    let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);
    let mut buffer = [0u8; 1024]; // Buffer for request headers

    info!("Starting TLS handshake");
    let mut http_req = match client
        .request(
            reqwless::request::Method::GET,
            "https://readmedium.com/nodejs-internals-and-architecture-0c3488c3fc49",
        )
        .await 
    {
        Ok(req) => req
            .headers(&[("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")])
            .headers(&[("Accept", "text/html")]),
        Err(e) => {
            error!("Error creating request");
            println!("{e:?}");
            return;
        }
    };

    info!("TLS handshake completed");

    // Send request
    let response = match http_req.send(&mut buffer).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Error sending request");
            println!("{e:?}");
            return;
        }
    };

    info!("Request made");
    println!("Status Code: {:?}", response.status);

    // Read the body in chunks
    let mut body_reader = response.body().reader();
    let mut chunk_count = 0;
    let deadline = embassy_time::Instant::now() + Duration::from_secs(30);

    while embassy_time::Instant::now() < deadline {
        let mut chunk_buffer = [0u8; 512];
        match body_reader.read(&mut chunk_buffer).await {
            Ok(0) => {
                info!("End of response");
                break;
            }
            Ok(len) => {
                chunk_count += 1;
                match core::str::from_utf8(&chunk_buffer[..len]) {
                    Ok(text) => {
                        println!("--- Chunk {} ---", chunk_count);
                        println!("{}", text);
                    }
                    Err(e) => {
                        info!("Received non-UTF8 data in chunk");
                        println!("{e:?}");
                    }
                }
            }
            Err(e) => {
                info!("Response bytes finished o Chunk error...");
                println!("{e:?}");
                break;
            }
        }
    }
}

