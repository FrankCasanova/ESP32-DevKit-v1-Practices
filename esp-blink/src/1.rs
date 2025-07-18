// fn main() {
//     // It is necessary to call this function once. Otherwise some patches to the runtime
//     // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
//     esp_idf_svc::sys::link_patches();

//     // Bind the log crate to the ESP Logging facilities
//     esp_idf_svc::log::EspLogger::initialize_default();

//     log::info!("Hello, world!");
// }
use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::sys::link_patches;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::*;

fn main() {
    // Ensure runtime patches are linked correctly
    link_patches();

    // Enable logging
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting LED blink example...");

    // Access GPIO peripherals
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio22).unwrap();

    loop {
        log::info!("LED ON");
        led.set_high().unwrap();
        sleep(Duration::from_millis(100));

        log::info!("LED OFF");
        led.set_low().unwrap();
        sleep(Duration::from_millis(100));
    }
}
