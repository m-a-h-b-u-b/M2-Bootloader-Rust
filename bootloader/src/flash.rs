//! M2 Bootloader RUST 
//! ------------------
//! License : Dual License
//!           - Apache 2.0 for open-source / personal use
//!           - Commercial license required for closed-source use
//! Author  : Md Mahbubur Rahman
//! URL     : <https://m-a-h-b-u-b.github.io>
//! GitHub  : <https://github.com/m-a-h-b-u-b/M2-Bootloader-Rust>
//!
//! bootloader/src/flash.rs
//! Flash abstraction and utility functions for the bootloader.
//!
//! This module provides:
//! - A generic `Flash` trait that the rest of the bootloader uses.
//! - A `MockFlash` in-memory implementation useful for testing and host-side
//!   unit-tests.
//! - An `InternalFlash` skeleton that can be completed with MCU-specific
//!   register sequences. The skeleton exposes safe high-level helpers such as
//!   `read_flash` and `write_flash` that the rest of the bootloader can call.
//!
//! Features and notes:
//! - The module is testable on host using the `MockFlash` type.
//! - For embedded/no_std targets you will want to enable a `alloc`/stack-buffer
//!   strategy and implement the `InternalFlash` sequences for your MCU.

#![allow(dead_code)]

use core::fmt;

/// Default page size used by mock devices and as a hint for internal drivers.
pub const DEFAULT_PAGE_SIZE: usize = 256;

/// Errors returned by flash operations.
#[derive(Debug, PartialEq, Eq)]
pub enum FlashError {
    OutOfBounds,
    AlignmentError,
    DeviceError(&'static str),
    VerificationFailed { addr: usize, expected: u8, found: u8 },
}

impl fmt::Display for FlashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashError::OutOfBounds => write!(f, "flash: address out of bounds"),
            FlashError::AlignmentError => write!(f, "flash: alignment error"),
            FlashError::DeviceError(s) => write!(f, "flash device error: {}", s),
            FlashError::VerificationFailed { addr, expected, found } => write!(
                f,
                "flash verify failed at {:#010x}: expected=0x{:02x} found=0x{:02x}",
                addr, expected, found
            ),
        }
    }
}

impl std::error::Error for FlashError {}

pub type Result<T> = core::result::Result<T, FlashError>;

/// Trait for flash devices used by the bootloader. Keep implementation minimal
/// to allow both on-chip flash and external SPI/NOR devices to implement it.
pub trait Flash {
    fn size(&self) -> usize;
    fn sector_size(&self) -> usize;
    fn page_size(&self) -> usize;
    fn read(&self, addr: usize, buf: &mut [u8]) -> Result<()>;
    fn erase_sector(&mut self, addr: usize) -> Result<()>;
    fn program_page(&mut self, addr: usize, data: &[u8]) -> Result<()>;

    /// Default verify implementation (reads and compares).
    fn verify(&self, addr: usize, data: &[u8]) -> Result<()> {
        // Using Vec here for host tests; embedded builds should override or
        // provide a temporary buffer strategy (stack buffer or preallocated).
        #[cfg(feature = "std")]
        let mut buf = vec![0u8; data.len()];
        #[cfg(not(feature = "std"))]
        let mut buf = alloc::vec::Vec::from_raw_parts(core::ptr::null_mut(), 0, 0); // placeholder

        #[cfg(feature = "std")]
        {
            self.read(addr, &mut buf)?;
            for (i, (&a, &b)) in data.iter().zip(buf.iter()).enumerate() {
                if a != b {
                    return Err(FlashError::VerificationFailed {
                        addr: addr + i,
                        expected: a,
                        found: b,
                    });
                }
            }
            Ok(())
        }
        #[cfg(not(feature = "std"))]
        {
            // For no_std builds users must implement their own verify or enable
            // an allocator and a temporary buffer. We'll return DeviceError to
            // indicate the lack of an implementation here.
            Err(FlashError::DeviceError("verify not implemented for no_std; enable alloc or override verify"))
        }
    }

    /// Write a region: erase affected sectors and program page-by-page.
    fn write_region(&mut self, addr: usize, data: &[u8]) -> Result<()> {
        if addr.checked_add(data.len()).is_none() || addr + data.len() > self.size() {
            return Err(FlashError::OutOfBounds);
        }

        let sector = self.sector_size();
        if sector == 0 {
            return Err(FlashError::DeviceError("invalid sector size"));
        }

        let start_sector = addr / sector;
        let end_sector = (addr + data.len() - 1) / sector;
        for s in start_sector..=end_sector {
            let sector_addr = s * sector;
            self.erase_sector(sector_addr)?;
        }

        let page = self.page_size();
        if page == 0 {
            return Err(FlashError::DeviceError("invalid page size"));
        }

        let mut offset = 0usize;
        while offset < data.len() {
            let write_addr = addr + offset;
            let remain = data.len() - offset;
            let write_len = core::cmp::min(remain, page);
            let page_slice = &data[offset..offset + write_len];

            self.program_page(write_addr, page_slice)?;
            self.verify(write_addr, page_slice)?;

            offset += write_len;
        }

        Ok(())
    }

    /// Compute CRC32 of a region. Default uses crc32fast when std is enabled.
    fn crc32(&self, addr: usize, len: usize) -> Result<u32> {
        if addr.checked_add(len).is_none() || addr + len > self.size() {
            return Err(FlashError::OutOfBounds);
        }

        #[cfg(feature = "std")]
        {
            let mut buf = vec![0u8; len];
            self.read(addr, &mut buf)?;
            Ok(crc32fast::hash(&buf))
        }
        #[cfg(not(feature = "std"))]
        {
            Err(FlashError::DeviceError("crc32 requires std or custom implementation"))
        }
    }
}

// -----------------------------------------------------------------------------
// MockFlash - in-memory implementation
// -----------------------------------------------------------------------------

pub struct MockFlash {
    pub storage: Vec<u8>,
    sector_size: usize,
    page_size: usize,
}

impl MockFlash {
    pub fn new(size: usize, sector_size: usize, page_size: usize) -> Self {
        MockFlash {
            storage: vec![0xFFu8; size],
            sector_size,
            page_size,
        }
    }

    pub fn fill(&mut self, v: u8) {
        for b in self.storage.iter_mut() {
            *b = v;
        }
    }
}

impl Flash for MockFlash {
    fn size(&self) -> usize {
        self.storage.len()
    }

    fn sector_size(&self) -> usize {
        self.sector_size
    }

    fn page_size(&self) -> usize {
        self.page_size
    }

    fn read(&self, addr: usize, buf: &mut [u8]) -> Result<()> {
        let end = addr.checked_add(buf.len()).ok_or(FlashError::OutOfBounds)?;
        if end > self.storage.len() {
            return Err(FlashError::OutOfBounds);
        }
        buf.copy_from_slice(&self.storage[addr..end]);
        Ok(())
    }

    fn erase_sector(&mut self, addr: usize) -> Result<()> {
        if addr >= self.storage.len() { return Err(FlashError::OutOfBounds); }
        if addr % self.sector_size != 0 { return Err(FlashError::AlignmentError); }
        let end = addr + self.sector_size;
        if end > self.storage.len() { return Err(FlashError::OutOfBounds); }
        for b in &mut self.storage[addr..end] { *b = 0xFF; }
        Ok(())
    }

    fn program_page(&mut self, addr: usize, data: &[u8]) -> Result<()> {
        if addr >= self.storage.len() { return Err(FlashError::OutOfBounds); }
        if data.len() > self.page_size { return Err(FlashError::AlignmentError); }
        let end = addr + data.len();
        if end > self.storage.len() { return Err(FlashError::OutOfBounds); }
        for (i, &b) in data.iter().enumerate() {
            let dst = &mut self.storage[addr + i];
            if (b & *dst) != b {
                return Err(FlashError::DeviceError("attempt to program 0->1"));
            }
            *dst = *dst & b;
        }
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// InternalFlash skeleton
// -----------------------------------------------------------------------------

/// InternalFlash offers a thin wrapper over MCU internal flash.
///
/// The actual erase/program sequences must be implemented per-MCU. Keep the
/// struct state minimal: base address and sizes for validation.
pub struct InternalFlash {
    pub base_addr: usize,
    pub total_size: usize,
    pub sector_size: usize,
    pub page_size: usize,
}

impl InternalFlash {
    pub const fn new(base_addr: usize, total_size: usize, sector_size: usize, page_size: usize) -> Self {
        Self { base_addr, total_size, sector_size, page_size }
    }

    /// Convert a relative flash offset into absolute pointer for read.
    fn abs_addr(&self, rel: usize) -> Result<usize> {
        if rel.checked_add(0).is_none() || rel >= self.total_size {
            return Err(FlashError::OutOfBounds);
        }
        Ok(self.base_addr + rel)
    }
}

impl Flash for InternalFlash {
    fn size(&self) -> usize { self.total_size }
    fn sector_size(&self) -> usize { self.sector_size }
    fn page_size(&self) -> usize { self.page_size }

    fn read(&self, addr: usize, buf: &mut [u8]) -> Result<()> {
        if addr.checked_add(buf.len()).is_none() || addr + buf.len() > self.total_size {
            return Err(FlashError::OutOfBounds);
        }
        let absolute = self.base_addr + addr;
        unsafe {
            let src = absolute as *const u8;
            for i in 0..buf.len() {
                buf[i] = core::ptr::read_volatile(src.add(i));
            }
        }
        Ok(())
    }

    fn erase_sector(&mut self, addr: usize) -> Result<()> {
        if addr >= self.total_size { return Err(FlashError::OutOfBounds); }
        if addr % self.sector_size != 0 { return Err(FlashError::AlignmentError); }
        // MCU-specific erase sequence needed here.
        Err(FlashError::DeviceError("InternalFlash::erase_sector not implemented - fill MCU-specific sequence"))
    }

    fn program_page(&mut self, addr: usize, data: &[u8]) -> Result<()> {
        if addr >= self.total_size { return Err(FlashError::OutOfBounds); }
        if data.len() > self.page_size { return Err(FlashError::AlignmentError); }
        // MCU-specific program sequence needed here.
        Err(FlashError::DeviceError("InternalFlash::program_page not implemented - fill MCU-specific sequence"))
    }
}

// -----------------------------------------------------------------------------
// High-level convenience API using a static InternalFlash instance
// -----------------------------------------------------------------------------

// NOTE: adjust these constants to your MCU memory map in docs/memory_map.md
const FLASH_BASE_ADDR: usize = 0x0800_0000;
const FLASH_TOTAL_BYTES: usize = 512 * 1024;
const FLASH_SECTOR_BYTES: usize = 2048;
const FLASH_PAGE_BYTES: usize = 256;

static mut BOOT_INTERNAL_FLASH: InternalFlash = InternalFlash::new(
    FLASH_BASE_ADDR,
    FLASH_TOTAL_BYTES,
    FLASH_SECTOR_BYTES,
    FLASH_PAGE_BYTES,
);

/// Read `buf.len()` bytes from absolute flash address `addr`.
pub fn read_flash(addr: u32, buf: &mut [u8]) -> Result<()> {
    let rel = addr as usize - FLASH_BASE_ADDR;
    unsafe { BOOT_INTERNAL_FLASH.read(rel, buf) }
}

/// Write `data` to absolute flash address `addr`. This will erase overlapping
/// sectors and program pages, verifying after each page.
pub fn write_flash(addr: u32, data: &[u8]) -> Result<()> {
    let rel = addr as usize - FLASH_BASE_ADDR;
    unsafe { BOOT_INTERNAL_FLASH.write_region(rel, data) }
}

// -----------------------------------------------------------------------------
// Unit tests for host
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash::MockFlash;

    #[test]
    fn mock_basic_flow() {
        let mut f = MockFlash::new(1024, 256, 128);
        assert_eq!(f.size(), 1024);

        let addr = 256;
        let data = vec![0xAAu8; 128];
        f.erase_sector(256).unwrap();
        f.program_page(addr, &data).unwrap();
        assert!(f.verify(addr, &data).is_ok());

        // attempt 0->1 should fail
        let bad = vec![0xFFu8; 128];
        assert!(matches!(f.program_page(addr, &bad), Err(FlashError::DeviceError(_))));
    }

    #[test]
    fn write_region_test() {
        let mut f = MockFlash::new(2048, 256, 128);
        let payload = vec![0x55u8; 300];
        assert!(f.write_region(100, &payload).is_ok());
        assert!(f.verify(100, &payload).is_ok());
    }
}
