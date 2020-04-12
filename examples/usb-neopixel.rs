#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU8, Ordering};
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use panic_halt as _;

use hal::{prelude::*, stm32, };

use hal::{interrupt};

use stm32f4xx_hal::gpio::Speed;
use cortex_m::peripheral::NVIC;

use stm32f4xx_hal::otg_fs::{UsbBus, USB, UsbBusType};
use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;

use smart_leds::{RGB8, SmartLedsWrite};
use ws2812_timer_delay::Ws2812;
use hal::timer::Timer;


static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;
static mut USB_SERIAL: Option<usbd_serial::SerialPort<UsbBusType>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBusType>> = None;

// Led colors
static CR: AtomicU8 = AtomicU8::new(0);
static CG: AtomicU8 = AtomicU8::new(0);
static CB: AtomicU8 = AtomicU8::new(0);


#[entry]
fn main() -> ! {

    let device = stm32::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Enable the clock for the SYSCFG
    device.RCC.apb2enr .modify(|_, w| w.syscfgen().enabled() );

    // Constrain clock registers
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr
        .use_hse(12.mhz())
        .require_pll48clk()
        .sysclk(168.mhz())
        .hclk(168.mhz())
        .pclk1(42.mhz())
        .pclk2(84.mhz())
        .freeze();


    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(core.SYST, clocks);

    let gpioa = device.GPIOA.split();
    let gpioc = device.GPIOC.split();

    unsafe {
        let usb = USB {
            usb_global: device.OTG_FS_GLOBAL,
            usb_device: device.OTG_FS_DEVICE,
            usb_pwrclk: device.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate_af10(),
            pin_dp: gpioa.pa12.into_alternate_af10(),
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
        USB_SERIAL = Some(serial);
    };

    let neopixel_pin = gpioc.pc0.into_push_pull_output().set_speed(Speed::High);
    let timer = Timer::tim2(device.TIM2, 3.mhz(), clocks);
    let mut ws = Ws2812::new(timer, neopixel_pin);

    unsafe {
        NVIC::unmask(stm32::Interrupt::OTG_FS);
    }

    loop {

        let rgb = RGB8 {
            r: CR.load(Ordering::SeqCst),
            g: CG.load(Ordering::SeqCst),
            b: CB.load(Ordering::SeqCst),
        };

        ws.write([rgb].iter().cloned()).unwrap();

        delay.delay_ms(100u16);
    }
}

#[interrupt]
fn OTG_FS() {
    usb_interrupt();
}


fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    let mut buf = [0u8; 8];

    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {

            if let Some(c) = buf.get(0) {

                match *c {
                    b'r' => {
                        CR.fetch_add(10, Ordering::SeqCst);
                    },
                    b'g' => {
                        CG.fetch_add(10, Ordering::SeqCst);
                    }
                    b'b' => {
                        CB.fetch_add(10, Ordering::SeqCst);
                    }
                    b'c' => {
                        CR.store(0, Ordering::SeqCst);
                        CG.store(0, Ordering::SeqCst);
                        CB.store(0, Ordering::SeqCst);
                    }
                    _ => ()
                }
            }

            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}




/*
/* Init flash */

let spi = {
    let sck = gpiob.pb3.into_alternate_af5();
    let miso = gpiob.pb4.into_alternate_af5();
    let mosi = gpiob.pb5.into_alternate_af5();

    Spi::spi1(
        device.SPI1,
        (sck, miso, mosi),
        MODE_0,
        MegaHertz(1).into(),
        clocks,
    )
};

let cs = {
    let mut cs = gpioa.pa15.into_push_pull_output();
    let _ = cs.set_low().ok();
    cs
};

let flash = SpiFlash::new(spi, cs);


/* Flash init */
*/

