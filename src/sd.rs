use core::ops::{Deref, DerefMut};
use stm32f4xx_hal::{
    gpio::{
        gpiob::PB12,
        gpioc::{PC10, PC11, PC12, PC8, PC9},
        gpiod::PD2,
        Input, PullUp,
    },
    rcc::Clocks,
    sdio::Sdio,
    pac::SDIO,
};

/// The sd host on the feather board
pub struct SdHost {
    /// Card detect pin
    pub cd: PB12<Input<PullUp>>,
    /// Sdio device
    sdio: Sdio,
}

impl SdHost {
    pub fn new<M0, M1, M2, M3, M4, M5, M6>(
        dev: SDIO,
        clk: PC12<M0>,
        cmd: PD2<M1>,
        (d0, d1, d2, d3): (PC8<M2>, PC9<M3>, PC10<M4>, PC11<M5>),
        card_detect: PB12<M6>,
        clocks: Clocks,
    ) -> Self {
        let clk = clk.into_alternate().internal_pull_up(false);
        let cmd = cmd.into_alternate().internal_pull_up(true);
        let d0 = d0.into_alternate().internal_pull_up(true);
        let d1 = d1.into_alternate().internal_pull_up(true);
        let d2 = d2.into_alternate().internal_pull_up(true);
        let d3 = d3.into_alternate().internal_pull_up(true);

        // Card detect pin
        let cd = card_detect.into_pull_up_input();

        let sdio = Sdio::new(dev, (clk, cmd, d0, d1, d2, d3), &clocks);
        SdHost { sdio, cd }
    }
}

impl Deref for SdHost {
    type Target = Sdio;

    fn deref(&self) -> &Self::Target {
        &self.sdio
    }
}

impl DerefMut for SdHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sdio
    }
}
