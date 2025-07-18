#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    // ***********LED********************
    let mut led_pin = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
    //***********************************

    //************LDR***************************************************
    let adc_pin = peripherals.GPIO4;
    let mut adc2_config = AdcConfig::new();
    let mut pin = adc2_config.enable_pin(adc_pin, Attenuation::_11dB);
    let mut adc2 = Adc::new(peripherals.ADC2, adc2_config);
    //******************************************************************

    let delay = Delay::new();
    loop {
        let pin_value: u16 = nb::block!(adc2.read_oneshot(&mut pin)).unwrap();
        
        info!("{pin_value}");
        
        if pin_value > 3300 {
            led_pin.set_high();
        } else {
            led_pin.set_low();
        }
        
        delay.delay_millis(5000);
    }
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
