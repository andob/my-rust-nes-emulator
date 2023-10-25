mod program_rom;
mod character_rom;

use crate::system::{address, byte};
use crate::system::rom::character_rom::CharacterROM;
use crate::system::rom::program_rom::ProgramROM;

pub trait ROM
{
    fn get(&self, raw_address : address) -> byte;
}

pub struct ROMParser {}

impl ROMParser
{
    pub fn parse(bytes : Box<[byte]>) -> (ProgramROM, CharacterROM)
    {
        let header = &bytes[0x00..0x10];
        if header[0]==0x4E && header[1]==0x45 && header[2]==0x53 && header[3]==0x1A
        {
            let mapper = header[6];

            let offset_of_program_rom = header.len();
            let size_of_program_rom = (header[4] as usize)*16*1024;
            let program_rom_bytes = &bytes[offset_of_program_rom..offset_of_program_rom+size_of_program_rom];
            let program_rom = ProgramROM::new(mapper, program_rom_bytes);

            let offset_of_character_rom = offset_of_program_rom+size_of_program_rom;
            let size_of_character_rom = (header[5] as usize)*8*1024;
            let character_rom_bytes = &bytes[offset_of_character_rom..offset_of_character_rom+size_of_character_rom];
            let character_rom = CharacterROM::new(mapper, character_rom_bytes);

            return (program_rom, character_rom);
        }

        return (ProgramROM::new(0, &*bytes), CharacterROM::empty());
    }
}
