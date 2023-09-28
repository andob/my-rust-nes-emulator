use crate::type_alias::{byte, word};

pub struct RAM
{
    bytes : Vec<byte>
}

impl RAM
{
    pub fn new() -> RAM
    {
        let ram_size = 2*1024; //2KB
        return RAM { bytes: vec![0; ram_size] };
    }

    pub fn get(self : &RAM, index : word) -> Option<&byte>
    {
        return self.bytes.get(index as usize);
    }

    pub fn put(self : &mut RAM, address : word, value : byte)
    {
        if (address as usize) < self.bytes.len()
        {
            self.bytes[address as usize] = value;
        }
    }
}
