#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::fmt::Write;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::iso_8859_10::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use esp_backtrace as _;
use esp_hal::analog::adc::AdcConfig;
use esp_hal::analog::adc::{Adc, Attenuation};
use esp_hal::gpio::{InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, gpio::Input};
use esp_println::println;
use heapless::{String, Vec};
use ssd1306::mode::DisplayConfigAsync;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306Async};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    // Initialize LED on GPIO15 (built-in LED on many ESP32 boards)
    let mut led_green = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    let mut led_green_is_on = false;

    let mut led_yellow = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let mut led_yellow_is_on = false;

    let mut buzzer = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    // Initialize button with pull-up resistor
    // Correct wiring: One leg of button to GPIO14, other leg to GND
    let blue_button = Input::new(
        peripherals.GPIO14,
        InputConfig::default().with_pull(Pull::Up),
    );
    let mut blue_button_was_pressed = false;

    let white_button = Input::new(
        peripherals.GPIO12,
        InputConfig::default().with_pull(Pull::Up),
    );
    let mut white_button_was_pressed = false;

    // Configure display
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut buffer: String<25> = String::new();

    loop {
        buffer.clear();
        display.clear_buffer();
        
        let blue_button_is_pressed = blue_button.is_low();
        let white_button_is_pressed = white_button.is_low();

        // Detect button press (transition from not pressed to pressed)
        if blue_button_is_pressed && !blue_button_was_pressed {
            info!("Blue Button pressed - toggling LED");
            

            // Toggle LED state
            if led_green_is_on {
                buzzer_bip(&mut buzzer).await;
                led_green.set_low();
                led_green_is_on = false;
                info!("LED Green turned OFF");
            } else {
                write!(buffer, "Led Green is ON!").unwrap();
                Text::with_baseline(&buffer, Point::new(90, 0), text_style, Baseline::Top)
                    .draw(&mut display)
                    .unwrap();
                buzzer_bip(&mut buzzer).await;
                led_green.set_high();
                led_green_is_on = true;
                info!("LED Green turned ON");
            }

            // Simple debounce - wait a bit after detecting press
            Timer::after(Duration::from_millis(200)).await;
        }

        if white_button_is_pressed && !white_button_was_pressed {
            info!("White Button pressed");

            if led_yellow_is_on {
                buzzer_bip_yellow(&mut buzzer).await;
                led_yellow.set_low();
                led_yellow_is_on = false;
                info!("LED Yellow OFF")
            } else {
                buzzer_bip_yellow(&mut buzzer).await;
                led_yellow.set_high();
                led_yellow_is_on = true;
                info!("LED Yellow ON");
            }

            Timer::after(Duration::from_millis(200)).await;
        }

        display.flush().await.unwrap();

        blue_button_was_pressed = blue_button_is_pressed;
        white_button_was_pressed = white_button_is_pressed;
        // Main loop delay
        Timer::after(Duration::from_millis(10)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

async fn buzzer_bip(buzzer: &mut Output<'static>) {
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
}

async fn buzzer_bip_yellow(buzzer: &mut Output<'static>) {
    buzzer.set_high();
    Timer::after(Duration::from_millis(66)).await;
    buzzer.set_low();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
}
