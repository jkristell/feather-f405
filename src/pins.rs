use stm32f4xx_hal::gpio::{AF0, PC0};

use crate::hal::{
    gpio::{
        PA15, PA4, PA5, PA6, PA7, PB10, PB11, PB12, PB13, PB14, PB15, PB3, PB4, PB5, PB6, PB7, PB8,
        PB9, PC1, PC10, PC11, PC12, PC2, PC3, PC4, PC5, PC6, PC7, PC8, PC9, PD2,
    },
    pac::{GPIOA, GPIOB, GPIOC, GPIOD},
    prelude::*,
};

// Top header

/// Gpio 13. Connected to the red led
pub type G13 = PC1;
pub type Led = G13;
pub type G12 = PC2;
pub type G11 = PC3;
pub type G10 = PB9;
pub type G9 = PB8;
pub type G6 = PC6;
pub type G5 = PC7;

/// I2C alsoa available on the STEMMA connector
pub type SCL = PB6;
/// I2C alsoa available on the STEMMA connector
pub type SDA = PB7;

// Bottom header

pub type A0 = PA4;
pub type A1 = PA5;
pub type A2 = PA6;
pub type A3 = PA7;
pub type A4 = PC4;
pub type A5 = PC5;

/// SPI Clock (PB13)
pub type SCK = PB13;
pub type MO = PB15;
pub type MI = PB14;

pub type RX = PB11;
pub type TX = PB10;

pub type SdClk = PC12;
pub type SdCmd = PD2;
pub type SdData0 = PC8;
pub type SdData1 = PC9;
pub type SdData2 = PC10;
pub type SdData3 = PC11;
pub type SdCardDetect = PB12;
pub type SdData = (SdData0, SdData1, SdData2, SdData3);

pub type FlashSck = PB3<AF0>;
pub type FlashMiso = PB4<AF0>;
pub type FlashMosi = PB5;
pub type FlashCs = PA15<AF0>;

pub type NeoPixel = PC0;

pub struct Pins {
    pub g13: G13,
    pub g12: G12,
    pub g11: G11,
    pub g10: G10,
    pub g9: G9,
    pub g6: G6,
    pub g5: G5,

    pub scl: SCL,
    pub sda: SDA,

    pub a0: A0,
    pub a1: A1,
    pub a2: A2,
    pub a3: A3,
    pub a4: A4,
    pub a5: A5,

    pub sck: SCK,
    pub mo: MO,
    pub mi: MI,
    pub rx: RX,
    pub tx: TX,

    pub sd_clk: SdClk,
    pub sd_cmd: SdCmd,
    pub sd_data: (SdData0, SdData1, SdData2, SdData3),
    pub sd_cd: SdCardDetect,

    pub flash_sck: FlashSck,
    pub flash_miso: FlashMiso,
    pub flash_mosi: FlashMosi,
    pub flash_cs: FlashCs,

    pub neopixel: NeoPixel,
}

/// Get all pins available on the board
pub fn pins(gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, gpiod: GPIOD) -> Pins {
    let porta = gpioa.split();
    let portb = gpiob.split();
    let portc = gpioc.split();
    let portd = gpiod.split();

    Pins {
        a0: porta.pa4,
        a1: porta.pa5,
        a2: porta.pa6,
        a3: porta.pa7,
        a4: portc.pc4,
        a5: portc.pc5,
        sck: portb.pb13,
        mo: portb.pb15,
        mi: portb.pb14,
        rx: portb.pb11,
        tx: portb.pb10,
        sd_clk: portc.pc12,
        sd_cmd: portd.pd2,
        sd_data: ((portc.pc8), (portc.pc9), (portc.pc10), (portc.pc11)),
        g13: portc.pc1,
        g12: portc.pc2,
        g11: portc.pc3,
        g10: portb.pb9,
        g9: portb.pb8,
        g6: portc.pc6,
        g5: portc.pc7,

        scl: portb.pb6,
        sda: portb.pb7,
        sd_cd: portb.pb12,
        flash_sck: portb.pb3,
        flash_miso: portb.pb4,
        flash_mosi: portb.pb5,
        flash_cs: porta.pa15,
        neopixel: portc.pc0,
    }
}
