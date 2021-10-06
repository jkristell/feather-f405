[![crates.io version](https://img.shields.io/crates/v/feather-f405.svg)](https://crates.io/crates/feather-f405)
[![docs.rs](https://docs.rs/feather-f405/badge.svg)](https://docs.rs/feather-f405)

# Feather-f405

Board support package for the Adafruit feather f405 with some abstractions
and helpers for the onboard peripherals.

Support and examples for the
 - onboard led
 - Neopixel led
 - SD-card reader
 - Spi flash

## Flashing

### with dfu-utils

```
dfu-util -a 0 --dfuse-address 0x08000000 -D firmware.bin
```

Or look at dfu-flash.sh for a complete example


### With cargo embed
You will have to solder leads to the programming pads on the back of then board. Then you can use
a programmer like a ST-link v2 and flash and debug the board with Probe or Openocd.

```
cargo embed --release example neopixel
```


## Resources

https://learn.adafruit.com/adafruit-stm32f405-feather-express/dfu-bootloader-details

