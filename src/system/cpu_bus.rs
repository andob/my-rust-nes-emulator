use crate::system::{address, byte};
use crate::system::ram::RAM;
use crate::system::rom::ROM;

pub struct CPUBus
{
    pub ram : RAM,
    pub program_rom : Box<dyn ROM>,
}

impl CPUBus
{
    pub fn get(self : &CPUBus, raw_address : address) -> byte
    {
        let mut offset = 0 as address;
        if raw_address >= offset && raw_address <= 0x1FFF
        {
            return self.ram.get(raw_address);
        }

        offset = 0x2000;
        if raw_address >= offset && raw_address <= 0x3FFF
        {
            //todo NES PPU registers
            return 0;
        }

        offset = 0x4000;
        if raw_address >= offset && raw_address <= 0x4017
        {
            //todo NES APU and IO registers
            return 0;
        }

        offset = 0x4020;
        if raw_address >= offset && raw_address < 0xFFFF
        {
            return self.program_rom.get(raw_address-offset);
        }

        return 0;
    }

    pub fn put(self : &mut CPUBus, raw_address : address, value : byte)
    {
        //todo should also map on write, right?
        return self.ram.put(raw_address, value);
    }
}
