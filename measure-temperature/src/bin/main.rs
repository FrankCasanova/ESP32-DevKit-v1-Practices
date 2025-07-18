#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::{info, Format};
use esp_hal::clock::CpuClock;

use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// DHT11 measurement result
#[derive(Debug, Format)]
struct DHT11Reading {
    temperature: i16,
    humidity: i16,
}

// Simulate DHT11 sensor reading
// In a real implementation, this would use bidirectional GPIO communication
// Think of this as a warehouse worker who gives you a report based on their observations
fn simulate_dht11_reading(pin: &mut Output<'_>, measurement_count: u32) -> DHT11Reading {
    // Simulate the start signal to the sensor
    // Like knocking on the warehouse door to get attention
    pin.set_low();
    pin.set_high();

    // Create realistic temperature and humidity variations
    // Think of this as natural fluctuations in warehouse conditions
    let base_temp = 23; // Base temperature of 23°C
    let base_humidity = 55; // Base humidity of 55%

    // Add some variation based on time to make it more realistic
    let temp_variation = ((measurement_count * 3) % 10) as i16 - 5; // ±5°C variation
    let humidity_variation = ((measurement_count * 7) % 20) as i16 - 10; // ±10% variation

    let temperature = base_temp + temp_variation;
    let humidity = base_humidity + humidity_variation;

    DHT11Reading {
        temperature,
        humidity,
    }
}

// Simple delay function using busy waiting
// This is like counting to yourself while waiting
fn simple_delay_ms(ms: u32) {
    let start = Instant::now();
    let duration = Duration::from_millis(ms as u64);
    while start.elapsed() < duration {
        // Busy wait - think of this as pacing back and forth while waiting
    }
}

#[main]
fn main() -> ! {
    // rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Create output pin for DHT11 communication
    // Think of this as setting up a communication line to the warehouse sensor
    let mut dht_pin = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

    info!("🌡️  DHT11 Temperature Measurement System Started!");
    info!("📍 Sensor connected to GPIO2");
    info!("⏰ Reading temperature every 2 seconds...");
    info!("🔌 Wiring guide:");
    info!("   - DHT11 VCC (pin 1) -> 3.3V");
    info!("   - DHT11 DATA (pin 2) -> GPIO2");
    info!("   - DHT11 GND (pin 4) -> GND");
    info!("   - Add 4.7kΩ pull-up resistor between VCC and DATA");
    info!("");
    info!("📝 NOTE: This is a demonstration showing the framework structure.");
    info!("    Temperature readings are simulated to show the concept.");
    info!("    For real DHT11 communication, you'd need bidirectional GPIO control.");
    info!("");

    let mut measurement_count = 0;

    loop {
        measurement_count += 1;

        // Simulate reading from DHT11 sensor
        // This is like our warehouse worker checking the climate conditions
        let measurement = simulate_dht11_reading(&mut dht_pin, measurement_count);

        info!(
            "📊 Measurement #{}: Temperature: {}°C, Humidity: {}%",
            measurement_count, measurement.temperature, measurement.humidity
        );

        // Provide contextual feedback like a smart warehouse climate control system
        // Think of this as automated alerts based on warehouse conditions
        if measurement.temperature > 30 {
            info!("🔥 Hot! Temperature is above 30°C - time to turn on the AC!");
        } else if measurement.temperature < 18 {
            info!("🧊 Cold! Temperature is below 18°C - might need some heating!");
        } else {
            info!("✅ Perfect temperature range - comfortable as a cozy living room!");
        }

        // Additional humidity feedback
        // Humidity control is just as important as temperature in a warehouse
        if measurement.humidity > 70 {
            info!("💧 High humidity detected - might feel muggy!");
        } else if measurement.humidity < 30 {
            info!("🏜️ Low humidity detected - might feel dry!");
        } else {
            info!("💨 Humidity is in comfortable range!");
        }

        // Add some educational insights about the readings
        // Think of this as having a knowledgeable warehouse manager explain the conditions
        match measurement_count % 5 {
            0 => info!("🎓 Fun fact: DHT11 sensors have ±2°C accuracy for temperature!"),
            1 => info!("🎓 Fun fact: DHT11 sensors have ±5% accuracy for humidity!"),
            2 => info!("🎓 Fun fact: DHT11 uses a one-wire protocol for communication!"),
            3 => info!("🎓 Fun fact: DHT11 needs at least 1 second between readings!"),
            4 => info!("🎓 Fun fact: DHT11 operates best at 20-60°C temperature range!"),
            _ => {}
        }

        // Show the Rust ownership concept in action
        // This is like explaining how warehouse management works
        if measurement_count % 10 == 0 {
            info!("🦀 Rust concept: The pin ownership ensures memory safety!");
            info!("   - Only one part of code can control the pin at a time");
            info!("   - This prevents electrical conflicts and memory bugs");
            info!("   - Think of it like having one key holder for warehouse access");
        }

        // Wait for 2 seconds before next measurement
        // DHT11 needs at least 1 second between readings, so 2 seconds is safe
        // Think of this as giving our warehouse worker time to reset for the next check
        simple_delay_ms(2000);
    }
}
