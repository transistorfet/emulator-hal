#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;

use emulator_hal::{BusAccess, Instant as EmuInstant, BasicBusError};

/// A contiguous block of memory, backed by a `Vec`
pub struct MemoryBlock<Instant> {
    read_only: bool,
    contents: Vec<u8>,
    instant: PhantomData<Instant>,
}

impl<Instant> MemoryBlock<Instant> {
    /// Construct a memory block from a given `Vec`
    pub fn from(contents: Vec<u8>) -> Self {
        MemoryBlock {
            read_only: false,
            contents,
            instant: PhantomData,
        }
    }

    /// Make this memory block read only
    pub fn read_only(&mut self) {
        self.read_only = true;
    }

    /// Resize the underlying `Vec` to be the given `newsize`
    pub fn resize(&mut self, new_size: usize) {
        self.contents.resize(new_size, 0);
    }
}

#[cfg(feature = "std")]
use std::io;

#[cfg(feature = "std")]
impl<Instant> MemoryBlock<Instant> {
    /// Load the binary contents of a file into a new `MemoryBlock`
    ///
    /// The resulting `MemoryBlock` will be sized to exactly the length of the file
    pub fn load(filename: &str) -> Result<Self, io::Error> {
        let contents = std::fs::read(filename)?;
        Ok(MemoryBlock::from(contents))
    }

    /// Load the binary contents of a file into an existing `MemoryBlock` at the given address
    ///
    /// The `MemoryBlock` must already be big enough to contain the contents of the file
    pub fn load_at(&mut self, addr: usize, filename: &str) -> Result<(), io::Error> {
        let contents = std::fs::read(filename)?;
        self.contents[(addr as usize)..(addr as usize) + contents.len()].copy_from_slice(&contents);
        Ok(())
    }
}

impl<Address, Instant> BusAccess<Address> for MemoryBlock<Instant>
where
    Address: TryInto<usize> + Copy,
    Instant: EmuInstant,
{
    type Instant = Instant;
    type Error = BasicBusError;

    fn read(
        &mut self,
        _now: Instant,
        addr: Address,
        data: &mut [u8],
    ) -> Result<usize, Self::Error> {
        let addr = addr
            .try_into()
            .map_err(|_| BasicBusError::UnmappedAddress)?;
        data.copy_from_slice(&self.contents[addr..addr + data.len()]);
        Ok(data.len())
    }

    fn write(&mut self, _now: Instant, addr: Address, data: &[u8]) -> Result<usize, Self::Error> {
        if self.read_only {
            return Ok(0);
        }

        let addr = addr
            .try_into()
            .map_err(|_| BasicBusError::UnmappedAddress)?;
        self.contents[addr..addr + data.len()].copy_from_slice(data);
        Ok(data.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use emulator_hal::Instant;
    use std::time::Duration;

    #[test]
    fn test_memory_block() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct SimpleError();

        impl From<core::num::TryFromIntError> for SimpleError {
            fn from(_: core::num::TryFromIntError) -> Self {
                SimpleError()
            }
        }

        let mut memory = MemoryBlock::<Duration>::from(vec![0; 1024]);

        let number = 0x1234_5678;
        memory.write_leu32(Duration::START, 0, number).unwrap();
        let result = memory.read_leu32(Duration::START, 0).unwrap();
        assert_eq!(result, number);
    }
}
