use crate::system::{address, byte};

#[derive(Eq, PartialEq)]
pub struct PPUControlFlags
{
    pub is_nmi_enabled : bool, //todo use this
    pub should_output_color_on_ext_pins : bool, //todo use this
    pub should_use_16pixel_high_sprites : bool,
    pub base_pattern_table_address_for_background : address, //todo use this
    pub base_pattern_table_address_for_foreground : address, //todo use this
    pub vram_address_increment_amount : byte, //todo use this
    pub base_nametable_address : address, //todo use this
}

impl PPUControlFlags
{
    pub fn new() -> PPUControlFlags
    {
        return PPUControlFlags
        {
            is_nmi_enabled: false,
            should_output_color_on_ext_pins: false,
            should_use_16pixel_high_sprites: false,
            base_pattern_table_address_for_background: 0,
            base_pattern_table_address_for_foreground: 0,
            vram_address_increment_amount: 1,
            base_nametable_address: 0x2000,
        }
    }

    pub fn to_byte(self : &PPUControlFlags) -> byte
    {
        let encoded_base_nametable_address_as_two_bits : (byte, byte) =
            match self.base_nametable_address { 0x2400 => (0,1), 0x2800 => (1,0), 0x2C00 => (1,1), _ => (0,0) };

        return ((self.is_nmi_enabled                                        as byte) << 7)
             | ((self.should_output_color_on_ext_pins                       as byte) << 6)
             | ((self.should_use_16pixel_high_sprites                       as byte) << 5)
             | (((self.base_pattern_table_address_for_background == 0x1000) as byte) << 4)
             | (((self.base_pattern_table_address_for_foreground == 0x1000) as byte) << 3)
             | (((self.vram_address_increment_amount==32)                   as byte) << 2)
             | ((encoded_base_nametable_address_as_two_bits.0               as byte) << 1)
             | ((encoded_base_nametable_address_as_two_bits.1               as byte) << 0);
    }

    pub fn from_byte(value : byte) -> PPUControlFlags
    {
        return PPUControlFlags
        {
            is_nmi_enabled:                               (value & 0b10000000) >> 7 == 1,
            should_output_color_on_ext_pins:              (value & 0b01000000) >> 6 == 1,
            should_use_16pixel_high_sprites:              (value & 0b00100000) >> 5 == 1,
            base_pattern_table_address_for_background: if (value & 0b00010000) >> 4 == 1 {0x1000} else {0},
            base_pattern_table_address_for_foreground: if (value & 0b00001000) >> 3 == 1 {0x1000} else {0},
            vram_address_increment_amount:             if (value & 0b00000100) >> 2 == 1 {32} else {1},
            base_nametable_address:                 match (value & 0b00000011) >> 0 { 1 => 0x2400, 2 => 0x2800, 3 => 0x2C00, _ => 0x2000 },
        }
    }
}

impl Clone for PPUControlFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return PPUControlFlags::from_byte(byte);
    }
}
