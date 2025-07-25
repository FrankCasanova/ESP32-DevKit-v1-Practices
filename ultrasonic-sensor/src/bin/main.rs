#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;

use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::main;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::time::Rate;
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let led = peripherals.GPIO33;

    // Configure LEDC
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config{
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();

    let mut channel0 = ledc.channel(channel::Number::Channel0, led);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,  // Use the previously configured timer
            duty_pct: 10,      // Initial duty cycle (10%)
            pin_config: channel::config::PinConfig::PushPull,  // Output mode
        })
        .unwrap();

    // Set up the Trigger Pin
    let mut trig = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());

    // Set up the Echo Pin
    let echo = Input::new(peripherals.GPIO18, InputConfig::default().with_pull(Pull::Down));

    let delay = Delay::new();

    let rtc = Rtc::new(peripherals.LPWR);

    
    

        loop {
        delay.delay_millis(5);

        // Trigger ultrasonic waves
        trig.set_low();
        delay.delay_micros(2);
        trig.set_high();
        delay.delay_micros(10);
        trig.set_low();

        // Measure the duration the signal remains high
        while echo.is_low() {}
        let time1 = rtc.current_time_us();
        while echo.is_high() {}
        let time2 = rtc.current_time_us();
        let pulse_width = time2 - time1;

        // Derive distance from the pulse width
        let distance = (pulse_width as f64 * 0.0343) / 2.0;
        // esp_println::println!("Pulse Width: {}", pulse_width);
        // esp_println::println!("Distance: {}", distance);

        // Our own logic to calculate duty cycle percentage for the distance
        let duty_pct: u8 = if distance < 30.0 {
            let ratio = (30.0 - distance) / 30.0;
            let p = (ratio * 100.0) as u8;
            p.min(100)
        } else {
            0
        };

        if let Err(e) = channel0.set_duty(duty_pct) {
            // esp_println::println!("Failed to set duty cycle: {:?}", e);
            panic!("Failed to set duty cycle: {:?}", e);
        }

        delay.delay_millis(60);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
