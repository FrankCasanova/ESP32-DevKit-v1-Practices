use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

/// A "smart LED" that blinks patterns based on log levels
pub struct LogLed {
    pub led: Output<'static>,
}

impl LogLed {
    /// Create a new LogLed from any GPIO pin
    pub fn new(led: Output<'static>) -> Self {
        Self {
            led: led.into(), // Erase pin type
        }
    }

    /// 🔵 Single short blink: routine info (e.g., "still alive")
    pub async fn info(&mut self) {
        self.blink(100, 400).await;
    }

    /// 🟡 Double blink: warning, something needs attention
    pub async fn warn(&mut self) {
        self.blink(150, 100).await;
        Timer::after(Duration::from_millis(100)).await;
        self.blink(150, 400).await;
    }

    /// 🔴 Rapid flash: error! Something went wrong
    pub async fn error(&mut self) {
        for _ in 0..5 {
            self.blink(50, 50).await;
        }
    }

    /// 🟢 Solid on: system is up and stable
    pub async fn success(&mut self) {
        self.led.set_high();
        Timer::after(Duration::from_millis(500)).await;
    }

    /// 🌟 Pulse once: event occurred (e.g., HTTP request)
    pub async fn event(&mut self) {
        self.blink(200, 200).await;
    }

    /// 🔁 Slow blink: waiting for something (e.g., Wi-Fi link)
    pub async fn waiting(&mut self) {
        self.blink(200, 800).await; // 1Hz, lazy blink
    }

    /// 🚀 Fast pulse: booting / initializing
    pub async fn boot(&mut self) {
        self.blink(50, 100).await;
        Timer::after(Duration::from_millis(100)).await;
        self.blink(50, 100).await;
        Timer::after(Duration::from_millis(100)).await;
        self.blink(50, 500).await; // Final pause
    }

    /// 📶 Heartbeat: system is alive, polling
    pub async fn heartbeat(&mut self) {
        self.blink(30, 30).await;
        Timer::after(Duration::from_millis(940)).await;
    }

    /// 🎉 Victory flash: full startup complete!
    pub async fn victory(&mut self) {
        // Three quick flashes
        for _ in 0..3 {
            self.blink(100, 100).await;
        }
        // Then stay on
        self.led.set_high();
    }

    /// 🛑 Off: system stopped or error state
    pub async fn off(&mut self) {
        self.led.set_low();
        Timer::after(Duration::from_millis(100)).await;
    }

    /// Generic blink: on_ms, off_ms
    async fn blink(&mut self, on_ms: u64, off_ms: u64) {
        self.led.set_high();
        Timer::after(Duration::from_millis(on_ms)).await;
        self.led.set_low();
        Timer::after(Duration::from_millis(off_ms)).await;
    }
}
