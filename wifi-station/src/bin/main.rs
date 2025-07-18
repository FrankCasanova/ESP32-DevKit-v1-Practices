#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use blocking_network_stack::Stack;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::Instant;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_println::println;

// New imports
use esp_hal::peripherals::Peripherals;
use esp_wifi::wifi;
use smoltcp::iface::{SocketSet, SocketStorage};
use smoltcp::wire::DhcpOption;

// My modules
use wifi_station::wifi_setup::{connect_wifi, create_interface, http_loop, scan_wifi};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// Helper function to initialize hardware
// Initialize hardware peripherals
fn init_hardware() -> Peripherals {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);
    peripherals
}

const SSID: &str = "sagemcomD440";
const PASSWORD: &str = "QMN2Q2YWUWMXEM";

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let peripherals = init_hardware();

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = esp_wifi::init(timg0.timer0, rng).unwrap();
    let (mut controller, interfaces) =
        esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();
    let mut device = interfaces.sta;

    // SocketSet initialization
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);

    // DHCP Socket
    let mut dhpc_socket = smoltcp::socket::dhcpv4::Socket::new();

    // We can set a hostname here (or add other DHCP options)
    dhpc_socket.set_outgoing_options(&[DhcpOption {
        kind: 12,
        data: b"implFrank",
    }]);
    socket_set.add(dhpc_socket);

    // Initializing the Network Stack
    let now = || Instant::now().duration_since_epoch().as_millis();
    let mut stack = Stack::new(
        create_interface(&mut device),
        device,
        socket_set,
        now,
        rng.random(),
    );

    // WiFi Operation Mode
    let client_config = wifi::Configuration::Client(wifi::ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });

    let res = controller.set_configuration(&client_config);
    println!("wifi_set_configuration_returned:{:?}", res);

    //Start the wifi controller
    controller.start().unwrap();

    scan_wifi(&mut controller);
    connect_wifi(&mut controller, &mut stack);

    // Build the buffer for the socket
    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];

    // Create a socket, is nothing more than 2 spaces in memory, just that.
    let mut socket: blocking_network_stack::Socket<'_, '_, wifi::WifiDevice<'_>> =
        stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    http_loop(&mut socket);

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
