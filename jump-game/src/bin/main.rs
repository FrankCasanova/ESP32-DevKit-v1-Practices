#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::i2c;
use esp_hal::i2c::master::I2c;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

use jump_game::game::{Game, GameState};
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    // Setting up I2C send text to OLED display
    // let i2c = i2c::I2c::new_async(peripherals.I2C1, scl, sda, Irqs, i2c::Config::default());
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(90))
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    // Setup the OLED Display
    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display.flush().unwrap();

    let button = Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up));

    let random_gen = RandomGen::new(esp_hal::rng::Rng::new(peripherals.RNG));
    let mut game = Game::new(random_gen, display);
    game.draw_trex().unwrap();
    let mut clicked_count = 0; // To restart the game when it two times button clicked

    info!("Starting Game!");

    loop {
        if game.state == GameState::GameOver {
            if button.is_low() {
                clicked_count += 1;
            }
            if clicked_count > 2 {
                clicked_count = 0;
                game = Game::new(game.obstacles.rng, game.display);
                Timer::after_millis(500).await;
            }
            Timer::after_millis(50).await;
            continue;
        }

        game.clear_screen().unwrap();
        game.draw_score().unwrap();

        if button.is_low() {
            game.trex_jump();
        }

        game.move_world().unwrap();
        game.draw_ground().unwrap();
        game.draw_trex().unwrap();

        if game.check_collison() {
            game.game_over().unwrap();
            game.display.flush().unwrap();
            Timer::after_millis(500).await;
            continue;
        }

        game.display.flush().unwrap();
        Timer::after_millis(5).await;
    }
}

// Helper struct for Random number generation
struct RandomGen {
    rng: esp_hal::rng::Rng,
}

impl RandomGen {
    fn new(rng: esp_hal::rng::Rng) -> Self {
        Self { rng }
    }
}

impl jump_game::rng::Rng for RandomGen {
    fn random_u32(&mut self) -> u32 {
        self.rng.random()
    }
}

// for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
