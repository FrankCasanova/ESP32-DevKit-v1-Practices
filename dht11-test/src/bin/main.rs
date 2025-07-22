#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

// use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{DriveMode, Output, OutputConfig, Pull};
use esp_hal::main;
// use esp_hal::time::{Duration, Instant};
use esp_println::println;
use embedded_dht_rs::dht11;
use esp_hal::delay::Delay;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let od_config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::None);

    let od_for_dht11 = Output::new(
            peripherals.GPIO4, esp_hal::gpio::Level::High, od_config
        )
        .into_flex();

    od_for_dht11.peripheral_input();
    

    let delay = Delay::new();

    let mut dht11 = dht11::Dht11::new(od_for_dht11, delay);
    println!("wait to set up the dht11, 5 secs...");
    delay.delay_millis(5000);
    loop {
        println!("---reading");
        match dht11.read() {
            Ok(sensor_reading) => println!(
                "DHT11 Sensor - Temperature: {} C , Humidity: {} %",
                sensor_reading.temperature,
                sensor_reading.humidity
            ),
            Err(e) => println!("error {e:?}"),
        }
        println!("_____________________________________________________");
        delay.delay_millis(2000);
    }
// }for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}