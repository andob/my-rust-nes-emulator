use crate::system::{address, byte, mapper};

pub const DONKEY_KONG_CHARACTER_ROM_HASH : &str = "c5a3bb0d1eb21b1a5bec887af7f42ad5";
pub const PINBALL_CHARACTER_ROM_HASH : &str = "ef5d81145c203594564482ca6c301bf2";
pub const SMB1_CHARACTER_ROM_HASH : &str = "7bbce748f81502207b5a3b87e4d3e856";

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
            let address = (raw_address as usize) % self.bytes.len();
            self.bytes[address] = value;
        }
    }

    pub fn hash(&self) -> String
    {
        return format!("{:x}", md5::compute(&*self.bytes));
    }
}
