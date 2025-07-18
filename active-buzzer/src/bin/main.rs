#![no_std] // Embedded Rust uses no_std to avoid OS dependencies and heap allocation
#![no_main] // Custom main function signature for bare-metal execution
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)] // Enforce memory safety by disabling unsafe practices

use esp_hal::clock::CpuClock; // CPU clock configuration
use esp_hal::gpio::{Level, Output, OutputConfig}; // GPIO pin handling
use esp_hal::time::{Duration, Instant}; // Timing utilities
use esp_hal::main; // Main entry point 
use active_buzzer::p_buzzer::Buzzer;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
} // Custom panic handler to prevent system crash on errors

// Creates application descriptor required by ESP-IDF bootloader
// Similar to a factory's product spec sheet for the bootloader
esp_bootloader_esp_idf::esp_app_desc!();

pub fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
} // Busy-wait delay function using time measurement, like a stopwatch

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    // Configure CPU clock to maximum for performance
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config); // Initialize hardware peripherals

    // Create GPIO33 as an output pin, taking ownership of the peripheral
    // Ownership ensures only this part of the code can control the pin
    let buzzer_pin = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
    let mut buzzer = Buzzer::new(buzzer_pin);
    
    buzzer.connected_sound();

    loop {
         // Wait 500ms
         buzzer.monitoring_sound();
    } // Infinite loop to create continuous beeping

    // For more examples and inspiration, check the esp-hal examples:
    // https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
