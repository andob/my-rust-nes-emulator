use crate::system::{address, byte, mapper};

pub struct CharacterROM
{
    mapper : mapper,
    bytes : Box<[byte]>,
}

impl CharacterROM
{
    pub fn new(mapper : mapper, bytes : &[byte]) -> CharacterROM
    {
        return CharacterROM { mapper:mapper, bytes: bytes.to_owned().into_boxed_slice() };
    }

    pub fn get(&self, raw_address : address) -> byte
    {
        if self.mapper==0
        {
            let address = (raw_address as usize) % self.bytes.len();
            return self.bytes[address];
        }

        if self.mapper==1
        {
            let address = (raw_address as usize) % self.bytes.len();
            return self.bytes[address];
        }

        return 0;
    }
}
