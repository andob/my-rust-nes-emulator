use crate::system::{address, byte};
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::palette::Palette;
use crate::system::ppu::vram::VRAM;

pub const PATTERN_TABLE0_START_ADDRESS : address = 0x0000;
pub const PATTERN_TABLE0_END_ADDRESS : address = 0x0FFF;
pub const PATTERN_TABLE1_START_ADDRESS : address = 0x1000;
pub const PATTERN_TABLE1_END_ADDRESS : address = 0x1FFF;
const NAMETABLE0_START_ADDRESS : address = 0x2000;
const NAMETABLE0_END_ADDRESS : address = 0x23FF;
const NAMETABLE1_START_ADDRESS : address = 0x2400;
const NAMETABLE1_END_ADDRESS : address = 0x27FF;
const NAMETABLE2_START_ADDRESS : address = 0x2800;
const NAMETABLE2_END_ADDRESS : address = 0x2BFF;
const NAMETABLE3_START_ADDRESS : address = 0x2C00;
const NAMETABLE3_END_ADDRESS : address = 0x2FFF;
const NAMETABLE_MIRROR_START_ADDRESS : address = 0x3000;
const NAMETABLE_MIRROR_END_ADDRESS : address = 0x3EFF;
const PALETTE_START_ADDRESS : address = 0x3F00;
const PALETTE_END_ADDRESS : address = 0x3FFF;

pub struct PPUBus
{
    pub vram : VRAM,
    pub character_rom : CharacterROM,
    pub palette : Palette,
}

impl PPUBus
{
    pub fn new(character_rom : CharacterROM) -> PPUBus
    {
        return PPUBus
        {
            vram: VRAM::new(),
            character_rom: character_rom,
            palette: Palette::new(),
        };
    }

    pub fn get(self : &PPUBus, raw_address : address) -> byte
    {
        if raw_address >= PATTERN_TABLE0_START_ADDRESS && raw_address <= PATTERN_TABLE1_END_ADDRESS
        {
            return self.character_rom.get(raw_address);
        }

        if raw_address >= NAMETABLE0_START_ADDRESS && raw_address <= NAMETABLE3_END_ADDRESS
        {
            return self.vram.get(raw_address-NAMETABLE0_START_ADDRESS);
        }

        if raw_address >= NAMETABLE_MIRROR_START_ADDRESS && raw_address <= NAMETABLE_MIRROR_END_ADDRESS
        {
            let max_address = NAMETABLE3_END_ADDRESS-NAMETABLE0_START_ADDRESS+1;
            return self.vram.get((raw_address-NAMETABLE_MIRROR_START_ADDRESS)%max_address);
        }

        if raw_address >= PALETTE_START_ADDRESS && raw_address <= PALETTE_END_ADDRESS
        {
            self.palette.get_index(raw_address-PALETTE_START_ADDRESS);
        }

        return 0;
    }

    pub fn put(self : &mut PPUBus, raw_address : address, value : byte)
    {
        if raw_address >= PATTERN_TABLE0_START_ADDRESS && raw_address <= PATTERN_TABLE1_END_ADDRESS
        {
            self.character_rom.put(raw_address, value);
        }

        if raw_address >= NAMETABLE0_START_ADDRESS && raw_address <= NAMETABLE3_END_ADDRESS
        {
            self.vram.put(raw_address-NAMETABLE0_START_ADDRESS, value);
        }

        if raw_address >= NAMETABLE_MIRROR_START_ADDRESS && raw_address <= NAMETABLE_MIRROR_END_ADDRESS
        {
            let max_address = NAMETABLE3_END_ADDRESS-NAMETABLE0_START_ADDRESS+1;
            self.vram.put((raw_address-NAMETABLE_MIRROR_START_ADDRESS)%max_address, value);
        }

        if raw_address >= PALETTE_START_ADDRESS && raw_address <= PALETTE_END_ADDRESS
        {
            self.palette.put_index(raw_address-PALETTE_START_ADDRESS, value);
        }
    }
}