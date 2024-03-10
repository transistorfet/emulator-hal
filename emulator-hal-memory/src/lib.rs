#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

extern crate alloc;

use core::marker::PhantomData;
use alloc::vec::Vec;

use emulator_hal::bus::{BusAccess, SimpleBusError};

/// A contiguous block of memory, backed by a `Vec`
pub struct MemoryBlock<Address, Instant>
where
    Address: Copy,
{
    read_only: bool,
    contents: Vec<u8>,
    address: PhantomData<Address>,
    instant: PhantomData<Instant>,
}

impl<Address, Instant> MemoryBlock<Address, Instant>
where
    Address: Copy,
{
    /// Construct a memory block from a given `Vec`
    pub fn from(contents: Vec<u8>) -> Self {
        MemoryBlock {
            read_only: false,
            contents,
            address: PhantomData,
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
impl<Address, Instant> MemoryBlock<Address, Instant>
where
    Address: TryInto<usize> + Copy,
{
    /*
    pub fn load(filename: &str) -> Result<Self, Bus::Error> {
        use std::fs;

        match fs::read(filename) {
            Ok(contents) => Ok(MemoryBlock::from(contents)),
            Err(_) => Err(Error::new(format!("Error reading contents of {}", filename))),
        }
    }

    pub fn load_at(&mut self, addr: Address, filename: &str) -> Result<(), Bus::Error> {
        use std::fs;

        let addr = addr.try_into()?;
        match fs::read(filename) {
            Ok(contents) => {
                self.contents[(addr as usize)..(addr as usize) + contents.len()].copy_from_slice(&contents);
                Ok(())
            },
            Err(_) => Err(Error::new(format!("Error reading contents of {}", filename))),
        }
    }
    */
}

impl<Address, Instant> BusAccess<Address, Instant> for MemoryBlock<Address, Instant>
where
    Address: TryInto<usize> + Copy,
{
    type Error = SimpleBusError;

    fn read(&mut self, _now: Instant, addr: Address, data: &mut [u8]) -> Result<usize, Self::Error> {
        let addr = addr.try_into()
            .map_err(|_| SimpleBusError::UnmappedAddress)?;
        data.copy_from_slice(&self.contents[addr .. addr + data.len()]);
        Ok(data.len())
    }

    fn write(&mut self, _now: Instant, addr: Address, data: &[u8]) -> Result<usize, Self::Error> {
        if self.read_only {
            //return Err(Error::breakpoint(format!("Attempt to write to read-only memory at {:x} with data {:?}", addr, data)));
        }

        let addr = addr.try_into()
            .map_err(|_| SimpleBusError::UnmappedAddress)?;
        self.contents[addr .. addr + data.len()].copy_from_slice(data);
        Ok(data.len())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use std::time::Instant;

    #[test]
    fn test_memory_block() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct SimpleError();

        impl From<core::num::TryFromIntError> for SimpleError {
            fn from(_: core::num::TryFromIntError) -> Self {
                SimpleError()
            }
        }

        let mut memory = MemoryBlock::<u64, Instant>::from(vec![0; 1024]);

        let number = 0x1234_5678;
        memory.write_leu32(Instant::now(), 0, number).unwrap();
        let result = memory.read_leu32(Instant::now(), 0).unwrap();
        assert_eq!(result, number);
    }
}
