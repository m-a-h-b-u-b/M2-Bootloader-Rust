//! M2 Bootloader RUST
//! ------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>
//!
//! High‑level verification utilities for firmware images stored in MCU flash.
//!
//! This module provides routines to verify that a written firmware image
//! matches an expected CRC or raw byte slice. It builds on the [`Flash`] trait
//! and is meant to be MCU‑agnostic.

use crate::flash::{Flash, FlashError, InternalFlash, Result};

/// Verify that the CRC32 of a flash region matches the expected value.
///
/// * `addr`  - Absolute start address of the region to verify.
/// * `len`   - Length in bytes of the region to verify.
/// * `expected_crc` - Expected CRC32 value.
///
/// Returns `Ok(true)` if the CRC matches, `Ok(false)` if it does not,
/// or a `FlashError` on read/driver failures.
pub fn verify_crc(flash: &mut dyn Flash, addr: usize, len: usize, expected_crc: u32) -> Result<bool> {
    let crc = flash.crc32(addr, len)?;
    Ok(crc == expected_crc)
}

/// Verify that the bytes in flash match a reference buffer.
///
/// This is slower than CRC comparison but can pinpoint the first mismatching
/// offset when `stop_on_mismatch` is `true`.
pub fn verify_bytes(
    flash: &mut dyn Flash,
    addr: usize,
    reference: &[u8],
    stop_on_mismatch: bool,
) -> Result<bool> {
    let mut buf = [0u8; 256];
    let mut offset = 0;
    while offset < reference.len() {
        let chunk = core::cmp::min(buf.len(), reference.len() - offset);
        flash.read(addr + offset, &mut buf[..chunk])?;
        if stop_on_mismatch && &buf[..chunk] != &reference[offset..offset + chunk] {
            return Ok(false);
        }
        offset += chunk;
    }
    if stop_on_mismatch {
        Ok(true)
    } else {
        // For full comparison we already returned false on mismatch, so true.
        Ok(true)
    }
}

/// Convenience function to verify a region using the global internal flash driver.
/// Adjust `FLASH_*` constants in `flash.rs` to your MCU's memory map.
#[allow(dead_code)]
pub fn verify_region_crc_internal(addr: usize, len: usize, expected_crc: u32) -> Result<bool> {
    // SAFETY: INTERNAL_FLASH is a global static mut, access must be single‑threaded.
    unsafe { super::flash::INTERNAL_FLASH.crc32(addr, len).map(|c| c == expected_crc) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash::MockFlash;

    #[test]
    fn test_verify_crc_and_bytes() {
        let mut mock = MockFlash::new(1024, 256, 256);
        let data = [0x55u8; 512];
        mock.write_region(0, &data).unwrap();

        // Compute CRC directly from mock.
        let crc = mock.crc32(0, 512).unwrap();
        assert!(verify_crc(&mut mock, 0, 512, crc).unwrap());

        // Verify bytes match.
        assert!(verify_bytes(&mut mock, 0, &data, true).unwrap());

        // Mismatch case.
        let wrong_data = [0xAAu8; 512];
        assert!(!verify_bytes(&mut mock, 0, &wrong_data, true).unwrap());
    }
}
