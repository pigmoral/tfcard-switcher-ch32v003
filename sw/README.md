# Software

build:
```sh
cargo build --release
```

flash:
> To flash, you need to connect the SWIO pin on the board to the SWIO pin on WCH-LinkE.
> Note that WCH-Link (without E) cannot flash the CH32V003 chip.
```sh
wlink flash target/riscv32ec-unknown-none-elf/release/firmware
```
