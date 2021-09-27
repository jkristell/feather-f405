#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::sdio::{self};
use hal::{prelude::*, stm32};

use core::cell::RefCell;
use embedded_sdmmc;
use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx, Controller, TimeSource, Timestamp};
use feather_f405::hal::sdio::ClockFreq;
use feather_f405::SdHost;

struct Sd {
    sdio: RefCell<SdHost>,
}

impl BlockDevice for Sd {
    type Error = sdio::Error;

    fn read(
        &self,
        blocks: &mut [Block],
        start: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        let mut addr = start.0;

        let mut sdio = self.sdio.borrow_mut();

        for b in blocks {
            sdio.read_block(addr, &mut b.contents)?;
            addr += 1;
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start: BlockIdx) -> Result<(), Self::Error> {
        let mut addr = start.0;
        let mut sdio = self.sdio.borrow_mut();
        for b in blocks {
            sdio.write_block(addr, &b.contents)?;
            addr += 1;
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        self.sdio
            .borrow()
            .card()
            .map(|c| BlockCount(c.block_count()))
    }
}

struct DummyTimesource;

impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp::from_fat(0, 0)
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Constrain clock registers
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

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(core.SYST, clocks);

    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let mut red_led = {
        let mut led = gpioc.pc1.into_push_pull_output();
        let _ = led.set_low().ok();
        led
    };

    let mut sdio = feather_f405::SdHost::new(
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

    // Loop until we have a card
    loop {
        match sdio.init_card(ClockFreq::F8Mhz) {
            Ok(_) => break,
            Err(err) => {
                rprintln!("Init err: {:?}", err);
            }
        }

        delay.delay_ms(1000u32);
        red_led.toggle().ok();
    }

    let sdhc = Sd {
        sdio: RefCell::new(sdio),
    };

    let mut fs = Controller::new(sdhc, DummyTimesource);

    rprintln!("OK!\nCard size...");
    let size = fs.device().sdio.borrow().card().map(|c| c.block_count());
    rprintln!("size: {:?}", size);

    rprintln!("Volume 0...");
    match fs.get_volume(embedded_sdmmc::VolumeIdx(0)) {
        Ok(v) => {
            rprintln!("{:?}\n", v);
            let root = fs.open_root_dir(&v).unwrap();

            rprintln!("Root content:");
            fs.iterate_dir(&v, &root, |x| {
                rprintln!("  {:?}", x.name);
            })
            .unwrap();
        }
        Err(e) => {
            rprintln!("Err: {:?}", e);
        }
    }

    loop {
        continue;
    }
}
