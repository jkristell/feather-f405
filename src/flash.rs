use stm32f4xx_hal::{
    gpio::{
        gpioa::PA15,
        gpiob::{PB3, PB4, PB5},
        Alternate, Output, PushPull,
    },
    pac::SPI1,
    prelude::*,
    rcc::Clocks,
    spi::{Spi, TransferModeNormal},
};

use embedded_hal::spi::MODE_0;
use spi_memory::{self, series25};

type FlashSpi = Spi<
    SPI1,
    (
        PB3<Alternate<PushPull, 5>>,
        PB4<Alternate<PushPull, 5>>,
        PB5<Alternate<PushPull, 5>>,
    ),
    TransferModeNormal,
>;
type FlashCs = PA15<Output<PushPull>>;

/// Spi flash
pub struct Flash {
    flash: series25::Flash<FlashSpi, FlashCs>,
}

impl Flash {
    pub fn new<M0, M1, M2, M3>(
        sck: PB3<M0>,
        miso: PB4<M1>,
        mosi: PB5<M2>,
        cs: PA15<M3>,
        spi1: SPI1,
        clocks: Clocks,
    ) -> Result<Flash, spi_memory::Error<FlashSpi, FlashCs>> {
        // Setup the Spi device
        let spi = {
            let sck = sck.into_alternate();
            let miso = miso.into_alternate();
            let mosi = mosi.into_alternate();

            Spi::new(spi1, (sck, miso, mosi), MODE_0, 1.MHz(), &clocks)
        };

        // Setup the chip select pin
        let cs = {
            let mut cs = cs.into_push_pull_output();
            let _ = cs.set_high();
            cs
        };

        // Construct the flash struct
        let flash = series25::Flash::init(spi, cs)?;

        Ok(Flash { flash })
    }

    pub fn read_id(&mut self) -> series25::Identification {
        self.flash.read_jedec_id().unwrap()
    }
}
