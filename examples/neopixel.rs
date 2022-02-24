#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use feather_f405::{hal::prelude::*, pac, setup_clocks, NeoPixel};
use smart_leds::{SmartLedsWrite, RGB8};

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

    let mut data = RGB8 { r: 0, g: 0, b: 0 };

    loop {
        data.r = data.r.wrapping_add(4);
        data.g = data.g.wrapping_add(8);
        data.b = data.b.wrapping_add(16);

        neopixel.write([data].iter().cloned()).unwrap();

        delay.delay_ms(50u16);
    }
}
