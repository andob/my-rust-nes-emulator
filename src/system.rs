use crate::system::cpu::CPU;
use crate::system::ram::RAM;
use crate::system::rom::ROM;

mod cpu;
mod ram;
mod rom;

pub struct System
{
    cpu : CPU,
    ram : RAM,
    rom : ROM,
}

impl System
{
    pub fn new() -> System
    {
        return System
        {
            cpu: CPU::new(),
            ram: RAM::new(),
            rom: ROM::new(Vec::new()),
        };
    }

    pub fn run(&mut self)
    {
        CPU::run(self);
    }
}
