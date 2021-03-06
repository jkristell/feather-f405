use stm32f4xx_hal::{
    gpio::{
        gpioa::PA15,
        gpiob::{PB3, PB4, PB5},
        Alternate, Output, PushPull, AF5,
    },
    rcc::Clocks,
    spi::Spi,
    stm32::SPI1,
    time::MegaHertz,
};

use embedded_hal::{digital::v2::OutputPin, spi::MODE_0};
use spi_memory::{self, series25};

type FlashSpi = Spi<
    SPI1,
    (
        PB3<Alternate<AF5>>,
        PB4<Alternate<AF5>>,
        PB5<Alternate<AF5>>,
    ),
>;
type FlashCs = PA15<Output<PushPull>>;

/// Spi flash
pub struct Flash {
    flash: series25::Flash<FlashSpi, FlashCs>,
}

impl Flash {
    pub fn new<M0, M1, M2, M3>(
        pb3: PB3<M0>,
        pb4: PB4<M1>,
        pb5: PB5<M2>,
        cs: PA15<M3>,
        spi1: SPI1,
        clocks: Clocks,
    ) -> Result<Flash, spi_memory::Error<FlashSpi, FlashCs>> {
        // Setup the Spi device
        let spi = {
            let sck = pb3.into_alternate_af5();
            let miso = pb4.into_alternate_af5();
            let mosi = pb5.into_alternate_af5();

            Spi::spi1(spi1, (sck, miso, mosi), MODE_0, MegaHertz(1).into(), clocks)
        };

        // Setup the chip select pin
        let cs = {
            let mut cs = cs.into_push_pull_output();
            let _ = cs.set_high().ok();
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
