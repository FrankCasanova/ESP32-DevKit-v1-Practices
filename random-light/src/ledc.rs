use esp_hal::gpio::OutputPin;
use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{channel, timer, LSGlobalClkSource, Ledc, LowSpeed};
use esp_hal::time::Rate;

pub fn setup_ledc_timer(
    ledc_peripheral: esp_hal::peripherals::LEDC,
) -> (Ledc, timer::Timer<LowSpeed>) {
    let mut ledc = Ledc::new(ledc_peripheral);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();

    (ledc, lstimer0)
}

pub fn setup_ledc_channel<'a, P: OutputPin + 'a>(
    ledc: Ledc<'a>,
    timer: &'a timer::Timer<'a, LowSpeed>,
    led_pin: P,
) -> channel::Channel<'a, LowSpeed> {
    let mut channel0 = ledc.channel(channel::Number::Channel0, led_pin);
    channel0
        .configure(channel::config::Config {
            timer,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    channel0
}
