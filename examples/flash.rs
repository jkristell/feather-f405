#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use feather_f405::{
    clock_setup,
    hal::{prelude::*, stm32},
    Flash,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let _p = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // Setup the clocks
    let clocks = clock_setup(dp.RCC);

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    // Construct the flash
    let mut flash =
        Flash::new(gpiob.pb3, gpiob.pb4, gpiob.pb5, gpioa.pa15, dp.SPI1, clocks).unwrap();

    let id = flash.read_id();

    rprintln!("Flash id: {:?}", id);

    loop {
        continue;
    }
}
