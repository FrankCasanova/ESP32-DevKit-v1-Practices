// Import necessary modules and types from the standard library and ESP-IDF HAL
use std::thread::sleep; // For adding delays
use std::time::Duration; // For specifying the duration of delays

use esp_idf_svc::sys::link_patches; // For linking runtime patches
use esp_idf_hal::peripherals::Peripherals; // For accessing GPIO peripherals
use esp_idf_hal::gpio::*; // For GPIO operations
use esp_idf_sys::EspError; // For error handling

// Define an enum to encapsulate different LED pin types
enum LedPin {
    RedLed(PinDriver<'static, Gpio23, Output>), // LED connected to GPIO23
    GreenLed(PinDriver<'static, Gpio19, Output>), // LED connected to GPIO19
    YellowLed(PinDriver<'static, Gpio18, Output>), // LED connected to GPIO18
}

// Implement methods for the LedPin enum to control the LEDs
impl LedPin {
    // Method to set the LED high (turn on)
    fn set_high(&mut self) -> Result<(), EspError> {
        match self {
            LedPin::RedLed(pin) => pin.set_high(), // Turn on LED connected to GPIO23
            LedPin::GreenLed(pin) => pin.set_high(), // Turn on LED connected to GPIO19
            LedPin::YellowLed(pin) => pin.set_high(), // Turn on LED connected to GPIO18
        }
    }

    // Method to set the LED low (turn off)
    fn set_low(&mut self) -> Result<(), EspError> {
        match self {
            LedPin::RedLed(pin) => pin.set_low(), // Turn off LED connected to GPIO23
            LedPin::GreenLed(pin) => pin.set_low(), // Turn off LED connected to GPIO19
            LedPin::YellowLed(pin) => pin.set_low(), // Turn off LED connected to GPIO18
        }
    }
}

fn main() {
    // Ensure runtime patches are linked correctly
    link_patches();

    // Enable logging
    esp_idf_svc::log::EspLogger::initialize_default();

    // Log a message indicating the start of the example
    log::info!("Starting multi-LED control example...");

    // Access GPIO peripherals
    let peripherals = Peripherals::take().unwrap();

    // Define LED pins using the LedPin enum
    let led_pins = [
        LedPin::RedLed(PinDriver::output(peripherals.pins.gpio23).unwrap()), // LED connected to GPIO23
        LedPin::GreenLed(PinDriver::output(peripherals.pins.gpio19).unwrap()), // LED connected to GPIO19
        LedPin::YellowLed(PinDriver::output(peripherals.pins.gpio18).unwrap()), // LED connected to GPIO18
    ];

    // Create LED drivers as a vector of LedPin enum instances
    let mut leds: Vec<LedPin> = led_pins.into_iter().collect();

    // Set up button on GPIO 12 as input with pull-up resistor
    let mut button = PinDriver::input(peripherals.pins.gpio12).unwrap();
    button.set_pull(Pull::Up).unwrap(); // Enable internal pull-up resistor

    // Initial LED index
    let mut current_led_index = 0;

    // Debounce time (milliseconds)
    let debounce_time = 50;

    // Track previous button state for edge detection
    let mut previous_button_state = true;

    // Main loop
    loop {
        // Read current button state
        let current_button_state = button.is_low();

        // Debounce the button to avoid multiple triggers from a single press
        if current_button_state != previous_button_state {
            sleep(Duration::from_millis(debounce_time)); // Wait for debounce time
            let current_button_state = button.is_low(); // Re-read the button state after debounce

            // Check if button was just released (falling edge)
            if !current_button_state && previous_button_state {
                log::info!("Button released, switching LED...");

                // Turn off all LEDs
                for led in &mut leds {
                    led.set_low().unwrap(); // Turn off each LED
                }

                // Turn on the next LED
                leds[current_led_index].set_high().unwrap(); // Turn on the current LED

                // Increment LED index (wrap around)
                current_led_index = (current_led_index + 1) % leds.len();

                log::info!("LED {} ON", current_led_index); // Log which LED is on
            }

            // Update previous button state
            previous_button_state = current_button_state;
        }

        // Short delay to avoid busy-waiting
        sleep(Duration::from_millis(10));
    }
}
