
cargo build --release && cargo objcopy -- -O binary target/thumbv7em-none-eabihf/release/hsd firmware.bin && dfu-util -a 0 --dfuse-address 0x08000000 -D firmware.bin
