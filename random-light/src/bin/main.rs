#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant};
use random_light::ledc::{setup_ledc_timer, setup_ledc_channel};


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
    // generator version: 0.4.0

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(size: 64 * 1024);
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    let led = peripherals.GPIO23;

    let (ledc, timer) = setup_ledc_timer(peripherals.LEDC);
    let channel0 = setup_ledc_channel(ledc, &timer, led);

    let mut rng = Rng::new(peripherals.RNG);

    // Generate a random word (u32):

    // Fill a buffer with random bytes:
    let mut buf = [0u8; 16];
    let _number = rng.read(&mut buf);

    // Keep timer alive for the duration of the program (it's already owned by this scope)

    loop {
        // info!("Hello world!");
        let rand_word = rng.random() % 100;
        let bits_8 = rand_word as u8;
        // info!("Random word: {}", rand_word);
        // info!("Random number: {:?}", number);
        channel0.set_duty(bits_8 as u8).unwrap();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
