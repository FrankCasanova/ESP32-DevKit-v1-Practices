#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::analog::adc::{Adc, AdcConfig};
use esp_hal::clock::CpuClock;
use esp_hal::ledc::{HighSpeed, LSGlobalClkSource, Ledc, timer, channel};
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::{main};
use esp_hal::time::{Duration, Instant, Rate};
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
    
    let potentiometer = peripherals.GPIO34;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(potentiometer, esp_hal::analog::adc::Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);
    
    let led = peripherals.GPIO23;
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
        hstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty5Bit,
                clock_source: timer::HSClockSource::APBClk,
                frequency: Rate::from_khz(24),
            })
            .unwrap();
    
    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 50,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    
    
    
    loop {
        let pot_value = match adc1.read_oneshot(&mut pin) {
            Ok(value) => {
                // info!("Potentiometer value: {}", value);
                value // Return the value if the read was successful
            }
            Err(_) => {
                continue;
            }
        };
        
        let duty_cycle = (pot_value as f32 / 4095.0) * 100.0;
        
        channel0.set_duty(duty_cycle as u8);
        
        
        // let delay_start = Instant::now();
        // while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
