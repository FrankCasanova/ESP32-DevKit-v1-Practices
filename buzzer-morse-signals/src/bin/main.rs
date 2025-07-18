#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

// Bring in the esp‑alloc crate’s global allocator:
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use core::cell::RefCell;
use core::alloc::Layout;

use esp_hal::{
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    time::{Duration, Instant},
};

// Pull in the crate‑provided allocator and heap‑region types:
use esp_alloc::{HEAP, HeapRegion, MemoryCapability};

// Linker symbols for the default heap region:
extern "C" {
    static mut _heap_start: u32;
    static mut _heap_end:   u32;
}

// Register the memory region once at startup:
unsafe fn init_heap() {
    let start = &raw mut _heap_start as *mut u32 as *mut u8;
    let size  = (&raw mut _heap_end as *mut u32 as usize)
              .wrapping_sub(start as usize);
    HEAP.add_region(HeapRegion::new(start, size, MemoryCapability::Internal.into()));
}

// Handles allocation failures:
#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    loop {}
}

// Handles panics:
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// ESP‑IDF bootloader descriptor
esp_bootloader_esp_idf::esp_app_desc!();

const MORSE_CODE: &[(char, &str)] = &[
    // … your table …
];

fn text_to_morse(text: &str) -> String {
    text.chars()
        .map(|c| {
            let uc = c.to_ascii_uppercase();
            MORSE_CODE
                .iter()
                .find(|&&(ltr, _)| ltr == uc)
                .map_or(String::new(), |&(_, code)| String::from(code))
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn play_morse_code(buzzer: &mut Output, morse: &str) {
    let dot    = Duration::from_millis(200);
    let dash   = dot * 3;
    let intra  = dot;
    let inter  = dot * 3;
    let word   = dot * 7;

    for ch in morse.chars() {
        match ch {
            '.' => { buzzer.set_high(); blocking_delay(dot); buzzer.set_low(); }
            '-' => { buzzer.set_high(); blocking_delay(dash); buzzer.set_low(); }
            ' ' => blocking_delay(inter),
            '/' => blocking_delay(word),
            _   => {}
        }
        blocking_delay(intra);
    }
}

fn blocking_delay(dur: Duration) {
    let start = Instant::now();
    while start.elapsed() < dur {}
}

#[main]
fn main() -> ! {
    // Set up the heap region before any allocations:
    unsafe { init_heap() };

    // Standard ESP‑HAL init:
    let config      = esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz);
    let peripherals = esp_hal::init(config);

    // GPIOs for button & buzzer:
    let button = Input::new(peripherals.GPIO22, InputConfig::default().with_pull(Pull::Up));
    let mut buzzer = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());

    // Prepare the (heap‑allocated) Morse message:
    let mut last_press = false;
    let message = RefCell::new(text_to_morse("HI IM FRANK CASANOVA, I LOVE YOU SO MUCH"));

    loop {
        let pressed = button.is_low();
        if pressed && !last_press {
            play_morse_code(&mut buzzer, &message.borrow());
            // Debounce / wait for release
            while button.is_low() {
                blocking_delay(Duration::from_millis(50));
            }
        }
        last_press = pressed;
        blocking_delay(Duration::from_millis(10));
    }
}
