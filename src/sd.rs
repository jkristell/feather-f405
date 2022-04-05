use core::ops::{Deref, DerefMut};

use stm32f4xx_hal::{
    pac::SDIO,
    rcc::Clocks,
    sdio::{SdCard, Sdio},
};

use crate::pins::{SdCardDetect, SdClk, SdCmd, SdData};

/// The sd host on the feather board
pub struct SdHost {
    /// Card detect pin
    pub cd: SdCardDetect,
    /// Sdio device
    sdio: Sdio<SdCard>,
}

impl SdHost {
    pub fn new(
        dev: SDIO,
        clk: SdClk,
        cmd: SdCmd,
        (d0, d1, d2, d3): SdData,
        card_detect: SdCardDetect,
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
    type Target = Sdio<SdCard>;

    fn deref(&self) -> &Self::Target {
        &self.sdio
    }
}

impl DerefMut for SdHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sdio
    }
}
