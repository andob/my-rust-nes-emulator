use std::cmp;
use anyhow::{anyhow, Result};
use crate::system::byte;
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::rom::Mapper::{_0, _1, _2, _3, _4, _5};

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Mapper
{
    _0, _1, _2, _3, _4, _5,
    SnakeTestGame,
    Unknown
}

impl From<byte> for Mapper
{
    fn from(value : byte) -> Self
    {
        return match value
        {
            0=>_0, 1=>_1, 2=>_2, 3=>_3, 4=>_4, 5=>_5,
            _ => Mapper::Unknown
        };
    }
}

pub struct ROMParser {}

impl ROMParser
{
    pub fn parse(bytes : Box<[byte]>) -> Result<(ProgramROM, CharacterROM)>
    {
        let header = &bytes[0x00..0x10];
        if header[0]==0x4E && header[1]==0x45 && header[2]==0x53 && header[3]==0x1A
        {
            let mapper = Mapper::from(header[6]);

            let offset_of_program_rom = header.len();
            let size_of_program_rom = (header[4] as usize)*16*1024;
            let program_rom_start_index = cmp::min(bytes.len(), offset_of_program_rom);
            let program_rom_end_index = cmp::min(bytes.len(), offset_of_program_rom+size_of_program_rom);
            let program_rom_bytes = &bytes[program_rom_start_index..program_rom_end_index];
            let program_rom = ProgramROM::new(mapper, program_rom_bytes);

            let offset_of_character_rom = offset_of_program_rom+size_of_program_rom;
            let size_of_character_rom = (header[5] as usize)*8*1024;
            let character_rom_start_index = cmp::min(bytes.len(), offset_of_character_rom);
            let character_rom_end_index = cmp::min(bytes.len(), offset_of_character_rom+size_of_character_rom);
            let character_rom_bytes = &bytes[character_rom_start_index..character_rom_end_index];
            let character_rom = CharacterROM::new(mapper, character_rom_bytes);

            return Ok((program_rom, character_rom));
        }

        return Err(anyhow!("ROM file cannot be parsed!"));
    }
}
