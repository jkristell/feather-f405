use stm32f4xx_hal::gpio::gpioc::PC0;
use stm32f4xx_hal::gpio::{Output, PushPull, Speed};
use ws2812_timer_delay::Ws2812;

use embedded_hal::timer::{CountDown, Periodic};

/// NeoPixel
pub struct NeoPixel<Timer> {
    pub ws: Ws2812<Timer, PC0<Output<PushPull>>>,
}

impl<Timer: CountDown + Periodic> NeoPixel<Timer> {
    // Create an abstraction for the onboard neopixel pin
    pub fn new<M>(pin: PC0<M>, timer: Timer) -> Self {
        let pin = pin.into_push_pull_output().set_speed(Speed::High);
        let ws = Ws2812::new(timer, pin);
        NeoPixel { ws }
    }
}
