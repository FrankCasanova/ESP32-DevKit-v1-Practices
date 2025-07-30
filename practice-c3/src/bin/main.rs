#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use alloc::string::{self, ToString};
use core::cell::RefCell;
use core::cell::UnsafeCell;
use core::fmt::Write;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_dht_rs::dht22::{self, Dht22};
use embedded_graphics::mono_font::iso_8859_10::{FONT_6X10, FONT_7X13_BOLD};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use esp_hal::delay::Delay;
use esp_hal::gpio::{DriveMode, Flex, Input, InputConfig, Level, OutputConfig, Pull};
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::{clock::CpuClock, gpio::Output};
use esp_println::println;
use heapless::{Deque, String};
use ssd1306::mode::DisplayConfigAsync;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306Async};
use static_cell::StaticCell;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// Static cell for sharing temperature data between tasks
// (temperature, humidity)

// Static cell for sharing temperature data between tasks
static TEMPERATURE_DATA: StaticCell<UnsafeCell<(f32, f32)>> = StaticCell::new();

#[embassy_executor::task]
async fn temperature_task(
    mut dht22: Dht22<Flex<'static>, Delay>,
    temperature_data: &'static UnsafeCell<(f32, f32)>,
) {
    loop {
        match dht22.read() {
            Ok(sensor_reading) => {
                // Update the shared temperature data
                unsafe {
                    *temperature_data.get() = (
                        sensor_reading.temperature,
                        sensor_reading.humidity,
                    );
                }
            }
            Err(_) => {
                continue;
            }
        }
        Timer::after(Duration::from_secs(2)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let button = Input::new(
        peripherals.GPIO6,
        InputConfig::default().with_pull(Pull::Up),
    );
    let mut button_was_pressed = false;

    let mut led = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let mut led_is_on = false;

    let mut buzzer = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());

    // Configure display
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO4)
    .with_sda(peripherals.GPIO3)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display: Ssd1306Async<
        ssd1306::prelude::I2CInterface<esp_hal::i2c::master::I2c<'_, esp_hal::Async>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsModeAsync<DisplaySize128x64>,
    > = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();
    let text_style: embedded_graphics::mono_font::MonoTextStyle<'_, BinaryColor> =
        MonoTextStyleBuilder::new()
            .font(&FONT_7X13_BOLD)
            .text_color(BinaryColor::On)
            .build();

    let mut buffer: String<25> = String::new();
    let mut buffer2: String<64> = String::new();

    let od_config = OutputConfig::default()
        .with_drive_mode(DriveMode::OpenDrain)
        .with_pull(Pull::None);

    let od_for_dht22 =
        Output::new(peripherals.GPIO0, esp_hal::gpio::Level::High, od_config).into_flex();

    od_for_dht22.peripheral_input();

    let delay = Delay::new();

    let dht22 = dht22::Dht22::new(od_for_dht22, delay);

    // Spawn the temperature reading task
    let temperature_data = TEMPERATURE_DATA.init(UnsafeCell::new((0.0, 0.0)));
    spawner.spawn(temperature_task(dht22, temperature_data)).unwrap();

    // Give the sensor some time to initialize
    Timer::after(Duration::from_secs(2)).await;

    loop {
        buffer.clear();
        buffer2.clear();
        display.clear_buffer();

        let (temperature, humidity) = unsafe { *temperature_data.get() };
        write!(buffer2, "{} {}", temperature, humidity).unwrap();
            Text::with_baseline(&buffer2, Point::new(0, 0), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

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
            Text::with_baseline(&buffer, Point::new(10, 20), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();
        } else {
            write!(buffer, "Led Green is OFF!").unwrap();
            Text::with_baseline(&buffer, Point::new(10, 20), text_style, Baseline::Top)
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
