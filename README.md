# M2 Bootloader Rust

[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)

## Overview

This repository provides a **Rust-based bootloader** and an example **IoT application** for microcontrollers (STM32F4 series as a reference).  
The bootloader is designed to enable:

- **Secure firmware updates** (UART, USB, or OTA-ready)
- **Firmware integrity verification** using CRC or SHA256
- **Fallback mechanism** for invalid firmware
- Minimal dependency on external runtime (`#![no_std]`)

This project serves as a **starting template** for embedded developers looking to build robust IoT bootloaders in Rust.

---

## Features

- Bare-metal Rust bootloader (`#![no_std]`)
- Hardware initialization module (`init.rs`)
- Flash read/write routines (`flash.rs`)
- Firmware verification (`verify.rs`)
- Update handling (`updater.rs`)
- Example IoT application (`app/`)
- Cross-platform scripts for flashing and verification
- Ready-to-use GitHub Actions CI for building bootloader and app

---

## Repository Structure

```
iot-bootloader-rust/
│
├─ Cargo.toml                   # Workspace manifest
├─ rust-toolchain.toml          # Rust version pinning
├─ README.md
├─ .gitignore
├─ .github/workflows/build.yml  # CI/CD workflow
│
├─ bootloader/                  # Bootloader crate
│   ├─ Cargo.toml
│   └─ src/
│       ├─ main.rs
│       ├─ init.rs
│       ├─ flash.rs
│       ├─ updater.rs
│       └─ verify.rs
│
├─ app/                         # IoT Application crate
│   ├─ Cargo.toml
│   └─ src/
│       ├─ main.rs
│       └─ peripherals.rs
│
├─ scripts/                     # Flashing and verification scripts
│   ├─ flash.sh
│   └─ check_firmware.sh
│
├─ examples/                    # Example applications for testing
│   └─ blinky.rs
│
└─ docs/
    └─ memory_map.md            # MCU flash layout and memory map
```

---

## Getting Started

### Prerequisites

- **Rust toolchain** (stable recommended)
- `cargo` and `rustup`
- Embedded development tools:
  - `probe-rs` CLI for flashing
  - STM32 USB/UART programmer
- Optional: VSCode + Rust Analyzer for development

### Build Bootloader

```bash
cd bootloader
cargo build --release --target thumbv7em-none-eabihf
```

### Build Application

```bash
cd app
cargo build --release --target thumbv7em-none-eabihf
```

### Flash Firmware

```bash
./scripts/flash.sh
```

### Verify Firmware

```bash
./scripts/check_firmware.sh
```

---

## Bootloader Workflow

```
+----------------+
| MCU Reset       |
+----------------+
        |
        v
+----------------+
| Bootloader      |
| init hardware   |
+----------------+
        |
        v
+----------------+
| Verify firmware |
+----------------+
        |
        v
+-------------------------+
| Check for update        |
| (UART/USB/OTA)         |
+-------------------------+
        |
        v
+-------------------------+
| Apply update if present |
+-------------------------+
        |
        v
+-------------------------+
| Jump to application     |
+-------------------------+
        |
        v
+-------------------------+
| Fallback if failure     |
+-------------------------+
```

---

## Memory Layout

```
Bootloader: 0x08000000 - 0x08003FFF
Application: 0x08004000 - 0x080FFFFF
Backup: 0x08100000 - 0x08103FFF
```

---

## Example Snippets

### Bootloader Main

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;

mod init;
mod flash;
mod updater;
mod verify;

#[entry]
fn main() -> ! {
    init::init_hardware();

    if verify::firmware_valid() {
        updater::check_for_update();
        jump_to_app();
    } else {
        loop {}
    }
}

fn jump_to_app() -> ! {
    const APP_START_ADDRESS: u32 = 0x0800_4000;
    let app: extern "C" fn() = unsafe { core::mem::transmute(APP_START_ADDRESS) };
    app();
}
```

### Application Main

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
mod peripherals;

#[entry]
fn main() -> ! {
    peripherals::init_led();

    loop {
        peripherals::toggle_led();
    }
}
```

### Peripheral Example

```rust
pub fn init_led() {}
pub fn toggle_led() {}
```

---

## Future Development

- Add **OTA over Wi-Fi or BLE**
- Secure boot with **digital signature verification**
- Dual-bank firmware for **rollback support**
- Support additional MCUs (ESP32, nRF52)
- Add **unit tests and CI/CD for embedded targets**

## License

![Apache 2.0 License](https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square)  
![Dual License](https://img.shields.io/badge/License-Dual%20License-green?style=flat-square) 

This project is **dual-licensed**:

- **Open-Source / Personal Use:** Apache 2.0  
- **Commercial / Closed-Source Use:** Proprietary license required 

For commercial licensing inquiries or enterprise use, please contact: [mahbub.aaman.app@gmail.com](mailto:mahbub.aaman.app@gmail.com)


## Author

**Md Mahbubur Rahman**
[GitHub](https://github.com/m-a-h-b-u-b) | [Website](https://m-a-h-b-u-b.github.io)

---

## Contributing

We welcome contributions!

* Fork the repo and submit pull requests
* Follow Rust coding guidelines and safety best practices
* Report issues or suggest features via GitHub Issues

---

# Further Reading / References

1. [Developing a Cryptographically Secure Bootloader for RISC-V in Rust](https://www.codethink.co.uk/articles/2024/secure_bootloader/)  
2. [Low-Power Secure Booting for IoT Edge Devices](https://www.researchgate.net/publication/393623129_Low-Power_Secure_Booting_for_IoT_Edge_Devices)  
3. [Secure Boot and Root-of-Trust in Heterogeneous SoCs](https://www.researchgate.net/publication/395172368_A_Comprehensive_Analysis_of_Secure_Boot_and_Root-of-Trust_Implementation_in_Heterogeneous_SoCs_with_a_RISC-V_Secure_Enclave)  
4. [Design and Implementation of a Secure Bootloader Using Public Key Cryptography](https://www.researchgate.net/publication/390574601_Design_and_Implementation_of_a_Secure_Bootloader_Using_Public_Key_Cryptography_Research)  
5. [Rust for Embedded Systems: Current State and Open Problems](https://arxiv.org/pdf/2311.05063)  
6. [Rust for Security and Correctness in the Embedded World](https://www.nccgroup.com/us/research-blog/rust-for-security-and-correctness-in-the-embedded-world/)  
7. [Building Safe and Secure Software with Rust on Arm](https://semiengineering.com/building-safe-and-secure-software-with-rust-on-arm/)  
8. [Building a Secure Operating System (Redox OS) with Rust](https://changelog.com/podcast/280)  
9. [Rust vs C: Language Choices in Embedded Systems and Cryptography](https://www.wolfssl.com/rust-vs-c-navigating-language-choices-in-embedded-systems-and-cryptography/)  
10. [Twine: An Embedded Trusted Runtime for WebAssembly](https://arxiv.org/abs/2103.15860)  
11. [Ferrocene: Rust Toolchain for Safety-Critical Applications](https://ferrocene.dev/)  
12. [Embedded Rust Book](https://docs.rust-embedded.org/book/)  
13. [Rust Embedded Working Group](https://github.com/rust-embedded/wg)  
14. [Rust Embedded HAL](https://github.com/rust-embedded/embedded-hal)  
15. [probe-rs: Debug Probe Library for Rust](https://github.com/probe-rs/probe-rs)  