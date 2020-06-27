//! Functionality for easy setup of the Adafruit feather f405 board
#![no_std]

mod clocks;
mod flash;
mod led;
mod neopixel;
mod sdcard;

pub use clocks::clock_setup;
pub use flash::Flash;
pub use led::Led;
pub use neopixel::NeoPixel;
pub use sdcard::SdHost;

pub use stm32f4xx_hal as hal;
pub use hal::stm32 as pac;
