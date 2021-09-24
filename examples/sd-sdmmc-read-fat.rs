#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::interrupt;
use hal::sdio::{self};
use hal::{prelude::*, stm32};

use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::otg_fs::{UsbBus, UsbBusType, USB};
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_sdmmc;
use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx, Controller, TimeSource, Timestamp};
use feather_f405::hal::sdio::ClockFreq;
use feather_f405::SdHost;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
//static mut USB_SERIAL: Option<usbd_serial::SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

static UCMD: AtomicU32 = AtomicU32::new(0);

static USERIAL: Mutex<RefCell<Option<usbd_serial::SerialPort<UsbBusType>>>> =
    Mutex::new(RefCell::new(None));

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

    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    unsafe {
        let usb = USB {
            usb_global: device.OTG_FS_GLOBAL,
            usb_device: device.OTG_FS_DEVICE,
            usb_pwrclk: device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
            hclk: clocks.hclk(),
        };

        let usb_bus = UsbBus::new(usb, &mut EP_MEMORY);
        USB_BUS = Some(usb_bus);

        let serial = usbd_serial::SerialPort::new(USB_BUS.as_ref().unwrap());

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        USB_DEVICE = Some(usb_dev);
        //USB_SERIAL = Some(serial);

        cortex_m::interrupt::free(|cs| {
            USERIAL.borrow(cs).replace(Some(serial));
        });
    };

    unsafe {
        NVIC::unmask(stm32::Interrupt::OTG_FS);
    }

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

    //rprintln!("Card: {:?}", sdio.card());

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

#[interrupt]
fn OTG_FS() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    //let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    cortex_m::interrupt::free(|cs| {
        let mut serial = USERIAL.borrow(cs).borrow_mut();
        let serial = serial.as_mut().unwrap();

        if !usb_dev.poll(&mut [serial]) {
            return;
        }

        let mut recv = [0u8; 64];
        serial.read(&mut recv);

        match recv[0] {
            b'l' => UCMD.store(b'l' as u32, Ordering::Relaxed),
            _ => (),
        }
    });
}
