#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embedded_hal::delay::DelayNs;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Flex, InputConfig, Level, OutputConfig, Pull};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize Flex pin for DHT11 on GPIO4
    let dht11_pin = Flex::new(peripherals.GPIO4);

    let mut dht = Dht11::new(dht11_pin, Delay::new());

    loop {
        match dht.read() {
            Ok((hum, temp)) => info!("Humidity: {}%, Temp: {}°C", hum, temp),
            Err(e) => info!("DHT11 read error: {:?}", e),
        }
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

/// Error kinds for DHT11 reading
#[derive(Debug)]
pub enum DhtError {
    Timeout,
    ChecksumMismatch,
}

/// Low-level DHT bit-banger using a flexible GPIO and timing
struct DhtCore<'d> {
    pin: Flex<'d>,
}

impl<'d> DhtCore<'d> {
    fn new(pin: Flex<'d>) -> Self {
        DhtCore { pin }
    }

    /// Wait until pin level == target, or timeout
    fn wait_until(&self, target: bool, timeout: Duration) -> Result<(), DhtError> {
        let start = Instant::now();
        while self.pin.is_high() != target {
            if start.elapsed() > timeout {
                return Err(DhtError::Timeout);
            }
        }
        Ok(())
    }

    /// Read one byte by measuring high pulse durations
    fn read_byte(&self) -> Result<u8, DhtError> {
        let mut value = 0u8;
        for bit in (0..8).rev() {
            // 50µs low -> start
            self.wait_until(true, Duration::from_micros(200))?;
            // measure high
            let t0 = Instant::now();
            self.wait_until(false, Duration::from_micros(500))?;
            if t0.elapsed() > Duration::from_micros(100) {
                value |= 1 << bit;
            }
        }
        Ok(value)
    }
}

/// High-level DHT11 driver
pub struct Dht11<'d> {
    core: DhtCore<'d>,
    delay: Delay,
}

impl<'d> Dht11<'d> {
    /// Construct from a Flex pin + Delay
    pub fn new(mut pin: Flex<'d>, delay: Delay) -> Self {
        // Configure pull-up input and default output
        let in_cfg = InputConfig::default().with_pull(Pull::Up);
        pin.apply_input_config(&in_cfg);
        let out_cfg = OutputConfig::default();
        pin.apply_output_config(&out_cfg);

        // Idle state: input enabled (pull-up), output disabled
        pin.set_output_enable(false);
        pin.set_input_enable(true);

        Dht11 {
            core: DhtCore::new(pin),
            delay,
        }
    }

    /// Read humidity (%) and temperature (°C)
    pub fn read(&mut self) -> Result<(u8, u8), DhtError> {
        // 1. Stabilize sensor (>1s)
        self.delay.delay_ms(1_000u32);
        
        // 2. Start signal: drive low 18ms
        self.core.pin.set_input_enable(false);
        self.core.pin.set_level(Level::Low);
        self.core.pin.set_output_enable(true);
        self.delay.delay_ms(18u32);

        // 3. Release: switch to input pull-up
        self.core.pin.set_output_enable(false);
        self.core.pin.set_input_enable(true);

        // 4. Sensor response: low ~80µs, high ~80µs, low ~50µs
        self.core.wait_until(false, Duration::from_micros(1_000))?;
        self.core.wait_until(true,  Duration::from_micros(1_000))?;
        self.core.wait_until(false, Duration::from_micros(1_000))?;

        // 5. Read bytes: humidity int, humidity dec, temp int, temp dec, checksum
        let h_int = self.core.read_byte()?;
        let h_dec = self.core.read_byte()?;
        let t_int = self.core.read_byte()?;
        let t_dec = self.core.read_byte()?;
        let chks = self.core.read_byte()?;

        // 6. Validate checksum
        let sum = h_int.wrapping_add(h_dec)
                      .wrapping_add(t_int)
                      .wrapping_add(t_dec);
        if sum != chks {
            return Err(DhtError::ChecksumMismatch);
        }

        Ok((h_int, t_int))
    }
}
