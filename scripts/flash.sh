#!/bin/bash
# M2 Bootloader RUST
# ------------------
# License : Dual License
#           - Apache 2.0 for open-source / personal use
#           - Commercial license required for closed-source use
# Author  : Md Mahbubur Rahman
# URL     : <https://m-a-h-b-u-b.github.io>
# GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>

probe-rs-cli download target/thumbv7em-none-eabihf/debug/bootloader --chip STM32F411RE
probe-rs-cli download target/thumbv7em-none-eabihf/debug/app --chip STM32F411RE --base-address 0x08004000