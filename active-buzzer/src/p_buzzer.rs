use esp_hal::gpio::{Level, Output};
use esp_hal::time::{Duration, Instant}; 

// Timing utilities
pub fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
} // Busy-wait delay function using time measurement, like a stopwatch

pub struct Buzzer<'a> {
    pin: Output<'a>,
}

impl<'a> Buzzer<'a> {
    pub fn new(pin: Output<'a>) -> Self {
        Buzzer { pin }
    }

    pub fn set_high(&mut self) {
        self.pin.set_level(Level::High);
    }

    pub fn set_low(&mut self) {
        self.pin.set_level(Level::Low);
    }
    
    pub fn connected_sound(&mut self) {
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        blocking_delay(Duration::from_millis(50));
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        blocking_delay(Duration::from_millis(50));
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        blocking_delay(Duration::from_millis(50));
        self.set_high();
        blocking_delay(Duration::from_millis(50));
        self.set_low();
        blocking_delay(Duration::from_millis(5000));
    }
    pub fn monitoring_sound(&mut self) {
        self.set_high();
        blocking_delay(Duration::from_millis(100));
        self.set_low();
        blocking_delay(Duration::from_millis(6000));
        
    }
}
