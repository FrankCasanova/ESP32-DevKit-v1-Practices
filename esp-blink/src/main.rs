// Import necessary modules and types from the standard library and ESP-IDF HAL
use std::thread::sleep; // For adding delays
use std::time::Duration; // For specifying the duration of delays
use std::collections::HashMap; // For Morse code mapping

use esp_idf_svc::sys::link_patches; // For linking runtime patches
use esp_idf_hal::peripherals::Peripherals; // For accessing GPIO peripherals
use esp_idf_hal::gpio::*; // For GPIO operations
use esp_idf_sys::EspError; // For error handling

// Define a struct to encapsulate the buzzer pin
struct Buzzer {
    pin: PinDriver<'static, Gpio12, Output>, // Assuming buzzer is connected to GPIO12
}

// Implement methods for the Buzzer struct to control the buzzer
impl Buzzer {
    // Method to turn the buzzer on
    fn on(&mut self) -> Result<(), EspError> {
        self.pin.set_high() // Turn on buzzer
    }

    // Method to turn the buzzer off
    fn off(&mut self) -> Result<(), EspError> {
        self.pin.set_low() // Turn off buzzer
    }

    // Method to send a dot (.)
    fn dot(&mut self) -> Result<(), EspError> {
        self.on()?;
        sleep(Duration::from_millis(100)); // Dot duration
        self.off()?;
        sleep(Duration::from_millis(100)); // Space between dots/dashes
        Ok(())
    }

    // Method to send a dash (-)
    fn dash(&mut self) -> Result<(), EspError> {
        self.on()?;
        sleep(Duration::from_millis(300)); // Dash duration
        self.off()?;
        sleep(Duration::from_millis(100)); // Space between dots/dashes
        Ok(())
    }

    // Method to send a letter
    fn letter(&mut self, letter: char) -> Result<(), EspError> {
        let morse_code_map: HashMap<char, &str> = [
            ('A', ".-"), ('B', "-..."), ('C', "-.-."), ('D', "-.."), ('E', "."), ('F', "..-."), 
            ('G', "--."), ('H', "...."), ('I', ".."), ('J', ".---"), ('K', "-.-"), ('L', ".-.."), 
            ('M', "--"), ('N', "-."), ('O', "---"), ('P', ".--."), ('Q', "--.-"), ('R', ".-."), 
            ('S', "..."), ('T', "-"), ('U', "..-"), ('V', "...-"), ('W', ".--"), ('X', "-..-"), 
            ('Y', "-.--"), ('Z', "--.."), ('0', "-----"), ('1', ".----"), ('2', "..---"), 
            ('3', "...--"), ('4', "....-"), ('5', "....."), ('6', "-...."), ('7', "--..."), 
            ('8', "---.."), ('9', "----."), (' ', "/"), // Space between words
        ].iter().cloned().collect();

        if let Some(morse_code) = morse_code_map.get(&letter) {
            for symbol in morse_code.chars() {
                match symbol {
                    '.' => self.dot()?,
                    '-' => self.dash()?,
                    '/' => sleep(Duration::from_millis(700)), // Space between words
                    _ => {}
                }
            }
            sleep(Duration::from_millis(300)); // Space between letters
        }
        Ok(())
    }

    // Method to send a message
    fn message(&mut self, message: &str) -> Result<(), EspError> {
        for letter in message.chars() {
            self.letter(letter)?;
        }
        Ok(())
    }
}

fn main() {
    // Ensure runtime patches are linked correctly
    link_patches();

    // Enable logging
    esp_idf_svc::log::EspLogger::initialize_default();

    // Log a message indicating the start of the example
    log::info!("Starting buzzer Morse code example...");

    // Access GPIO peripherals
    let peripherals = Peripherals::take().unwrap();

    // Define buzzer pin using the Buzzer struct
    let mut buzzer = Buzzer {
        pin: PinDriver::output(peripherals.pins.gpio12).unwrap(), // Buzzer connected to GPIO12
    };

    // Main loop
    loop {
        buzzer.message("HELLO IM FRANK CASANOVA AND IM DEVELOPING THIS PROJECT USING RUST AND ESP32!").unwrap(); // Send message in Morse code
    }
}
