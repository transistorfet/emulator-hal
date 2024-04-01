//! Bus Adapters to translate address and error type

use crate::{BusAccess, Error};

/// Used to translate an address from one address space into another
pub trait FromAddress<T> {
    /// Translate the given address into an address of type `Self`
    fn from_address(address: T) -> Self;
}

/// Used to translate an address from one address space into another
pub trait IntoAddress<T> {
    /// Translate the given address into an address of type `T`
    fn into_address(self) -> T;
}

impl<T, S> IntoAddress<T> for S
where
    T: FromAddress<S>,
{
    fn into_address(self) -> T {
        T::from_address(self)
    }
}

/// An adapter that applies an address translation before accessing a wrapped bus object
///
/// This object implements the `BusAccess` trait, and takes address of type `AddressIn`,
/// applies the provided address translation function to produce an address of type `AddressOut`,
/// and then calls the equivalent trait method with that produced address, return the result
pub struct BusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: Copy,
    Bus: BusAccess<AddressOut>,
{
    /// The underlying object implementing `BusAccess` that this object adapts
    pub inner: Bus,
    /// The translation function applied
    pub translate: fn(AddressIn) -> AddressOut,
    /// The error mapping function applied
    pub map_err: fn(Bus::Error) -> ErrorOut,
}

impl<AddressIn, AddressOut, Bus, ErrorOut> BusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: Copy,
    Bus: BusAccess<AddressOut>,
{
    /// Construct a new instance of an adapter for the given `bus` object
    pub fn new(
        inner: Bus,
        translate: fn(AddressIn) -> AddressOut,
        map_err: fn(Bus::Error) -> ErrorOut,
    ) -> Self {
        Self {
            inner,
            translate,
            map_err,
        }
    }
}

impl<AddressIn, AddressOut, Bus, ErrorOut> BusAccess<AddressIn>
    for BusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: Error,
{
    type Instant = Bus::Instant;
    type Error = ErrorOut;

    #[inline]
    fn read(
        &mut self,
        now: Self::Instant,
        addr: AddressIn,
        data: &mut [u8],
    ) -> Result<usize, Self::Error> {
        let addr = (self.translate)(addr);
        self.inner.read(now, addr, data).map_err(self.map_err)
    }

    #[inline]
    fn write(
        &mut self,
        now: Self::Instant,
        addr: AddressIn,
        data: &[u8],
    ) -> Result<usize, Self::Error> {
        let addr = (self.translate)(addr);
        self.inner.write(now, addr, data).map_err(self.map_err)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_implemeting_memory() {
        // TODO write tests
    }
}
