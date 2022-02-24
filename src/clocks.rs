use stm32f4xx_hal::pac;
use stm32f4xx_hal::{
    prelude::*,
    rcc::{Clocks, RccExt},
};

/// Helper for setting up the clocks on the board
pub fn setup_clocks(rcc: pac::RCC) -> Clocks {
    let rcc = rcc.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(12.MHz())
        .require_pll48clk()
        .sysclk(168.MHz())
        .hclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    assert!(clocks.is_pll48clk_valid());

    clocks
}
