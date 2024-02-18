use crate::system::{address, byte};

pub struct VRAM
{
    bytes : Box<[byte]>
}

impl VRAM
{
    pub fn new() -> VRAM
    {
        return VRAM { bytes: Box::new([0; 16*1024]) }; //16kB
    }

    pub fn get(self : &VRAM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }

    pub fn put(self : &mut VRAM, raw_address : address, value : byte)
    {
        let address = (raw_address as usize) % self.bytes.len();
        self.bytes[address] = value;
    }
}
