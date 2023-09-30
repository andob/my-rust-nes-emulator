use crate::system::{address, byte};

pub struct ROM
{
    bytes : Vec<byte>
}

impl ROM
{
    pub fn new(bytes : Vec<byte>) -> ROM
    {
        return ROM { bytes };
    }

    pub fn get(self : &ROM, index : address) -> Option<byte>
    {
        return self.bytes.get(index as usize).cloned();
    }
}
