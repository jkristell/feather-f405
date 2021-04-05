use stm32f4xx_hal::{
    gpio::{
        gpiob::PB12,
        gpioc::{PC10, PC11, PC12, PC8, PC9},
        gpiod::PD2,
        Input, PullUp,
    },
    sdio::Sdio,
    stm32::SDIO,
    rcc::Clocks
};

/// The sd host on the feather board
pub struct SdHost {
    /// Card detect pin
    pub cd: PB12<Input<PullUp>>,
    /// Sdio device
    pub sdio: Sdio,
}

impl SdHost {
    pub fn new<M0, M1, M2, M3, M4, M5, M6>(
        dev: SDIO,
        clk: PC12<M0>,
        cmd: PD2<M1>,
        d0: PC8<M2>,
        d1: PC9<M3>,
        d2: PC10<M4>,
        d3: PC11<M5>,
        card_detect: PB12<M6>,
        clocks: Clocks,
    ) -> Self {
        let clk = clk.into_alternate_af12().internal_pull_up(false);
        let cmd = cmd.into_alternate_af12().internal_pull_up(true);
        let d0 = d0.into_alternate_af12().internal_pull_up(true);
        let d1 = d1.into_alternate_af12().internal_pull_up(true);
        let d2 = d2.into_alternate_af12().internal_pull_up(true);
        let d3 = d3.into_alternate_af12().internal_pull_up(true);

        // Card detect pin
        let cd = card_detect.into_pull_up_input();

        let sdio = Sdio::new(dev, (clk, cmd, d0, d1, d2, d3), clocks);
        SdHost { sdio, cd }
    }
}
