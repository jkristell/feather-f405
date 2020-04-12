# feather-f405

## Flashing

### with dfu-utils

```
dfu-util -a 0 --dfuse-address 0x08000000 -D firmware.bin
```

Or look at dfu-flash.sh for a complete example


### With cargo embed
You will have to solder leds to the programming pads on the back of then board. Then you can use
a programmer and flash and debug the board with probe and openocd.

```
cargo embed --release example neopixel
```


## Resources

https://learn.adafruit.com/adafruit-stm32f405-feather-express/dfu-bootloader-details

## Leds
- PC1 Red led
- PC0 ws2812b

## SD Pins
- PB12 - SD_CARD_DETECT
- PD2  - SDIO_CMD
- PC8  - SDIO_D0
- PC9  - SDIO_D1
- PC10 - SDIO_D2
- PC11 - SDIO D3
- PC12 - SDIO_CLK

## SPI flash
Uses the Spi1 peripheral and the following pins
- PA15 - CS
- PB3 - SCK
- PB4 - MISO
- PB5 - MOSI

