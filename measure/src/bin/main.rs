#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]


use esp_hal::clock::CpuClock;
use log::info;
use esp32_dht11_rs::DHT11;
use embedded_hal::delay::DelayNs;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// Simple delay function using busy waiting
// This is like counting to yourself while waiting
fn simple_delay_ms(ms: u32) {
    let start = Instant::now();
    let duration = Duration::from_millis(ms as u64);
    while start.elapsed() < duration {
        // Busy wait - think of this as pacing back and forth while waiting
    }
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    // Create output pin for DHT11 communication
    // Think of this as setting up a communication line to the warehouse sensor
    let delay = Delay::new();

    let mut dht11 = DHT11::new(peripherals.GPIO2, delay);

    loop {
        match dht11.read() {
            Ok(m) => info!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                m.temperature, m.humidity
            ),
            Err(error) => info!("An error occurred while trying to read sensor: {:?}", error),
        }
        delay.delay_millis(1000);
    }
}




