use crate::system::{address, byte};

pub struct ROM
{
    bytes : Box<[byte]>
}

impl ROM
{
    pub fn new(bytes : Box<[byte]>) -> ROM
    {
        return ROM { bytes };
    }

    pub fn get(self : &ROM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }
}
