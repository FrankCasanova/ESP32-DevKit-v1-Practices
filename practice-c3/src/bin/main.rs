#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::prelude::Point;
use embedded_graphics::Drawable;
use esp_hal::gpio::{Input, InputConfig, Level, OutputConfig, Pull};
use esp_hal::time::Rate;
use esp_hal::{clock::CpuClock, gpio::Output};
use esp_hal::timer::systimer::SystemTimer;
use ssd1306::mode::DisplayConfigAsync;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306Async};
use embedded_graphics::mono_font::iso_8859_10::FONT_6X10;
use heapless::String;
use core::fmt::Write;
use {esp_backtrace as _, esp_println as _};

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

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let button = Input::new(peripherals.GPIO6, InputConfig::default().with_pull(Pull::Up));
    let mut button_was_pressed = false;

    let mut led = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let mut led_is_on = false;

    let mut buzzer = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    
        // Configure display
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO4)
    .with_sda(peripherals.GPIO3)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display = Ssd1306Async::new(
        interface, 
        DisplaySize128x64, 
        DisplayRotation::Rotate0
    )
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
        
        let button_is_pressed = button.is_low();

        if button_is_pressed && !button_was_pressed {
            if led_is_on {
                led.set_low();
                buzzer_led_off(&mut buzzer).await;
                led_is_on = false;
            } else {
                led.set_high();
                buzzer_led_on(&mut buzzer).await;
                led_is_on = true;
            }

            Timer::after(Duration::from_millis(200)).await;
        }
        
        // Display message when LED is on
        if led_is_on {
            write!(buffer, "Led Green is ON!").unwrap();
            Text::with_baseline(&buffer, Point::new(20, 20), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
        }
        
        button_was_pressed = button_is_pressed;
        
        display.flush().await.unwrap();

        Timer::after(Duration::from_millis(20)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}


async fn buzzer_led_on(buzzer: &mut Output<'static>) {
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_high();
    Timer::after(Duration::from_millis(33)).await;
    buzzer.set_low();
}

async fn buzzer_led_off(buzzer: &mut Output<'static>) {
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
