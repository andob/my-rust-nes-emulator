use crate::system::{address, byte};

pub const RAM_SIZE : usize = 2*1024; //2KB
pub const RAM_PAGE_SIZE : address = 256; //one page = 256 bytes

pub struct RAM
{
    bytes : Box<[byte; RAM_SIZE]>
}

impl RAM
{
    pub fn new() -> RAM
    {
        return RAM { bytes: Box::new([0; RAM_SIZE]) };
    }

    pub fn get(self : &RAM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % RAM_SIZE;
        return self.bytes[address];
    }

    pub fn put(self : &mut RAM, raw_address : address, value : byte)
    {
        let address = (raw_address as usize) % RAM_SIZE;
        self.bytes[address] = value;
    }
}
