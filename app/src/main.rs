//! M2 Bootloader RUST APP Module
//! ------------------------------
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

/// Entry point for the bootloader
#[entry]
fn main() -> ! {
    // Initialize system peripherals (LED, timers, etc.)
    peripherals::init_led();

    // Optional: indicate bootloader start
    peripherals::led_on();

    loop {
        // Toggle LED with a delay for visible blinking
        peripherals::toggle_led();
        delay();
    }
}

/// Simple busy-wait delay
#[inline(always)]
fn delay() {
    // Adjust the count depending on the target MCU clock speed
    for _ in 0..5_000_000 {
        asm::nop();
    }
}
