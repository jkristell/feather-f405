#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_mass_storage;
use usbd_scsi::{BlockDevice, BlockDeviceError, Scsi};

use feather_f405::{
    clock_setup,
    hal::{
        delay, interrupt,
        otg_fs::{UsbBus, UsbBusType, USB},
        prelude::*,
        sdio::ClockFreq,
    },
    pac,
    Led, SdHost,
};
use core::convert::TryInto;

// Globals
static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static USB_DEV: Mutex<RefCell<Option<UsbDevice<UsbBusType>>>> = Mutex::new(RefCell::new(None));
static USB_STORAGE: Mutex<RefCell<Option<usbd_scsi::Scsi<UsbBusType, Storage>>>> =
    Mutex::new(RefCell::new(None));

struct Storage {
    host: RefCell<SdHost>,
}

impl BlockDevice for Storage {
    const BLOCK_BYTES: usize = 512;

    fn read_block(&self, lba: u32, block: &mut [u8]) -> Result<(), BlockDeviceError> {
        let sdio = &mut self.host.borrow_mut().sdio;

        let block: &mut [u8; 512] = block.try_into().map_err(|_e| BlockDeviceError::InvalidAddress)?;

        sdio.read_block(lba, block).map_err(|e| {
            rprintln!("read error: {:?}", e);
            BlockDeviceError::HardwareError
        })
    }

    fn write_block(&mut self, lba: u32, block: &[u8]) -> Result<(), BlockDeviceError> {
        let sdio = &mut self.host.borrow_mut().sdio;

        let block: &[u8; 512] = block.try_into()
            .map_err(|_e| BlockDeviceError::InvalidAddress)?;

        sdio.write_block(lba, block).map_err(|e| {
            rprintln!("write error: {:?}", e);
            BlockDeviceError::WriteError
        })
    }

    fn max_lba(&self) -> u32 {
        let sdio = &self.host.borrow().sdio;

        sdio.card().map(|c| c.block_count() - 1).unwrap_or(0)
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let dp = pac::Peripherals::take().unwrap();
    let p = cortex_m::Peripherals::take().unwrap();

    let clocks = clock_setup(dp.RCC);

    // Create a delay abstraction based on SysTick
    let mut delay = delay::Delay::new(p.SYST, clocks);

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let mut led = Led::new(gpioc.pc1);

    let mut sd = SdHost::new(
        dp.SDIO, gpioc.pc12, gpiod.pd2, gpioc.pc8, gpioc.pc9, gpioc.pc10, gpioc.pc11, gpiob.pb12, clocks
    );

    rprintln!("Init done");

    // Loop until we have a card
    loop {
        match sd.sdio.init_card(ClockFreq::F24Mhz) {
            Ok(_) => break,
            Err(err) => {
                rprintln!("Init err: {:?}", err);
            }
        }

        rprintln!("Waiting for card...");

        delay.delay_ms(1000u32);
        led.toggle();
    }

    rprintln!(
        "Card with blocks: {:?} detected. Initiating usb...",
        sd.sdio.card().map(|c| c.block_count())
    );

    let sdhc = Storage {
        host: RefCell::new(sd),
    };

    unsafe {
        let usb = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
            hclk: clocks.hclk(),
        };

        let usb_bus = UsbBus::new(usb, &mut EP_MEMORY);
        USB_BUS = Some(usb_bus);

        let scsi = Scsi::new(
            USB_BUS.as_ref().unwrap(),
            64,
            sdhc,
            "Fake Co.",
            "Feather",
            "F405",
        );

        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("Fake company")
            .product("SdUsb")
            .serial_number("TEST")
            .self_powered(true)
            .device_class(usbd_mass_storage::USB_CLASS_MSC)
            .build();

        cortex_m::interrupt::free(|cs| {
            USB_DEV.borrow(cs).replace(Some(usb_dev));
            USB_STORAGE.borrow(cs).replace(Some(scsi));
        });
    };

    unsafe {
        NVIC::unmask(pac::Interrupt::OTG_FS);
    }

    rprintln!("Init done");

    loop {
        continue;
    }
}

#[interrupt]
fn OTG_FS() {
    cortex_m::interrupt::free(|cs| {
        let mut dev = USB_DEV.borrow(cs).borrow_mut();
        let usb_dev = dev.as_mut().unwrap();

        let mut scsi = USB_STORAGE.borrow(cs).borrow_mut();
        let scsi = scsi.as_mut().unwrap();

        if !usb_dev.poll(&mut [scsi]) {
            return;
        }
    });
}
