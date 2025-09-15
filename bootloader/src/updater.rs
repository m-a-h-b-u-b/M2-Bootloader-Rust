//! M2 Bootloader RUST
//! ------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>

//! Firmware update handling module.
//!
//! This module manages the process of receiving, validating,
//! and writing a new firmware image to flash memory. It builds
//! upon the flash abstraction (`flash.rs`) and verification
//! routines (`verify.rs`).

use crate::flash::{Flash, FlashError, Result};
use crate::verify::{verify_crc};

/// Metadata describing the incoming firmware update.
#[derive(Debug, Clone, Copy)]
pub struct UpdateMetadata {
    /// Absolute start address in flash where the new image will be written.
    pub target_addr: usize,
    /// Total size of the firmware image in bytes.
    pub image_size: usize,
    /// Expected CRC32 checksum of the entire image.
    pub expected_crc: u32,
}

/// Possible errors during the update process.
#[derive(Debug)]
pub enum UpdateError {
    Flash(FlashError),
    InvalidSize,
    CrcMismatch,
    TransferIncomplete,
    Other(&'static str),
}

impl From<FlashError> for UpdateError {
    fn from(e: FlashError) -> Self {
        UpdateError::Flash(e)
    }
}

pub type UpdateResult<T> = core::result::Result<T, UpdateError>;

/// Handles the reception and flashing of a new firmware image.
///
/// Typical workflow:
/// 1. Call [`begin_update`] with metadata to erase target sectors.
/// 2. Call [`write_chunk`] repeatedly to program image data.
/// 3. Call [`finalize_update`] to verify CRC and finalize.
pub struct FirmwareUpdater<'a> {
    flash: &'a mut dyn Flash,
    meta: UpdateMetadata,
    written: usize,
}

impl<'a> FirmwareUpdater<'a> {
    /// Prepare for a new firmware update by erasing the target region.
    pub fn begin_update(flash: &'a mut dyn Flash, meta: UpdateMetadata) -> UpdateResult<Self> {
        if meta.image_size == 0 {
            return Err(UpdateError::InvalidSize);
        }
        // Erase all sectors covering the target region.
        let mut addr = meta.target_addr;
        while addr < meta.target_addr + meta.image_size {
            flash.erase_sector(addr)?;
            addr += flash.sector_size();
        }
        Ok(FirmwareUpdater { flash, meta, written: 0 })
    }

    /// Write a contiguous chunk of firmware data.
    /// The caller must supply chunks aligned to the flash page size.
    pub fn write_chunk(&mut self, offset: usize, data: &[u8]) -> UpdateResult<()> {
        if offset != self.written {
            return Err(UpdateError::Other("Offset mismatch"));
        }
        let abs_addr = self.meta.target_addr + offset;
        self.flash.write_region(abs_addr, data)?;
        self.written += data.len();
        Ok(())
    }

    /// Verify the written firmware image against the expected CRC.
    pub fn finalize_update(mut self) -> UpdateResult<()> {
        if self.written != self.meta.image_size {
            return Err(UpdateError::TransferIncomplete);
        }
        let ok = verify_crc(self.flash, self.meta.target_addr, self.meta.image_size, self.meta.expected_crc)
            .map_err(|e| UpdateError::Flash(e))?;
        if !ok {
            return Err(UpdateError::CrcMismatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash::MockFlash;

    #[test]
    fn test_firmware_update_flow() {
        let mut mock = MockFlash::new(4096, 1024, 256);
        let data = [0x42u8; 1024];
        let crc = mock.crc32(0, data.len()).unwrap(); // computing CRC of empty flash (not used)
        // Instead compute CRC of our data.
        let mut tmp = MockFlash::new(2048, 1024, 256);
        tmp.write_region(0, &data).unwrap();
        let expected_crc = tmp.crc32(0, data.len()).unwrap();

        let meta = UpdateMetadata {
            target_addr: 0,
            image_size: data.len(),
            expected_crc,
        };

        let mut updater = FirmwareUpdater::begin_update(&mut mock, meta).unwrap();
        updater.write_chunk(0, &data).unwrap();
        updater.finalize_update().unwrap();
    }
}