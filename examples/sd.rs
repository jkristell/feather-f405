#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use feather_f405::{
    hal::{delay, prelude::*, sdio::ClockFreq, stm32},
    setup_clocks, SdHost,
};

#[entry]
fn main() -> ! {
    let device = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    rtt_init_print!(BlockIfFull);

    let clocks = setup_clocks(device.RCC);
    assert!(clocks.is_pll48clk_valid());

    let mut delay = delay::Delay::new(core.SYST, clocks);

    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let mut sd = SdHost::new(
        device.SDIO,
        gpioc.pc12,
        gpiod.pd2,
        gpioc.pc8,
        gpioc.pc9,
        gpioc.pc10,
        gpioc.pc11,
        gpiob.pb12,
        clocks,
    );

    rprintln!("Waiting for card...");

    // Wait for card to be ready
    loop {
        match sd.init_card(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(err) => rprintln!("Err: {:?}", err),
        }

        delay.delay_ms(1000u32);
    }

    rprintln!(
        "Card ready\n
         Status: {:?}",
        sd.read_sd_status()
    );

    if let Ok(card) = sd.card() {
        rprintln!("Card Information");
        rprintln!("Card address: {:X}", card.rca.address());
        rprintln!("blocks: {}", card.block_count());
        rprintln!("Product name: {}", card.cid.product_name());
        rprintln!("OEM ID: {}", card.cid.oem_id());

        rprintln!("{:?}", card.ocr);
        rprintln!("{:?}", card.cid);
        rprintln!("{:?}", card.scr);
        rprintln!("{:?}", card.csd);
    }

    rprintln!("Done");

    loop {
        continue;
    }
}
