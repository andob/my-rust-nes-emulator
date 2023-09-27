use crate::system::cpu::CPU;
use crate::system::ram::RAM;

mod cpu;
mod ram;

pub struct System
{
    cpu : CPU,
    ram : RAM,
    rom : Vec<u8>,
}

impl System
{
    pub fn new() -> System
    {
        return System
        {
            cpu: CPU::new(),
            ram: RAM::new(),
            rom: Vec::new(),
        };
    }

    pub fn run(&mut self)
    {
        CPU::run(self);
    }
}
