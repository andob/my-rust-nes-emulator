use crate::system::{address, byte};

pub struct RAM
{
    bytes : Box<[byte]>
}

impl RAM
{
    pub fn new() -> RAM
    {
        return RAM { bytes: Box::new([0; 2*1024]) }; //2kB
    }

    pub fn get(self : &RAM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }

    pub fn put(self : &mut RAM, raw_address : address, value : byte)
    {
        let address = (raw_address as usize) % self.bytes.len();
        self.bytes[address] = value;
    }
}
