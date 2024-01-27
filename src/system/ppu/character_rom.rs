use crate::system::{address, byte, mapper};

pub struct CharacterROM
{
    mapper : mapper,
    is_writeable : bool,
    bytes : Box<[byte]>,
}

impl CharacterROM
{
    pub fn new(mapper : mapper, bytes : &[byte]) -> CharacterROM
    {
        let should_use_character_ram = bytes.is_empty();
        let bytes = if !should_use_character_ram
            { bytes.to_owned().into_boxed_slice() } //CHR-ROM
        else { Box::new([0; 8*1024]) }; //8kB CHR-RAM

        return CharacterROM { mapper, is_writeable:should_use_character_ram, bytes };
    }

    pub fn get(&self, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }

    pub fn put(&mut self, raw_address : address, value : byte)
    {
        if self.is_writeable
        {
            self.bytes[raw_address as usize] = value;
        }
    }
}
