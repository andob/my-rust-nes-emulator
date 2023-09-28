use crate::type_alias::{byte, word};

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

    pub fn get(self : &ROM, index : word) -> Option<&byte>
    {
        return self.bytes.get(index as usize);
    }
}
