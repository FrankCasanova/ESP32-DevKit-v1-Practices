#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use music_buzzer::music::{self, Song};
use esp_hal::clock::CpuClock;
use esp_hal::ledc::{channel, timer, HighSpeed, Ledc, LowSpeed};
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use music_buzzer::pink_panther;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();
// Helper function
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
#[main]
fn main() -> ! {
    // generator version: 0.4.0
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);
    
    let buzzer_pin = peripherals.GPIO33;  // Remove mutability since we're moving ownership
    let ledc = Ledc::new(peripherals.LEDC);
    let song = Song::new(pink_panther::TEMPO);
    
    
    // Initialize LEDC timer once
    let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
    hstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty10Bit,
            clock_source: timer::HSClockSource::APBClk,
            frequency: Rate::from_hz(440), // Default frequency
        })
        .unwrap();
    
    // Initialize LEDC channel once (move buzzer here)
    let mut channel0 = ledc.channel(channel::Number::Channel0, buzzer_pin);
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 0,  // Start with 0% duty (silent)
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();
    
    for (note, duration_type) in pink_panther::MELODY {
        let note_duration = song.calc_note_duration(duration_type) as u64;
        let pause_duration = note_duration / 10; // 10% of note_duration
        
        if note == music::REST {
            channel0.set_duty(0).unwrap();  // Ensure silence
            blocking_delay(Duration::from_millis(note_duration));
            continue;
        }
        
        // Update frequency for this note
        let freq = Rate::from_hz(note as u32);
        let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
        hstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty10Bit,
                clock_source: timer::HSClockSource::APBClk,
                frequency: freq, //notes
            })
            .unwrap();
        
        // Set duty to 50% to play the note
        channel0.set_duty(50).unwrap();
        blocking_delay(Duration::from_millis(note_duration - pause_duration));
        
        // Brief pause between notes
        channel0.set_duty(0).unwrap();
        blocking_delay(Duration::from_millis(pause_duration));
    }
    
    loop {
        blocking_delay(Duration::from_millis(5));
    }
}