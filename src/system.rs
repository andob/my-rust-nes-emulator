use crate::system::cpu::CPU;
use crate::system::debugger::{Debugger, EmptyDebugger};
use crate::system::ram::RAM;
use crate::system::rom::ROM;
use crate::system::test::Test;

mod cpu;
mod ram;
mod rom;
mod debugger;
mod test;

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type address = u16;

pub struct System
{
    cpu : CPU,
    ram : RAM,
    rom : ROM,
}

impl System
{
    pub fn new(rom_data : Box<[byte]>) -> System
    {
        return System
        {
            cpu: CPU::new(),
            ram: RAM::new(),
            rom: ROM::new(rom_data),
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
