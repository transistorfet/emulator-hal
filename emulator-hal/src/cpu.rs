
use core::fmt;

use crate::bus::{BusType, BusAccess};

/// Represents a device that can change state with the passage of a clock signal
pub trait Step<Bus>
where
    Bus: BusAccess,
{
    /// A type that is return if the step cannot be performed
    ///
    /// Note: this is not the same as BusType::Error.  It is up to the implementor to impose a constraint
    ///       if desired, that BusType::Error: Self::Error or BusType::Error: Into<Self::Error>
    type Error;

    /// Returns true if this device is still running.  This can be used to detect a stop condition
    fn is_running(&mut self) -> bool;

    /// Reset the device to its initial state, as if the device's reset signal was asserted
    fn reset(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<(), Self::Error>;

    /// Step the execute by one unit of time, and return the time at which this device's step should be called again
    ///
    /// The given `Instant` is the time at which this step occurs, and the returned `Instant` is the time of the
    /// next step should occur, according to the device itself.  The given bus can be used to access the system
    fn step(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<Bus::Instant, Self::Error>;

    // TODO should this be in the Debug trait instead, where the Debug trait will mainly be implemented by CPUs
    /// Optional method to set the address used by the next call to `step`
    ///
    /// This is specifically for CPU implementations
    fn set_start_address(&mut self, _address: Bus::Address) -> Result<(), Self::Error> {
        Ok(())
    }
}

// TODO should this depend on Step, which is the most common way it will be used, even though it technically could
// be used for a device that just has a bus interface with no clock
/// Inspect the state of a device, and emit it to an object that implements `fmt::Write`
pub trait Inspect<W, Bus>
where
    W: fmt::Write,
    Bus: BusAccess,
{
    /// A type that describes the types of information or state that this device can emit
    type InfoType;

    /// A type that is returned if the data cannot be written as expected
    type Error;

    /// Write the given information type to the given writer, or return an error
    fn inspect(&mut self, writer: &mut W, info: Self::InfoType, bus: &mut Bus) -> Result<(), Self::Error>;
}

/// Control the execution of a CPU device for debugging purposes
pub trait Debug<W, Bus>: Inspect<W, Bus> + Step<Bus>
where
    W: fmt::Write,
    Bus: BusAccess,
{
    type Error;

    fn run_command(&mut self, bus: &mut Bus, args: &[&str]) -> Result<bool, <Self as Debug<W, Bus>>::Error>;

    fn get_current_address(&mut self) -> Result<Bus::Address, <Self as Debug<W, Bus>>::Error>;
    fn set_start_address(&mut self, address: Bus::Address) -> Result<(), <Self as Debug<W, Bus>>::Error>;
}



#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Instant, Duration};

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
        fn read(&mut self, _now: Instant, addr: u64, data: &mut [u8]) -> Result<usize, Self::Error> {
            let addr = addr as usize;
            data.copy_from_slice(&self.0[addr..addr + data.len()]);
            Ok(data.len())
        }

        fn write(&mut self, _now: Instant, addr: u64, data: &[u8]) -> Result<usize, Self::Error> {
            let addr = addr as usize;
            self.0[addr..addr + data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    #[derive(Default)]
    struct Cpu {
        pc: u64,
        sum: u32,
        running: bool,
    }

    impl<Bus> Step<Bus> for Cpu
    where
        Bus: BusAccess<Address = u64, Error = Error, Instant = Instant>
    {
        type Error = Bus::Error;

        fn is_running(&mut self) -> bool {
            self.running
        }

        fn reset(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<(), Self::Error> {
            self.running = true;
            self.pc = bus.read_beu32(now, 0x0000)? as u64;
            Ok(())
        }

        fn step(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<Bus::Instant, Self::Error> {
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
        let mut memory = Memory(vec![0; 1024]);

        let mut cpu = Cpu::default();

        let location = 0x100;
        memory.write_beu32(Instant::now(), 0x0000, location as u32).unwrap();

        for i in 0..100 {
            memory.write_beu32(Instant::now(), location + 4 * i as u64, 1 + i as u32).unwrap();
        }

        fn run_static_test<B, C, E>(bus: &mut B, cpu: &mut C) -> Result<(), E>
        where
            B: BusAccess<Error = E, Instant = Instant>,
            C: Step<B, Error = E>,
        {
            cpu.reset(Instant::now(), bus)?;

            while cpu.is_running() {
                cpu.step(Instant::now(), bus)?;
            }
            Ok(())
        }

        run_static_test(&mut memory, &mut cpu).unwrap();

        assert_eq!(cpu.sum, 5050);
    }

    #[test]
    fn test_dynamic_system() {
        let mut memory = Memory(vec![0; 1024]);

        let mut cpu = Cpu::default();

        let location = 0x100 as u64;
        memory.write_beu32(Instant::now(), 0x0000, location as u32).unwrap();

        for i in 0..100 {
            memory.write_beu32(Instant::now(), location + 4 * i as u64, 1 + i as u32).unwrap();
        }

        type Bus = Box<dyn BusAccess<Address = u64, Error = Error, Instant = Instant>>;

        //let _trait_obj_cpu: &mut dyn Step<Bus, Error = Error> = &mut cpu;

        fn run_dynamic_test(mut bus: Bus, cpu: &mut dyn Step<Bus, Error = Error>) -> Result<(), Error> {
            cpu.reset(Instant::now(), &mut bus)?;

            while cpu.is_running() {
                cpu.step(Instant::now(), &mut bus)?;
            }
            Ok(())
        }

        run_dynamic_test(Box::new(memory), &mut cpu).unwrap();

        assert_eq!(cpu.sum, 5050);
    }
}
