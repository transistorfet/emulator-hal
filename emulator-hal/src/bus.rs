//! Traits for emulating read and write bus operations

use crate::time::Instant;
use core::convert::Infallible;
use core::fmt;

/// Represents an error that occurred during a bus transaction
pub trait Error: fmt::Debug {}

impl Error for Infallible {}

/// A simple pre-defined error type for bus transactions
#[derive(Debug)]
#[non_exhaustive]
pub enum BasicBusError {
    /// A write access was requested, but the target is read-only
    ReadOnly,

    /// The address requested is not mapped to a device, so no data can be returned
    UnmappedAddress,

    /// Some other kind of error has occurred
    #[cfg(feature = "alloc")]
    Other(alloc::boxed::Box<dyn Error>),

    /// Some other kind of error has occurred
    #[cfg(not(feature = "alloc"))]
    Other,
}

impl Error for BasicBusError {}

/// Represents the order of bytes in a `BusAccess` operation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little endian byte order
    Little,
    /// Big endian byte order
    Big,
}

/// A device that can be addressed to read data from or write data to the device.
///
/// This represents access to a peripheral device or a bus of multiple devices, which can be
/// used by a controller (eg. CPU).  The address can either be a single number or a tuple to
/// represent different address spaces, such as memory vs I/O spaces as in the Z80 CPUs, or
/// supervisor vs user access as in the Function Code present on 68k CPUs.
pub trait BusAccess<Address>
where
    Address: Copy,
{
    /// The type of an instant in simulated time that the bus access is meant to occur at
    type Instant: Instant;

    /// The type of an error returned by this bus
    type Error: Error;

    /// Read an arbitrary length of bytes from this device, at time `now`
    ///
    /// Returns the number of bytes read, which would normally be the same as `data.len()`
    /// but could be less or zero if no data is returned
    fn read(
        &mut self,
        now: Self::Instant,
        addr: Address,
        data: &mut [u8],
    ) -> Result<usize, Self::Error>;

    /// Write an arbitrary length of bytes into this device, at time `now`
    ///
    /// Returns the number of bytes written, which would normally be the same as `data.len()`
    /// but could be less or zero if no data was written or the memory was read-only
    fn write(
        &mut self,
        now: Self::Instant,
        addr: Address,
        data: &[u8],
    ) -> Result<usize, Self::Error>;

    /// Read a single u8 value at the given address
    #[inline]
    fn read_u8(&mut self, now: Self::Instant, addr: Address) -> Result<u8, Self::Error> {
        let mut data = [0; 1];
        self.read(now, addr, &mut data)?;
        Ok(data[0])
    }

    /// Read a single u16 value in big endian byte order at the given address
    #[inline]
    fn read_beu16(&mut self, now: Self::Instant, addr: Address) -> Result<u16, Self::Error> {
        let mut data = [0; 2];
        self.read(now, addr, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    /// Read a single u16 value in little endian byte order at the given address
    #[inline]
    fn read_leu16(&mut self, now: Self::Instant, addr: Address) -> Result<u16, Self::Error> {
        let mut data = [0; 2];
        self.read(now, addr, &mut data)?;
        Ok(u16::from_le_bytes(data))
    }

    /// Read a single u16 value in the given byte order at the given address
    #[inline]
    fn read_u16(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
    ) -> Result<u16, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu16(now, addr),
            ByteOrder::Big => self.read_beu16(now, addr),
        }
    }

    /// Read a single u32 value in big endian byte order at the given address
    #[inline]
    fn read_beu32(&mut self, now: Self::Instant, addr: Address) -> Result<u32, Self::Error> {
        let mut data = [0; 4];
        self.read(now, addr, &mut data)?;
        Ok(u32::from_be_bytes(data))
    }

    /// Read a single u32 value in little endian byte order at the given address
    #[inline]
    fn read_leu32(&mut self, now: Self::Instant, addr: Address) -> Result<u32, Self::Error> {
        let mut data = [0; 4];
        self.read(now, addr, &mut data)?;
        Ok(u32::from_le_bytes(data))
    }

    /// Read a single u32 value in the given byte order at the given address
    #[inline]
    fn read_u32(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
    ) -> Result<u32, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu32(now, addr),
            ByteOrder::Big => self.read_beu32(now, addr),
        }
    }

    /// Read a single u64 value in big endian byte order at the given address
    #[inline]
    fn read_beu64(&mut self, now: Self::Instant, addr: Address) -> Result<u64, Self::Error> {
        let mut data = [0; 8];
        self.read(now, addr, &mut data)?;
        Ok(u64::from_be_bytes(data))
    }

    /// Read a single u64 value in little endian byte order at the given address
    #[inline]
    fn read_leu64(&mut self, now: Self::Instant, addr: Address) -> Result<u64, Self::Error> {
        let mut data = [0; 8];
        self.read(now, addr, &mut data)?;
        Ok(u64::from_le_bytes(data))
    }

    /// Read a single u64 value in the given byte order at the given address
    #[inline]
    fn read_u64(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
    ) -> Result<u64, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu64(now, addr),
            ByteOrder::Big => self.read_beu64(now, addr),
        }
    }

    /// Write a single u8 value to the given address
    #[inline]
    fn write_u8(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u8,
    ) -> Result<(), Self::Error> {
        let data = [value];
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u16 value in big endian byte order to the given address
    #[inline]
    fn write_beu16(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u16,
    ) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u16 value in little endian byte order to the given address
    #[inline]
    fn write_leu16(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u16,
    ) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u16 value in the given byte order to the given address
    #[inline]
    fn write_u16(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
        value: u16,
    ) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu16(now, addr, value),
            ByteOrder::Big => self.write_beu16(now, addr, value),
        }
    }

    /// Write the given u32 value in big endian byte order to the given address
    #[inline]
    fn write_beu32(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u32,
    ) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u32 value in little endian byte order to the given address
    #[inline]
    fn write_leu32(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u32,
    ) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u32 value in the given byte order to the given address
    #[inline]
    fn write_u32(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
        value: u32,
    ) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu32(now, addr, value),
            ByteOrder::Big => self.write_beu32(now, addr, value),
        }
    }

    /// Write the given u64 value in big endian byte order to the given address
    #[inline]
    fn write_beu64(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u64,
    ) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u64 value in little endian byte order to the given address
    #[inline]
    fn write_leu64(
        &mut self,
        now: Self::Instant,
        addr: Address,
        value: u64,
    ) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)?;
        Ok(())
    }

    /// Write the given u64 value in the given byte order to the given address
    #[inline]
    fn write_u64(
        &mut self,
        order: ByteOrder,
        now: Self::Instant,
        addr: Address,
        value: u64,
    ) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu64(now, addr, value),
            ByteOrder::Big => self.write_beu64(now, addr, value),
        }
    }
}

impl<Address, T> BusAccess<Address> for &mut T
where
    Address: Copy,
    T: BusAccess<Address> + ?Sized,
{
    type Instant = T::Instant;
    type Error = T::Error;

    #[inline]
    fn read(
        &mut self,
        now: Self::Instant,
        addr: Address,
        data: &mut [u8],
    ) -> Result<usize, T::Error> {
        T::read(self, now, addr, data)
    }

    #[inline]
    fn write(&mut self, now: Self::Instant, addr: Address, data: &[u8]) -> Result<usize, T::Error> {
        T::write(self, now, addr, data)
    }
}

#[cfg(feature = "alloc")]
impl<Address, T> BusAccess<Address> for alloc::boxed::Box<T>
where
    Address: Copy,
    T: BusAccess<Address> + ?Sized,
{
    type Instant = T::Instant;
    type Error = T::Error;

    #[inline]
    fn read(
        &mut self,
        now: Self::Instant,
        addr: Address,
        data: &mut [u8],
    ) -> Result<usize, T::Error> {
        T::read(self, now, addr, data)
    }

    #[inline]
    fn write(&mut self, now: Self::Instant, addr: Address, data: &[u8]) -> Result<usize, T::Error> {
        T::write(self, now, addr, data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_implemeting_memory() {
        #[derive(Clone, Debug)]
        enum Error {}

        impl super::Error for Error {}

        struct Memory(Vec<u8>);

        impl BusAccess<u64> for Memory {
            type Instant = Duration;
            type Error = Error;

            fn read(
                &mut self,
                _now: Duration,
                addr: u64,
                data: &mut [u8],
            ) -> Result<usize, Self::Error> {
                let addr = addr as usize;
                data.copy_from_slice(&self.0[addr..addr + data.len()]);
                Ok(data.len())
            }

            fn write(
                &mut self,
                _now: Duration,
                addr: u64,
                data: &[u8],
            ) -> Result<usize, Self::Error> {
                let addr = addr as usize;
                self.0[addr..addr + data.len()].copy_from_slice(data);
                Ok(data.len())
            }
        }

        let mut bus = Memory(vec![0; 1024]);

        let number = 0x1234_5678;
        bus.write_beu32(Duration::START, 0, number).unwrap();
        assert_eq!(
            u32::from_be_bytes(bus.0[0..4].try_into().unwrap()),
            0x1234_5678
        );

        assert_eq!(
            bus.read_u32(ByteOrder::Big, Duration::START, 0).unwrap(),
            number
        );
    }
}
