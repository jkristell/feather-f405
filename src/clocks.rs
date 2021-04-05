use stm32f4xx_hal::rcc::{Clocks, RccExt};
use stm32f4xx_hal::stm32;
use stm32f4xx_hal::time::U32Ext;

/// Helper for setting up the clocks on the board
pub fn setup_clocks(rcc: stm32::RCC) -> Clocks {
    // Constrain clock registers
    let rcc = rcc.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(12.mhz())
        .require_pll48clk()
        .sysclk(168.mhz())
        .hclk(168.mhz())
        .pclk1(42.mhz())
        .pclk2(84.mhz())
        .freeze();

    assert!(clocks.is_pll48clk_valid());

    clocks
}
