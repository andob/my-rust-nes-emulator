use crate::system::{address, byte};

const RAM_SIZE : usize = 2*1024; //2KB

pub struct RAM
{
    bytes : Vec<byte>
}

impl RAM
{
    pub fn new() -> RAM
    {
        return RAM { bytes: vec![0; RAM_SIZE] };
    }

    pub fn get(self : &RAM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % RAM_SIZE;
        return self.bytes.get(address).cloned().unwrap_or_default();
    }

    pub fn put(self : &mut RAM, raw_address : address, value : byte)
    {
        let address = (raw_address as usize) % RAM_SIZE;
        self.bytes[address] = value;
    }
}
