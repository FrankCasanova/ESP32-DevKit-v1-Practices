#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::cell::RefCell;
use esp_alloc::string::{String, ToString};
use core::str::FromStr;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// Morse code mapping
const MORSE_CODE: &[(&str, &str)] = &[
    ("H", "...."), ("I", ".."), ("F", "..-."),
    ("R", ".-."), ("A", ".-"), ("N", "-."),
    ("K", "-.-"), ("C", "-.-."), ("S", "..."),
    ("O", "---"), ("V", "...-"), ("A", ".-"),
    (" ", "/"), (",", "--..--"), (".", ".-.-.-")
];

// Convert text to Morse code
// Convert text to Morse code
fn text_to_morse(text: &str) -> String {
    text.chars()
        .map(|c| {
            MORSE_CODE.iter()
                .find(|&&(letter, _)| letter == c.to_string())
                .map_or(String::new(), |&(_, code)| code.to_string())
        })
        .collect::<Vec<String>>()
        .join(" ")
}
// Add this function to play Morse code
fn play_morse_code(morse: &str) {
    let dot_duration = Duration::from_millis(200);
    let dash_duration = Duration::from_millis(600);
    let symbol_gap = Duration::from_millis(200);
    let word_gap = Duration::from_millis(700);
    
    for c in morse.chars() {
        match c {
            '.' => { buzzer.set_high(); blocking_delay(dot_duration); buzzer.set_low(); },
            '-' => { buzzer.set_high(); blocking_delay(dash_duration); buzzer.set_low(); },
            ' ' => blocking_delay(symbol_gap),
            '/' => blocking_delay(word_gap),
            _ => {}
        }
        blocking_delay(symbol_gap);
    }
}


// Define a simple blocking delay function
fn blocking_delay(duration: Duration) {
    let start = esp_hal::time::Instant::now();
    while start.elapsed() < duration {}
}

#[main]
fn main() -> ! {
    // generator version: 0.4.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);
    
    let button = Input::new(peripherals.GPIO22, InputConfig::default().with_pull(Pull::Up));
    let mut buzzer = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());
    
    let mut last_press = false;
    let mut message = RefCell::new(text_to_morse("HI IM FRANK CASANOVA, I LOVE YOU SO MUCH"));
    
    loop {
        let is_pressed = button.is_low();
        
        // Debounce logic
        if is_pressed && !last_press {
            // play the full message
            play_morse_code(&message.borrow());
            
            // Wait for button release
            while button.is_low() {
                blocking_delay(Duration::from_millis(50));
            }
        }
        
        last_press = is_pressed;
        blocking_delay(Duration::from_millis(10));
    }


    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
