//! M2 Bootloader RUST App Peripherals Module
//! -----------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>

#![no_std]

use core::ptr;

const GPIO_PORT_BASE: u32 = 0x5000_0000; // Replace with your MCU GPIO base
const LED_PIN: u32 = 5; // Example: Pin 5

/// Initialize LED GPIO pin as output
pub fn init_led() {
    unsafe {
        // Configure LED_PIN as output
        let moder = (GPIO_PORT_BASE + 0x00) as *mut u32; // MODER register
        let current = ptr::read_volatile(moder);
        // Clear previous mode bits and set as output (01)
        ptr::write_volatile(
            moder,
            (current & !(0b11 << (LED_PIN * 2))) | (0b01 << (LED_PIN * 2)),
        );
    }
}

/// Turn LED on
pub fn led_on() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32; // Output Data Register
        ptr::write_volatile(odr, ptr::read_volatile(odr) | (1 << LED_PIN));
    }
}

/// Turn LED off
pub fn led_off() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32;
        ptr::write_volatile(odr, ptr::read_volatile(odr) & !(1 << LED_PIN));
    }
}

/// Toggle LED state
pub fn toggle_led() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32;
        ptr::write_volatile(odr, ptr::read_volatile(odr) ^ (1 << LED_PIN));
    }
}
