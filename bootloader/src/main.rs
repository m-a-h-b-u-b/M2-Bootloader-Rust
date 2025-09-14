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