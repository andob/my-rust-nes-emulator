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
        let bytes = if !bytes.is_empty()
            { bytes.to_owned().into_boxed_slice() } //CHR-ROM
        else { Box::new([0; 8*1024]) }; //8kB CHR-RAM

        return CharacterROM { mapper, bytes };
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

        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }
}
