//! Traits for CPU and Peripheral devices that advance their internal state via a clock signal

use core::fmt;

use crate::bus::BusAccess;

/// Represents a device that can change state with the passage of a clock signal
///
/// Typically this would represent both CPU devices and peripheral devices that use a clock
/// signal to advance some internal process, such as a timer or state machine
pub trait Step<Address, Instant, Bus>
where
    Address: Copy,
    Bus: BusAccess<Address, Instant>,
{
    /// A type that is return if the step cannot be performed
    ///
    /// Note: this is not the same as BusAccess::Error
    type Error; //: From<Bus::Error>;

    /// Returns true if this device is still running.  This can be used to detect a stop or halt condition
    fn is_running(&mut self) -> bool;

    /// Reset the device to its initial state, as if the device's reset signal was asserted
    fn reset(&mut self, now: Instant, bus: &mut Bus) -> Result<(), Self::Error>;

    /// Step the process by one unit of time, and return the time at which this function should be called again
    ///
    /// The given `Instant` is the time at which this step occurs, and the returned `Instant` is the time that the
    /// next step should occur, according to the device itself.  The given bus can be used to access the system
    /// during this step of execution
    fn step(&mut self, now: Instant, bus: &mut Bus) -> Result<Instant, Self::Error>;
}

// TODO should this depend on Step, which is the most common way it will be used, even though it technically could
// be used for a device that just has a bus interface with no clock
/// Inspect the state of a device, and emit it to an object that implements `fmt::Write`
pub trait Inspect<Address, Instant, Bus, Writer>
where
    Address: Copy,
    Bus: BusAccess<Address, Instant>,
    Writer: fmt::Write,
{
    /// A type that describes the types of information or state that this device can emit
    type InfoType;

    /// A type that is returned if the data cannot be written as expected
    type Error;

    /// Write the given information type to the given writer, or return an error
    fn inspect(
        &mut self,
        writer: &mut Writer,
        info: Self::InfoType,
        bus: &mut Bus,
    ) -> Result<(), Self::Error>;
}

/// Control the execution of a CPU device for debugging purposes
pub trait Debug<Address, Instant, Bus, Writer>:
    Inspect<Address, Instant, Bus, Writer> + Step<Address, Instant, Bus>
where
    Address: Copy,
    Bus: BusAccess<Address, Instant>,
    Writer: fmt::Write,
{
    /// Represents an error that can occur while debugging
    type DebugError;

    /// Returns the `Address` where execution will take place the next time `step()` is called
    fn get_execution_address(&mut self) -> Result<Address, Self::DebugError>;
    /// Sets the `Address` where execution will take place the next time `step()` is called
    fn set_execution_address(&mut self, address: Address) -> Result<(), Self::DebugError>;

    // TODO this is too vague
    /// Perform a debug command
    fn run_command(&mut self, bus: &mut Bus, args: &[&str]) -> Result<bool, Self::DebugError>;
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::bus::{self, BusAdapter, SimpleBusError};
    use std::ops::Range;
    use std::str;
    use std::time::{Duration, Instant};

    #[derive(Clone, Debug)]
    enum Error {
        BusError,
    }

    impl bus::Error for Error {}

    struct Memory(Vec<u8>);

    impl BusAccess<u32, Instant> for Memory {
        type Error = SimpleBusError;

        fn read(
            &mut self,
            _now: Instant,
            addr: u32,
            data: &mut [u8],
        ) -> Result<usize, Self::Error> {
            let addr = addr as usize;
            data.copy_from_slice(&self.0[addr..addr + data.len()]);
            Ok(data.len())
        }

        fn write(&mut self, _now: Instant, addr: u32, data: &[u8]) -> Result<usize, Self::Error> {
            let addr = addr as usize;
            self.0[addr..addr + data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    #[derive(Clone, Debug)]
    enum OutputError {
        Utf8Error,
    }

    impl bus::Error for OutputError {}

    struct Output();

    impl BusAccess<u16, Instant> for Output {
        type Error = OutputError;

        fn read(
            &mut self,
            _now: Instant,
            _addr: u16,
            _data: &mut [u8],
        ) -> Result<usize, Self::Error> {
            Ok(0)
        }

        fn write(&mut self, _now: Instant, _addr: u16, data: &[u8]) -> Result<usize, Self::Error> {
            let string = str::from_utf8(data).map_err(|_| OutputError::Utf8Error)?;
            print!("{}", string);
            Ok(data.len())
        }
    }

    struct FixedBus {
        output: Output,
        memory: Memory,
    }

    impl BusAccess<u64, Instant> for FixedBus {
        type Error = Error;

        fn read(&mut self, now: Instant, addr: u64, data: &mut [u8]) -> Result<usize, Self::Error> {
            if (0..0x1_0000).contains(&addr) {
                self.memory
                    .read(now, addr as u32 % 0x1_0000, data)
                    .map_err(|_| Error::BusError)
            } else {
                self.output
                    .read(now, addr as u16, data)
                    .map_err(|_| Error::BusError)
            }
        }

        fn write(&mut self, now: Instant, addr: u64, data: &[u8]) -> Result<usize, Self::Error> {
            if (0..0x1_0000).contains(&addr) {
                self.memory
                    .write(now, addr as u32 % 0x1_0000, data)
                    .map_err(|_| Error::BusError)
            } else {
                self.output
                    .write(now, addr as u16, data)
                    .map_err(|_| Error::BusError)
            }
        }
    }

    struct DynamicBus {
        devices: Vec<(Range<u64>, Box<dyn BusAccess<u64, Instant, Error = Error>>)>,
    }

    impl BusAccess<u64, Instant> for DynamicBus {
        type Error = Error;

        fn read(&mut self, now: Instant, addr: u64, data: &mut [u8]) -> Result<usize, Self::Error> {
            for (range, device) in self.devices.iter_mut() {
                if range.contains(&addr) {
                    return device.read(now, addr, data).map_err(|_| Error::BusError);
                }
            }
            Ok(0)
        }

        fn write(&mut self, now: Instant, addr: u64, data: &[u8]) -> Result<usize, Self::Error> {
            for (range, device) in self.devices.iter_mut() {
                if range.contains(&addr) {
                    return device.write(now, addr, data).map_err(|_| Error::BusError);
                }
            }
            Ok(0)
        }
    }

    #[derive(Default)]
    struct Cpu {
        pc: u64,
        sum: u32,
        running: bool,
    }

    impl<Bus> Step<u64, Instant, Bus> for Cpu
    where
        Bus: BusAccess<u64, Instant>,
        Error: From<Bus::Error>,
    {
        type Error = Error;

        fn is_running(&mut self) -> bool {
            self.running
        }

        fn reset(&mut self, now: Instant, bus: &mut Bus) -> Result<(), Self::Error> {
            self.running = true;
            self.pc = bus.read_beu32(now, 0x0000)? as u64;
            Ok(())
        }

        fn step(&mut self, now: Instant, bus: &mut Bus) -> Result<Instant, Self::Error> {
            if self.running {
                let value = bus.read_beu32(now, self.pc)?;
                self.pc += 4;

                if value == 0 {
                    self.running = false;
                } else {
                    self.sum += value;
                }
            }
            Ok(now + Duration::from_nanos(100))
        }
    }

    #[test]
    fn test_static_system() {
        let memory = Memory(vec![0; 1024]);
        let output = Output();

        let mut bus = FixedBus { memory, output };

        let mut cpu = Cpu::default();

        let location = 0x100;
        bus.memory
            .write_beu32(Instant::now(), 0x0000, location as u32)
            .unwrap();

        for i in 0..100 {
            bus.memory
                .write_beu32(Instant::now(), location + 4 * i as u32, 1 + i as u32)
                .unwrap();
        }

        fn run_static_test<A, B, C>(bus: &mut B, cpu: &mut C) -> Result<(), C::Error>
        where
            A: Copy,
            B: BusAccess<A, Instant>,
            C: Step<A, Instant, B>,
            C::Error: From<B::Error>,
        {
            cpu.reset(Instant::now(), bus)?;

            while cpu.is_running() {
                cpu.step(Instant::now(), bus)?;
            }
            Ok(())
        }

        run_static_test(&mut bus, &mut cpu).unwrap();

        assert_eq!(cpu.sum, 5050);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dynamic_system() {
        let memory = Memory(vec![0; 1024]);
        let output = Output();

        let mut bus = DynamicBus {
            devices: vec![
                (
                    0..0x1_0000,
                    Box::new(BusAdapter::new(
                        memory,
                        |addr| addr as u32,
                        |_| Error::BusError,
                    )),
                ),
                (
                    0x2_0000..0x2_0010,
                    Box::new(BusAdapter::new(
                        output,
                        |addr| addr as u16,
                        |_| Error::BusError,
                    )),
                ),
            ],
        };

        let mut cpu = Cpu::default();

        let location = 0x100 as u64;
        bus.write_beu32(Instant::now(), 0x0000, location as u32)
            .unwrap();

        for i in 0..100 {
            bus.write_beu32(Instant::now(), location + 4 * i as u64, 1 + i as u32)
                .unwrap();
        }

        type Bus = Box<dyn BusAccess<u64, Instant, Error = Error>>;

        //let _trait_obj_cpu: &mut dyn Step<Bus, Error = Error> = &mut cpu;

        fn run_dynamic_test(
            mut bus: Bus,
            cpu: &mut dyn Step<u64, Instant, Bus, Error = Error>,
        ) -> Result<(), Error> {
            cpu.reset(Instant::now(), &mut bus)?;

            while cpu.is_running() {
                cpu.step(Instant::now(), &mut bus)?;
            }
            Ok(())
        }

        run_dynamic_test(Box::new(bus), &mut cpu).unwrap();

        assert_eq!(cpu.sum, 5050);
    }
}