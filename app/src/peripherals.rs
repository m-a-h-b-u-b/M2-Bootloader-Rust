//! M2 Bootloader RUST App Peripherals Module
//! -----------------------------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>

// Build without the Rust standard library—important for embedded targets.
#![no_std] 

// Provides volatile read/write functions for direct memory access.
use core::ptr; 

// Hardware-Specific Constants
const GPIO_PORT_BASE: u32 = 0x5000_0000; // Base address of the MCU's GPIO peripheral.
// NOTE: Replace this address with the actual GPIO base of your target MCU.
const LED_PIN: u32 = 5;                  // LED connected to GPIO pin number 5 (example).

// LED GPIO Initialization
// Initialize LED GPIO pin as output
//
// This function configures the selected LED pin to act as an output pin
// by writing to the GPIO port's MODER (Mode Register).
pub fn init_led() {
    unsafe {
        // Calculate the address of the MODER register.
        let moder = (GPIO_PORT_BASE + 0x00) as *mut u32;

        // Read the current MODER value without compiler optimizations
        // to ensure hardware register consistency.
        let current = ptr::read_volatile(moder);

        // Clear the existing 2-bit mode field for LED_PIN,
        // then set it to `01` (binary) for "General Purpose Output Mode".
        ptr::write_volatile(
            moder,
            (current & !(0b11 << (LED_PIN * 2))) | (0b01 << (LED_PIN * 2)),
        );
    }
}

// -----------------------------------------------------------------------------
// LED Control Functions
// -----------------------------------------------------------------------------

/// Turn LED on
///
/// Sets the LED pin's bit in the Output Data Register (ODR) to logic high.
pub fn led_on() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32; // ODR register offset is 0x14.
        // OR the current ODR value with the LED bit to turn it on.
        ptr::write_volatile(odr, ptr::read_volatile(odr) | (1 << LED_PIN));
    }
}

/// Turn LED off
///
/// Clears the LED pin's bit in the Output Data Register (ODR) to logic low.
pub fn led_off() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32;
        // AND the current ODR value with the inverse of LED bit to turn it off.
        ptr::write_volatile(odr, ptr::read_volatile(odr) & !(1 << LED_PIN));
    }
}

/// Toggle LED state
///
/// Flips the current logic level of the LED pin by XORing the ODR value.
pub fn toggle_led() {
    unsafe {
        let odr = (GPIO_PORT_BASE + 0x14) as *mut u32;
        // XOR toggles the LED bit: if on → off, if off → on.
        ptr::write_volatile(odr, ptr::read_volatile(odr) ^ (1 << LED_PIN));
    }
}
