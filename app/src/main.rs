//! M2 Bootloader RUST 
//! ------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-RUST>


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