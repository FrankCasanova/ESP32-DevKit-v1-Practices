use blocking_network_stack::Stack;
use defmt::info;
use embedded_io::{Read, Write};
use esp_hal::time::Instant;
use esp_hal::{delay::Delay, time::Duration};
use esp_println as _;
use esp_println::println;
use esp_wifi::wifi::WifiController;
use smoltcp::wire::IpAddress;

// Imagine the ESP32 as a warehouse factory. The `create_interface` function sets up
// the network interface, similar to setting up the communication channels between
// different parts of the factory.
pub fn create_interface(device: &mut esp_wifi::wifi::WifiDevice) -> smoltcp::iface::Interface {
    // Users could create multiple instances but since they only
    // have one WifiDevice they probably can't do anything bad with that
    smoltcp::iface::Interface::new(
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        device,
        timestamp(),
    )
}

// Think of `timestamp` as the factory's clock, keeping track of time for all operations.
fn timestamp() -> smoltcp::time::Instant {
    smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros() as i64,
    )
}

// `scan_wifi` is like the factory's inventory system, scanning for available resources (WiFi networks).
pub fn scan_wifi(controller: &mut WifiController<'_>) {
    info!("Start Wifi Scan");
    let res = controller.scan_n(10);
    if let Ok(res) = res {
        for ap in res {
            info!("{:?}", ap);
        }
    }
}

// `connect_wifi` is the process of connecting the factory to the supply chain (WiFi network).
pub fn connect_wifi(
    controller: &mut WifiController<'_>,
    stack: &mut Stack<'_, esp_wifi::wifi::WifiDevice<'_>>,
) {
    println!("{:?}", &controller.capabilities());
    info!("wifi_connect {:?}", &controller.connect());

    info!("Wait to get connected");
    loop {
        match controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {
                println!("connecting...");
                Delay::new().delay(Duration::from_millis(500));
            }
            Err(err) => panic!("{:?}", err),
        }
    }
    info!("Connected: {:?}", &controller.is_connected());

    optain_ip(stack);
}

// `optain_ip` is like getting the factory's address in the supply chain, necessary for communication.
fn optain_ip(stack: &mut Stack<'_, esp_wifi::wifi::WifiDevice<'_>>) {
    info!("Wait for IP address");
    loop {
        stack.work();
        if stack.is_iface_up() {
            let ip_info = stack.get_ip_info();
            println!("IP acquired{:?}", ip_info);
            break;
        }
    }
}

// `http_loop` is the continuous process of sending and receiving goods (data) in the factory.
pub fn http_loop(
    socket: &mut blocking_network_stack::Socket<'_, '_, esp_wifi::wifi::WifiDevice<'_>>,
) -> ! {
    info!("Starting HTTP client loop");
    let delay = Delay::new();
    loop {
        info!("Making HTTP request");
        socket.work();

        // The `remote_addr` is like the address of the supplier (server) we want to order goods from.
        let remote_addr = IpAddress::v4(142, 250, 185, 115);
        socket.open(remote_addr, 80).unwrap();

        // Here we're sending an order (HTTP request) to the supplier.
        socket
            .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
            .unwrap();
        socket.flush().unwrap();

        // The `buffer` is like the warehouse's storage area where incoming goods (data) are placed.
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut buffer = [0u8; 512];
        while let Ok(len) = socket.read(&mut buffer) {
            // Convert the received data into a readable format (text).
            let Ok(text) = core::str::from_utf8(&buffer[..len]) else {
                panic!("Invalid UTF-8 sequence encountered");
            };

            println!("{}", text);

            // If the deadline is reached, stop waiting for more goods.
            if Instant::now() > deadline {
                info!("Timeout");
                break;
            }
        }

        // Disconnect from the supplier after receiving the order.
        socket.disconnect();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            socket.work();
        }

        // Wait before placing the next order.
        delay.delay_millis(1000);
    }
}
