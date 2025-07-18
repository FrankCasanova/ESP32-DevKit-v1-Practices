#![no_std]  // This tells Rust to not include the standard library, essential for bare-metal embedded development
#![no_main] // Indicates that this is a no_main application, which doesn't use the standard main function setup

use esp_hal::clock::CpuClock;  // Import CPU clock configuration options
use esp_hal::ledc::channel::ChannelIFace;  // LEDC channel interface
use esp_hal::ledc::timer::TimerIFace;  // LEDC timer interface
use esp_hal::ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed};  // Various LEDC components
use esp_hal::main;  // Import the main attribute macro for entry point
use esp_hal::time::Rate;  // Import rate configuration for timers
use esp_println::println;  // A print macro that works without std

#[panic_handler]  // Define a custom panic handler since there's no OS to handle panics
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}  // On panic, enter an infinite loop to halt the system
}

esp_bootloader_esp_idf::esp_app_desc!();  // Generate app description for ESP bootloader

#[main]  // Mark this as the entry point for the application
fn main() -> ! {
    // generator version: 0.3.1

    // Configure the CPU clock speed
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    // Initialize the ESP32 peripherals with the given configuration
    let peripherals = esp_hal::init(config);

    // Assign GPIO pins 23 and 22 to control the LEDs
    let led1 = peripherals.GPIO23;
    let led2 = peripherals.GPIO22;

    // Initialize the LEDC peripheral
    let mut ledc = Ledc::new(peripherals.LEDC);
    // Set the global clock source for LEDC to APB clock
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    
    // Configure LEDC timer 0 for low-speed channel
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    // Set timer configuration: 5-bit duty cycle, APB clock source, 12kHz frequency
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(12),
        })
        .unwrap();  // Assume configuration succeeds; in production, handle errors here

    // Configure channel 0 for LED1
    let mut channel0 = ledc.channel(channel::Number::Channel0, led1);
    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,  // Use the previously configured timer
            duty_pct: 10,      // Initial duty cycle (10%)
            pin_config: channel::config::PinConfig::PushPull,  // Output mode
        })
        .unwrap();

    // Configure channel 1 for LED2 similarly
    let mut channel1 = ledc.channel(channel::Number::Channel1, led2);
    channel1
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    println!("STARTING THE PROGRAM");
    
    loop {  // Main loop to continuously fade the LEDs
        // Fade LED1 from 0% to 100% over 2 seconds
        channel0.start_duty_fade(0, 100, 2000).unwrap();
        while channel0.is_duty_fade_running() {}  // Wait until fade completes
        
        // Fade LED2 from 0% to 100% over 2 seconds
        channel1.start_duty_fade(0, 100, 2000).unwrap();
        while channel1.is_duty_fade_running() {}
        
        // Fade LED1 from 100% to 0% over 0.5 seconds
        channel0.start_duty_fade(100, 0, 500).unwrap();
        while channel0.is_duty_fade_running() {}
        
        // Fade LED2 from 100% to 0% over 0.5 seconds
        channel1.start_duty_fade(100, 0, 500).unwrap();
        while channel1.is_duty_fade_running() {}
    }
}
