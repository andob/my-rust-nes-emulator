use crate::system::{address, byte};
use crate::system::rom::ROM;

pub struct CharacterROM
{
    mapper : byte,
    bytes : Box<[byte]>,
}

impl CharacterROM
{
    pub fn empty() -> CharacterROM
    {
        return CharacterROM { mapper:0, bytes: Box::new([]) }
    }

    pub fn new(mapper : byte, bytes : &[byte]) -> CharacterROM
    {
        return CharacterROM { mapper:mapper, bytes: bytes.to_owned().into_boxed_slice() };
    }
}

impl ROM for CharacterROM
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
