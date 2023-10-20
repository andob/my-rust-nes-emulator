use crate::system::{address, byte};

pub struct ROM
{
    program : Box<[byte]>,
    _character : Box<[byte]>,
    mapper : byte,
}

impl ROM
{
    pub fn new(bytes : Box<[byte]>) -> ROM
    {
        let header = &bytes[0x00..0x10];
        if header[0]==0x4E && header[1]==0x45 && header[2]==0x53 && header[3]==0x1A
        {
            let offset_of_program_rom = header.len();
            let size_of_program_rom = (header[4] as usize)*16*1024;
            let program_rom = &bytes[offset_of_program_rom..offset_of_program_rom+size_of_program_rom];

            let offset_of_character_rom = offset_of_program_rom+size_of_program_rom;
            let size_of_character_rom = (header[5] as usize)*8*1024;
            let character_rom = &bytes[offset_of_character_rom..offset_of_character_rom+size_of_character_rom];

            return ROM
            {
                program: program_rom.to_owned().into_boxed_slice(),
                _character: character_rom.to_owned().into_boxed_slice(),
                mapper: header[6],
            };
        }

        return ROM
        {
            program: bytes,
            _character: Box::new([]),
            mapper: 0,
        };
    }

    pub fn get(self : &ROM, raw_address : address) -> byte
    {
        if self.mapper==0
        {
            let address = (raw_address as usize) % self.program.len();
            return self.program[address];
        }

        return 0;
    }
}
