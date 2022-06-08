use stm32f4xx_hal::gpio::{gpioc::PC1, Output, PushPull};

/// The red led on the feather board
pub struct Led {
    led: PC1<Output<PushPull>>,
}

impl Led {
    pub fn new(led: crate::pins::Led) -> Self {
        let mut led = led.into_push_pull_output();
        let _ = led.set_low();
        Self { led }
    }

    pub fn toggle(&mut self) {
        self.led.toggle();
    }

    pub fn set(&mut self, on: bool) {
        if on {
            self.led.set_high();
        } else {
            self.led.set_low();
        }
    }
}
