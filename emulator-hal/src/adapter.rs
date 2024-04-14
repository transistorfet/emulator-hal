//! Bus Adapters to translate address and error type

use crate::{BusAccess, Error};
use core::marker::PhantomData;

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
    ErrorOut: From<Bus::Error>,
{
    /// The underlying object implementing `BusAccess` that this object adapts
    pub inner: Bus,
    /// The translation function applied
    pub translate: fn(AddressIn) -> AddressOut,
    /// Marker for the error type
    error_out: PhantomData<ErrorOut>,
}

impl<AddressIn, AddressOut, Bus, ErrorOut> BusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: From<Bus::Error>,
{
    /// Construct a new instance of an adapter for the given `bus` object
    pub fn new(inner: Bus, translate: fn(AddressIn) -> AddressOut) -> Self {
        Self {
            inner,
            translate,
            error_out: PhantomData,
        }
    }
}

impl<AddressIn, AddressOut, Bus, ErrorOut> BusAccess<AddressIn>
    for BusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: Error + From<Bus::Error>,
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
        self.inner.read(now, addr, data).map_err(|err| err.into())
    }

    #[inline]
    fn write(
        &mut self,
        now: Self::Instant,
        addr: AddressIn,
        data: &[u8],
    ) -> Result<usize, Self::Error> {
        let addr = (self.translate)(addr);
        self.inner.write(now, addr, data).map_err(|err| err.into())
    }
}

/// An adapter that uses the `FromAddress` trait to translate an address before accessing a wrapped bus object
///
/// This object implements the `BusAccess` trait, and takes address of type `AddressIn`,
/// applies `FromAddress<AddressIn>` trait to produce an address of type `AddressOut`,
/// and then calls the equivalent trait method with that produced address, return the result
pub struct AutoBusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressOut: FromAddress<AddressIn> + Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: From<Bus::Error>,
{
    /// The underlying object implementing `BusAccess` that this object adapts
    pub inner: Bus,

    address_in: PhantomData<AddressIn>,
    address_out: PhantomData<AddressOut>,
    error_out: PhantomData<ErrorOut>,
}

impl<AddressIn, AddressOut, Bus, ErrorOut> AutoBusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressOut: FromAddress<AddressIn> + Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: From<Bus::Error>,
{
    /// Construct a new instance of an adapter for the given `bus` object
    pub fn new(inner: Bus) -> Self {
        Self {
            inner,
            address_in: PhantomData,
            address_out: PhantomData,
            error_out: PhantomData,
        }
    }
}

impl<AddressIn, AddressOut, Bus, ErrorOut> BusAccess<AddressIn>
    for AutoBusAdapter<AddressIn, AddressOut, Bus, ErrorOut>
where
    AddressIn: Copy,
    AddressOut: FromAddress<AddressIn> + Copy,
    Bus: BusAccess<AddressOut>,
    ErrorOut: Error + From<Bus::Error>,
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
        let addr = addr.into_address();
        self.inner.read(now, addr, data).map_err(|err| err.into())
    }

    #[inline]
    fn write(
        &mut self,
        now: Self::Instant,
        addr: AddressIn,
        data: &[u8],
    ) -> Result<usize, Self::Error> {
        let addr = addr.into_address();
        self.inner.write(now, addr, data).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

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

        fn write(&mut self, _now: Duration, addr: u64, data: &[u8]) -> Result<usize, Self::Error> {
            let addr = addr as usize;
            self.0[addr..addr + data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    type Address = u8;
    impl FromAddress<Address> for u64 {
        fn from_address(address: Address) -> u64 {
            address as u64
        }
    }

    #[derive(Clone, Debug)]
    enum Error2 {
        BusError,
    }

    impl super::Error for Error2 {}

    impl From<Error> for Error2 {
        fn from(_err: Error) -> Self {
            Error2::BusError
        }
    }

    #[test]
    fn test_adapt_address() {
        let bus = Memory(vec![0; 1024]);

        let mut adapter = BusAdapter::new(bus, |addr| addr as u64);

        let expected_value = 0x1234;
        adapter
            .write_beu16(Duration::ZERO, 0, expected_value)
            .unwrap();
        let result: Result<u16, Error> = adapter.read_beu16(Duration::ZERO, 0);
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_adapt_error() {
        let bus = Memory(vec![0; 1024]);

        let mut adapter = BusAdapter::new(bus, |addr| addr as u64);

        let expected_value = 0x1234;
        adapter
            .write_beu16(Duration::ZERO, 0, expected_value)
            .unwrap();
        let result: Result<u16, Error2> = adapter.read_beu16(Duration::ZERO, 0);
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_auto_adapt_address() {
        let bus = Memory(vec![0; 1024]);

        let mut adapter = AutoBusAdapter::new(bus);

        let expected_value = 0x1234;
        adapter
            .write_beu16(Duration::ZERO, 0, expected_value)
            .unwrap();
        let result: Result<u16, Error> = adapter.read_beu16(Duration::ZERO, 0);
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_auto_adapt_error() {
        let bus = Memory(vec![0; 1024]);

        let mut adapter = AutoBusAdapter::new(bus);

        let expected_value = 0x1234;
        adapter
            .write_beu16(Duration::ZERO, 0, expected_value)
            .unwrap();
        let result: Result<u16, Error2> = adapter.read_beu16(Duration::ZERO, 0);
        assert_eq!(result.unwrap(), expected_value);
    }
}
