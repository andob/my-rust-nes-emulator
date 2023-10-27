use crate::system::cpu::CPU;
use crate::system::cpu_bus::CPUBus;
use crate::system::debugger::{Debugger, EmptyDebugger};
use crate::system::ram::RAM;
use crate::system::rom::ROMParser;
use crate::system::test::Test;

mod cpu;
mod ram;
mod rom;
mod debugger;
mod test;
mod cpu_bus;

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type address = u16;

#[macro_export]
macro_rules! address_from_high_low
{
    ($high : expr, $low : expr) => ((($high as address)<<8)|($low as address))
}

pub struct System
{
    cpu : CPU,
    cpu_bus : CPUBus,
}

impl System
{
    pub fn new(rom_data : Box<[byte]>) -> System
    {
        let (program_rom, _character_rom) = ROMParser::parse(rom_data);

        return System
        {
            cpu: CPU::new(),
            cpu_bus: CPUBus
            {
                ram: RAM::new(),
                program_rom: Box::new(program_rom),
            }
        };
    }

    pub fn test() -> Test { Test{} }

    pub fn run(&mut self)
    {
        let debugger = EmptyDebugger::new();
        self.run_with_debugger(Box::new(debugger));
    }

    pub fn run_with_debugger(&mut self, debugger : Box<dyn Debugger>)
    {
        CPU::run(self, debugger);
    }
}