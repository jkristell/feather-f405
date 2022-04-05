#![no_std]
#![no_main]

use cortex_m_rt::entry;
use feather_f405::{
    hal::{pac, prelude::*, sdio::ClockFreq},
    pins::pins,
    setup_clocks, SdHost,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    rtt_init_print!(BlockIfFull);

    let clocks = setup_clocks(device.RCC);
    assert!(clocks.is_pll48clk_valid());

    let mut delay = core.SYST.delay(&clocks);

    let pins = pins(device.GPIOA, device.GPIOB, device.GPIOC, device.GPIOD);

    let mut sd = {
        SdHost::new(
            device.SDIO,
            pins.sd_clk,
            pins.sd_cmd,
            pins.sd_data,
            pins.sd_cd,
            clocks,
        )
    };

    rprintln!("Waiting for card...");

    // Wait for card to be ready
    loop {
        match sd.init(ClockFreq::F24Mhz) {
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
