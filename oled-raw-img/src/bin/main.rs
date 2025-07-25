#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use esp_hal::{clock::CpuClock, time::Rate};
use esp_hal::timer::timg::TimerGroup;
use ssd1306::{mode::DisplayConfigAsync, prelude::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async};
use embedded_graphics::Drawable;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// 32x32 pixel happy face
#[rustfmt::skip]
const IMG_DATA: &[u8] = &[
    // Row 1-8: Top of head
    0b00000111,0b11111111,0b11111111,0b11100000,
    0b00111111,0b11111111,0b11111111,0b11111000,
    0b01111111,0b11111111,0b11111111,0b11111110,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    
    // Row 9-16: Eyes area
    0b11111111,0b11000011,0b11000011,0b11111111,
    0b11111111,0b11000011,0b11000011,0b11111111,
    0b11111111,0b11000011,0b11000011,0b11111111,
    0b11111111,0b11000011,0b11000011,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    
    // Row 17-24: Smile start
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b11111110,0b11111111,0b11111111,0b01111111,
    0b11111100,0b01111111,0b11111100,0b00111111,
    0b11111100,0b00000000,0b00000000,0b00111111,
    0b11111110,0b00000000,0b00000000,0b01111111,
    0b11111111,0b00000000,0b00000000,0b11111111,
    0b11111111,0b11111111,0b11111111,0b11111111,
    
    // Row 25-32: Bottom of face
    0b11111111,0b11111111,0b11111111,0b11111111,
    0b01111111,0b11111111,0b11111111,0b11111110,
    0b00111111,0b11111111,0b11111111,0b11111100,
    0b00011111,0b11111111,0b11111111,0b11111000,
    0b00000111,0b11111111,0b11111111,0b11100000,
    0b00000011,0b11111111,0b11111111,0b11000000,
    0b00000000,0b11111111,0b11111111,0b00000000,
    0b00000000,0b00111111,0b11111100,0b00000000,
];


#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);

    // initialize the display
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    let raw_image= ImageRaw::<BinaryColor>::new(IMG_DATA, 32);
    let image = Image::new(&raw_image, Point::new(48, 16));

    image.draw(&mut display).unwrap();
    display.flush().await.unwrap();
    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
