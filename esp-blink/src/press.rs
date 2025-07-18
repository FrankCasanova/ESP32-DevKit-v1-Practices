use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::sys::link_patches;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::*;

fn main() {
    // Ensure runtime patches are linked correctly
    link_patches();

    // Enable logging
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting button-controlled LED example...");

    // Access GPIO peripherals
    let peripherals = Peripherals::take().unwrap();

    // Set up LED on GPIO 23 as output
    let mut led = PinDriver::output(peripherals.pins.gpio23).unwrap();

    // Set up button on GPIO 12 as input with pull-up resistor
    let mut button = PinDriver::input(peripherals.pins.gpio12).unwrap();
    button.set_pull(Pull::Up); // Enable internal pull-up

    // Initial LED state
    let mut led_state = false;

    // Debounce time (milliseconds)
    let debounce_time = 10;

    // Track previous button state for edge detection
    let mut previous_button_state = true; // Initialize to high, as button is normally high

    loop {
        // Read current button state
        let mut current_button_state = button.is_low();

        // Debounce the button
        if current_button_state != previous_button_state {
            sleep(Duration::from_millis(debounce_time));
            current_button_state = button.is_low(); // Re-read after debounce
        }

        // Check if button was just released (falling edge)
        if !current_button_state && previous_button_state {
            log::info!("Button released, toggling LED...");

            // Toggle LED state
            led_state = !led_state;

            // Set LED state
            if led_state {
                led.set_high().unwrap();
                log::info!("LED ON");
            } else {
                led.set_low().unwrap();
                log::info!("LED OFF");
            }
        }

        // Update previous button state
        previous_button_state = current_button_state;

        // Short delay
        sleep(Duration::from_millis(10));
    }
}
