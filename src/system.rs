use anyhow::Result;
use crate::system::cpu::CPU;
use crate::system::cpu_bus::CPUBus;
use crate::system::debugger::{Debugger, EmptyDebugger};
use crate::system::ram::RAM;
use crate::system::rom::{ROM, ROMParser};
use crate::system::test::Test;

mod cpu;
mod ram;
mod rom;
mod debugger;
mod test;
mod cpu_bus;
mod logger;

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type address = u16;

#[allow(non_camel_case_types)]
pub type mapper = u8;

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
    pub fn with_rom_bytes(rom_data : Box<[byte]>) -> Result<System>
    {
        let (program_rom, character_rom) = ROMParser::parse(rom_data)?;
        return Ok(System::with_parsed_rom(Box::new(program_rom), Box::new(character_rom)));
    }

    pub fn with_parsed_rom(program_rom : Box<dyn ROM>, _character_rom : Box<dyn ROM>) -> System
    {
        return System
        {
            cpu: CPU::new(),
            cpu_bus: CPUBus
            {
                ram: RAM::new(),
                program_rom: program_rom,
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
