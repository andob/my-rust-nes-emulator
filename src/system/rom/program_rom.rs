use crate::system::{address, byte};
use crate::system::rom::ROM;

pub struct ProgramROM
{
    mapper : byte,
    bytes : Box<[byte]>
}

impl ProgramROM
{
    pub fn new(mapper : byte, bytes : &[byte]) -> ProgramROM
    {
        return ProgramROM { mapper:mapper, bytes: bytes.to_owned().into_boxed_slice() };
    }
}

impl ROM for ProgramROM
{
    fn get(&self, raw_address : address) -> byte
    {
        if self.mapper==0
        {
            let address = (raw_address as usize) % self.bytes.len();
            return self.bytes[address];
        }

        return 0;
    }
}
