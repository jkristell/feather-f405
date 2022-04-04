
use crate::hal::{
    gpio::{
        Debugger, Floating, Input, PA15, PA4, PA5, PA6, PA7, PB10, PB11, PB12, PB13, PB14, PB15,
        PB3, PB4, PB5, PB6, PB7, PB8, PB9, PC1, PC10, PC11, PC12, PC2, PC3, PC4, PC5, PC6, PC7,
        PC8, PC9, PD2,
    },
    pac::{GPIOA, GPIOB, GPIOC, GPIOD},
    prelude::*,
};

// Top header

/// Gpio 13. Connected to the red led
pub type G13<M> = PC1<M>;
pub type G12<M> = PC2<M>;
pub type G11<M> = PC3<M>;
pub type G10<M> = PB9<M>;
pub type G9<M> = PB8<M>;
pub type G6<M> = PC6<M>;
pub type G5<M> = PC7<M>;

/// I2C alsoa available on the STEMMA connector
pub type SCL<M> = PB6<M>;
/// I2C alsoa available on the STEMMA connector
pub type SDA<M> = PB7<M>;

// Bottom header

pub type A0<M> = PA4<M>;
pub type A1<M> = PA5<M>;
pub type A2<M> = PA6<M>;
pub type A3<M> = PA7<M>;
pub type A4<M> = PC4<M>;
pub type A5<M> = PC5<M>;

/// SPI Clock (PB13)
pub type SCK<M> = PB13<M>;
pub type MO<M> = PB15<M>;
pub type MI<M> = PB14<M>;

pub type RX<M> = PB11<M>;
pub type TX<M> = PB10<M>;

pub type SdClk<M> = PC12<M>;
pub type SdCmd<M> = PD2<M>;
pub type SdData0<M> = PC8<M>;
pub type SdData1<M> = PC9<M>;
pub type SdData2<M> = PC10<M>;
pub type SdData3<M> = PC11<M>;
pub type SdCd<M> = PB12<M>;

pub type FlashSck<M> = PB3<M>;
pub type FlashMiso<M> = PB4<M>;
pub type FlashMosi<M> = PB5<M>;
pub type FlashCs<M> = PA15<M>;

pub struct Pins {
    pub g13: G13<Input<Floating>>,
    pub g12: G12<Input<Floating>>,
    pub g11: G11<Input<Floating>>,
    pub g10: G10<Input<Floating>>,
    pub g9: G9<Input<Floating>>,
    pub g6: G6<Input<Floating>>,
    pub g5: G5<Input<Floating>>,

    pub scl: SCL<Input<Floating>>,
    pub sda: SDA<Input<Floating>>,

    pub a0: A0<Input<Floating>>,
    pub a1: A1<Input<Floating>>,
    pub a2: A2<Input<Floating>>,
    pub a3: A3<Input<Floating>>,
    pub a4: A4<Input<Floating>>,
    pub a5: A5<Input<Floating>>,

    pub sck: SCK<Input<Floating>>,
    pub mo: MO<Input<Floating>>,
    pub mi: MI<Input<Floating>>,
    pub rx: RX<Input<Floating>>,
    pub tx: TX<Input<Floating>>,

    pub sd_clk: SdClk<Input<Floating>>,
    pub sd_cmd: SdCmd<Input<Floating>>,
    pub sd_data: (
        SdData0<Input<Floating>>,
        SdData1<Input<Floating>>,
        SdData2<Input<Floating>>,
        SdData3<Input<Floating>>,
    ),
    pub sd_cd: SdCd<Input<Floating>>,

    pub flash_sck: FlashSck<Debugger>,
    pub flash_miso: FlashMiso<Debugger>,
    pub flash_mosi: FlashMosi<Input<Floating>>,
    pub flash_cs: FlashCs<Debugger>,
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
    }
}
