use std::cmp;
use anyhow::{anyhow, Result};
use crate::system::{byte, mapper};
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::ppu::character_rom::CharacterROM;

#[derive(Eq, PartialEq)]
pub enum ROMFormat
{
    iNES, //standard NES ROM file format
    NSF, //NES sound file format
    Unknown,
}

impl ROMFormat
{
    pub fn detect(bytes : &[byte]) -> ROMFormat
    {
        return if bytes.len()>=4 && bytes[0]==0x4E && bytes[1]==0x45 && bytes[2]==0x53 && bytes[3]==0x1A { ROMFormat::iNES }
        else if bytes.len()>=5 && bytes[0]==0x4E && bytes[1]==0x45 && bytes[2]==0x53 && bytes[3]==0x4D && bytes[4]==0x1A { ROMFormat::NSF }
        else { ROMFormat::Unknown };
    }
}

pub struct ParsedROM
{
    pub format : ROMFormat,
    pub mapper : mapper,
    pub program_rom : ProgramROM,
    pub character_rom : CharacterROM,
}

pub struct ROMParser {}

impl ROMParser
{
    pub fn parse(bytes : Box<[byte]>) -> Result<ParsedROM>
    {
        return match ROMFormat::detect(&bytes)
        {
            ROMFormat::iNES => { ROMParser::parse_ines(bytes) }
            ROMFormat::NSF => { ROMParser::parse_nsf(bytes) }
            ROMFormat::Unknown => { Err(anyhow!("ROM file cannot be parsed!")) }
        };
    }

    fn parse_ines(bytes : Box<[byte]>) -> Result<ParsedROM>
    {
        let format = ROMFormat::iNES;
        let header = &bytes[0x00..0x10];
        let mapper = header[6] as mapper;

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

        return Ok(ParsedROM { format, mapper, program_rom, character_rom });
    }

    fn parse_nsf(bytes : Box<[byte]>) -> Result<ParsedROM>
    {
        let format = ROMFormat::NSF;
        let mapper = 0;
        let program_rom_bytes = &bytes[0x080..bytes.len()];
        let program_rom = ProgramROM::new(mapper, program_rom_bytes);
        let character_rom = CharacterROM::new(mapper, &[]);

        return Ok(ParsedROM { format, mapper, program_rom, character_rom });
    }
}
