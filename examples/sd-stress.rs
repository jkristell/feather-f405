#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{
    delay,
    prelude::*,
    sdio::{ClockFreq, Sdio},
    stm32,
};

#[entry]
fn main() -> ! {
    let device = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    rtt_init_print!(BlockIfFull);

    let rcc = device.RCC.constrain();
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

    let mut delay = delay::Delay::new(core.SYST, clocks);

    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let d0 = gpioc.pc8.into_alternate_af12().internal_pull_up(true);
    let d1 = gpioc.pc9.into_alternate_af12().internal_pull_up(true);
    let d2 = gpioc.pc10.into_alternate_af12().internal_pull_up(true);
    let d3 = gpioc.pc11.into_alternate_af12().internal_pull_up(true);
    let clk = gpioc.pc12.into_alternate_af12().internal_pull_up(false);
    let cmd = gpiod.pd2.into_alternate_af12().internal_pull_up(true);
    let mut sdio = Sdio::new(device.SDIO, (clk, cmd, d0, d1, d2, d3), clocks );

    rprintln!("Waiting for card...");

    // Wait for card to be ready
    loop {
        match sdio.init_card(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(_err) => (),
        }

        delay.delay_ms(1000u32);
    }

    if let Ok(card) = sdio.card() {
        rprintln!("Card detected");
        rprintln!("Card address: {:X}", card.rca.address());
        rprintln!("blocks: {}", card.block_count());

        rprintln!("Card name: {}", card.cid.product_name());

        rprintln!("{}", card.cid.oem_id());

        rprintln!("ocr: {:?}", card.ocr);
        rprintln!("cid: {:?}", card.cid);
        //rprintln!("status: {:?}", card.);
        rprintln!("scr: {:?}", card.scr);
        rprintln!("scd: {:?}", card.csd);
        rprintln!("rca: {:?}", card.rca.address());

        for i in 0 .. 32 {
            // Read a block from the card and print the data
            let mut block = [(i & 0xFF) as u8; 512];

            rprintln!("block: {}", i);


        /*
            match sdio.write_block(i, &mut block) {
                Ok(()) => (),
                Err(err) => {
                    rprintln!("Failed to write block {}: {:?}", i, err);
                }
            }
        */

            match sdio.read_block(i, &mut block) {
                Ok(()) => (),
                Err(err) => {
                    rprintln!("Failed to read block: {}: {:?}", i, err);
                }
            }

        }

    }



    rprintln!("Done");

    /*
    for b in block.iter() {
        rprint!("{:X} ", b);
    }

     */

    loop {
        continue;
    }
}
