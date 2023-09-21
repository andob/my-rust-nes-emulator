use crate::system::cpu::CPU;

mod cpu;

pub struct System
{
    cpu : CPU,
    ram : Vec<u8>,
    rom : Vec<u8>,
}

impl System
{
    pub fn new() -> System
    {
        return System
        {
            cpu: CPU::new(),
            ram: vec![],
            rom: vec![ 0x69u8, 0xFAu8 ],
        };
    }

    pub fn run(&mut self)
    {
        CPU::run(self);
    }
}
