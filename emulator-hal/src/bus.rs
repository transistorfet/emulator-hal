
use core::fmt;
use core::ops::{Add, Sub};


/// Represents the types common to a bus abstraction
pub trait BusType {
    /// A measure of time at which a bus transaction can occur
    type Instant;

    /// An address used by this bus
    type Address: Copy;

    /// The type of an error returned by this bus
    type Error: fmt::Debug;
}

impl<T: BusType + ?Sized> BusType for &mut T {
    type Instant = T::Instant;
    type Address = T::Address;
    type Error = T::Error;
}

#[cfg(feature = "alloc")]
impl<T: BusType + ?Sized> BusType for alloc::boxed::Box<T> {
    type Instant = T::Instant;
    type Address = T::Address;
    type Error = T::Error;
}

/// Represents the order of bytes in a `BusAccess` operation
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    Little,
    Big,
}

/// This interface is meant to be used by the initiator of a bus request (eg. a controller)
/// and should be implemented by devices that can respond to a bus request (eg. peripherals)

/// A device that can be addressed to read data from or write data to the device.
///
/// This represents access to a peripheral device or a bus of multiple devices, which can be
/// used by a controller (eg. CPU).  The address can either be a single number of a tuple to
/// represent different address spaces, such as memory vs I/O spaces as in the Z80 CPUs, or
/// supervisor vs user access as in the Function Code present on 68k CPUs.
pub trait BusAccess: BusType {
    /// Returns the size of the addressable block that this device represents
    //fn size(&self) -> usize;

    /// Read an arbitrary length of bytes from this device, at time `now`
    fn read(&mut self, now: Self::Instant, addr: Self::Address, data: &mut [u8]) -> Result<(), Self::Error>;

    /// Write an arbitrary length of bytes into this device, at time `now`
    fn write(&mut self, now: Self::Instant, addr: Self::Address, data: &[u8]) -> Result<(), Self::Error>;

    /// Read a single u8 value at the given address
    #[inline]
    fn read_u8(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u8, Self::Error> {
        let mut data = [0; 1];
        self.read(now, addr, &mut data)?;
        Ok(data[0])
    }

    /// Read a single u16 value in big endian byte order at the given address
    #[inline]
    fn read_beu16(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u16, Self::Error> {
        let mut data = [0; 2];
        self.read(now, addr, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    /// Read a single u16 value in little endian byte order at the given address
    #[inline]
    fn read_leu16(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u16, Self::Error> {
        let mut data = [0; 2];
        self.read(now, addr, &mut data)?;
        Ok(u16::from_le_bytes(data))
    }

    /// Read a single u16 value in the given byte order at the given address
    #[inline]
    fn read_u16(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address) -> Result<u16, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu16(now, addr),
            ByteOrder::Big => self.read_beu16(now, addr),
        }
    }

    /// Read a single u32 value in big endian byte order at the given address
    #[inline]
    fn read_beu32(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u32, Self::Error> {
        let mut data = [0; 4];
        self.read(now, addr, &mut data)?;
        Ok(u32::from_be_bytes(data))
    }

    /// Read a single u32 value in little endian byte order at the given address
    #[inline]
    fn read_leu32(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u32, Self::Error> {
        let mut data = [0; 4];
        self.read(now, addr, &mut data)?;
        Ok(u32::from_le_bytes(data))
    }

    /// Read a single u32 value in the given byte order at the given address
    #[inline]
    fn read_u32(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address) -> Result<u32, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu32(now, addr),
            ByteOrder::Big => self.read_beu32(now, addr),
        }
    }

    /// Read a single u64 value in big endian byte order at the given address
    #[inline]
    fn read_beu64(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u64, Self::Error> {
        let mut data = [0; 8];
        self.read(now, addr, &mut data)?;
        Ok(u64::from_be_bytes(data))
    }

    /// Read a single u64 value in little endian byte order at the given address
    #[inline]
    fn read_leu64(&mut self, now: Self::Instant, addr: Self::Address) -> Result<u64, Self::Error> {
        let mut data = [0; 8];
        self.read(now, addr, &mut data)?;
        Ok(u64::from_le_bytes(data))
    }

    /// Read a single u64 value in the given byte order at the given address
    #[inline]
    fn read_u64(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address) -> Result<u64, Self::Error> {
        match order {
            ByteOrder::Little => self.read_leu64(now, addr),
            ByteOrder::Big => self.read_beu64(now, addr),
        }
    }

    /// Write a single u8 value to the given address
    #[inline]
    fn write_u8(&mut self, now: Self::Instant, addr: Self::Address, value: u8) -> Result<(), Self::Error> {
        let data = [value];
        self.write(now, addr, &data)
    }

    /// Write the given u16 value in big endian byte order to the given address
    #[inline]
    fn write_beu16(&mut self, now: Self::Instant, addr: Self::Address, value: u16) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u16 value in little endian byte order to the given address
    #[inline]
    fn write_leu16(&mut self, now: Self::Instant, addr: Self::Address, value: u16) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u16 value in the given byte order to the given address
    #[inline]
    fn write_u16(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address, value: u16) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu16(now, addr, value),
            ByteOrder::Big => self.write_beu16(now, addr, value),
        }
    }

    /// Write the given u32 value in big endian byte order to the given address
    #[inline]
    fn write_beu32(&mut self, now: Self::Instant, addr: Self::Address, value: u32) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u32 value in little endian byte order to the given address
    #[inline]
    fn write_leu32(&mut self, now: Self::Instant, addr: Self::Address, value: u32) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u32 value in the given byte order to the given address
    #[inline]
    fn write_u32(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address, value: u32) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu32(now, addr, value),
            ByteOrder::Big => self.write_beu32(now, addr, value),
        }
    }

    /// Write the given u64 value in big endian byte order to the given address
    #[inline]
    fn write_beu64(&mut self, now: Self::Instant, addr: Self::Address, value: u64) -> Result<(), Self::Error> {
        let data = value.to_be_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u64 value in little endian byte order to the given address
    #[inline]
    fn write_leu64(&mut self, now: Self::Instant, addr: Self::Address, value: u64) -> Result<(), Self::Error> {
        let data = value.to_le_bytes();
        self.write(now, addr, &data)
    }

    /// Write the given u64 value in the given byte order to the given address
    #[inline]
    fn write_u64(&mut self, order: ByteOrder, now: Self::Instant, addr: Self::Address, value: u64) -> Result<(), Self::Error> {
        match order {
            ByteOrder::Little => self.write_leu64(now, addr, value),
            ByteOrder::Big => self.write_beu64(now, addr, value),
        }
    }
}

impl<T: BusAccess + ?Sized> BusAccess for &mut T {
    #[inline]
    fn read(&mut self, now: T::Instant, addr: T::Address, data: &mut [u8]) -> Result<(), T::Error> {
        T::read(self, now, addr, data)
    }

    #[inline]
    fn write(&mut self, now: T::Instant, addr: T::Address, data: &[u8]) -> Result<(), T::Error> {
        T::write(self, now, addr, data)
    }
}

#[cfg(feature = "alloc")]
impl<T: BusAccess + ?Sized> BusAccess for alloc::boxed::Box<T> {
    #[inline]
    fn read(&mut self, now: T::Instant, addr: T::Address, data: &mut [u8]) -> Result<(), T::Error> {
        T::read(self, now, addr, data)
    }

    #[inline]
    fn write(&mut self, now: T::Instant, addr: T::Address, data: &[u8]) -> Result<(), T::Error> {
        T::write(self, now, addr, data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_implemeting() {
        #[derive(Clone, Debug)]
        enum Error {

        }

        struct Memory(Vec<u8>);

        impl BusType for Memory {
            type Error = Error;
            type Instant = Instant;
            type Address = u64;
        }

        impl BusAccess for Memory {
            fn read(&mut self, _now: Instant, addr: u64, data: &mut [u8]) -> Result<(), Self::Error> {
                let addr = addr as usize;
                data.copy_from_slice(&self.0[addr..addr + data.len()]);
                Ok(())
            }

            fn write(&mut self, _now: Instant, addr: u64, data: &[u8]) -> Result<(), Self::Error> {
                let addr = addr as usize;
                self.0[addr..addr + data.len()].copy_from_slice(data);
                Ok(())
            }
        }

        let mut bus = Memory(vec![0; 1024]);

        let number = 0x1234_5678;
        bus.write_beu32(Instant::now(), 0, number).unwrap();
        assert_eq!(u32::from_be_bytes(bus.0[0..4].try_into().unwrap()), 0x1234_5678);

        assert_eq!(bus.read_u32(ByteOrder::Big, Instant::now(), 0).unwrap(), number);
    }
}
