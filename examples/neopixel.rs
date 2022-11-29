#![no_std]
#![no_main]

use cortex_m_rt::entry;
use feather_f405::{hal::prelude::*, pac, setup_clocks, NeoPixel};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use smart_leds::{SmartLedsWrite, RGB8};

///
/// NOTE: Neopixels require very fast and fairly precise timing therefore
/// this example should be compiled in release mode in order for it to work.
///
#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let clocks = setup_clocks(dp.RCC);

    let mut delay = p.SYST.delay(&clocks);
    let gpioc = dp.GPIOC.split();

    let mut timer = dp.TIM2.counter_hz(&clocks);
    timer.start(3.MHz()).unwrap();

    let mut neopixel = NeoPixel::new(gpioc.pc0, timer);

    let mut data = RGB8 { r: 127, g: 30, b: 20 };

    neopixel.write([data].iter().cloned()).unwrap();

    loop {
        (data.r, data.g, data.b) = (data.g, data.b, data.r);
        neopixel.write([data].iter().cloned()).unwrap();

        delay.delay_ms(500u16);
    }
}
