//! M2 Bootloader RUST Blink Module
//! -------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>

#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;

mod peripherals;

#[entry]
fn main() -> ! {
    // Initialize LED
    peripherals::init_led();

    loop {
        // Toggle LED state
        peripherals::toggle_led();

        // Simple busy-wait delay for visible blink
        delay();
    }
}

/// Busy-wait delay function
#[inline(always)]
fn delay() {
    // Adjust this value according to your MCU clock speed
    for _ in 0..5_000_000 {
        asm::nop();
    }
}
