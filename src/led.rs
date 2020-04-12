use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};
use stm32f4xx_hal::gpio::{gpioc::PC1, Output, PushPull};

/// The red led on the feather board
pub struct Led {
    led: PC1<Output<PushPull>>,
}

impl Led {
    pub fn new<M>(pc1: PC1<M>) -> Self {
        let mut led = pc1.into_push_pull_output();
        let _ = led.set_low().ok();
        Self { led }
    }

    pub fn toggle(&mut self) {
        self.led.toggle().ok();
    }

    pub fn set(&mut self, on: bool) {
        if on {
            self.led.set_high().ok();
        } else {
            self.led.set_low().ok();
        }
    }
}
