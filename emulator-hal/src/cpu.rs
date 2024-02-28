
use core::fmt;

use crate::bus::{BusType, BusAccess};

/// Represents a device that can change state with the passage of a clock signal
pub trait Step<Bus>
where
    Bus: BusAccess,
{
    type Error;

    fn is_running(&mut self) -> bool;

    fn reset(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<(), Self::Error>;

    fn step(&mut self, now: Bus::Instant, bus: &mut Bus) -> Result<Bus::Instant, Self::Error>;

    // TODO should this be in the Debug trait instead, where the Debug trait will mainly be implemented by CPUs
    /// Optional method to set the address used by the next call to `step`
    ///
    /// This is specifically for CPU implementations
    fn set_start_address(&mut self, address: Bus::Address) -> Result<(), Self::Error> {
        Ok(())
    }
}

/*
/// A device (cpu) that can debugged using the built-in debugger
pub trait Debug<Bus>
where
    Bus: BusAccess
{
    type Error;

    // TODO these can be implemented by the caller using get/set address
    //fn add_breakpoint(&mut self, addr: Bus::Address);
    //fn remove_breakpoint(&mut self, addr: Bus::Address);

    // TODO these can be implemented using Inspect
    //fn print_current_step(&mut self, bus: Bus) -> Result<(), Self::Error>;
    //fn print_disassembly(&mut self, addr: Bus::Address, count: usize);

    fn run_command(&mut self, bus: Bus, args: &[&str]) -> Result<bool, Self::Error>;

    fn get_current_address(&mut self) -> Result<Bus::Address, Self::Error>;
    fn set_start_address(&mut self, address: Bus::Address) -> Result<(), Self::Error>;
}
*/

/// Inspect the state of a device, and emit it to an object that implements `fmt::Write`
pub trait Inspect<W>
where
    W: fmt::Write,
{
    type InfoType;
    type Error;

    fn inspect(&mut self, info: Self::InfoType, writer: &mut W) -> Result<(), Self::Error>;
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
