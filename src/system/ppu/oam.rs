use crate::system::{address, byte};

pub struct PPUOAM
{
    bytes : Box<[byte]>
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sprite
{
    pub index : byte,
    pub x : byte,
    pub y : byte,
    pub should_use_right_pattern_table : bool,
    pub pattern_table_index : address,
    pub palette_index : byte,
    pub should_flip_horizontally : bool,
    pub should_flip_vertically : bool,
}

impl PPUOAM
{
    pub fn new() -> PPUOAM
    {
        return PPUOAM { bytes: Box::new([0; 256]) };
    }

    pub fn get(self : &PPUOAM, raw_address : address) -> byte
    {
        let address = (raw_address as usize) % self.bytes.len();
        return self.bytes[address];
    }

    pub fn put(self : &mut PPUOAM, raw_address : address, value : byte)
    {
        let address = (raw_address as usize) % self.bytes.len();
        self.bytes[address] = value;
    }

    pub fn put_many(self : &mut PPUOAM, start_raw_address : address, values : &Box<[byte]>)
    {
        let mut index = (start_raw_address as usize) % self.bytes.len();
        for value in values.iter()
        {
            if index < self.bytes.len()
            {
                self.bytes[index] = value.clone();
                index += 1;
            }
        }
    }

    pub fn get_8pixel_high_background_sprites(&self) -> Vec<Sprite>
    {
        let flags_filter = |flags : byte| ((flags & 0b00100000) >> 5) == 1;
        let pattern_table_index_parser = |data : byte| data as address;

        return self.get_sprites(flags_filter, pattern_table_index_parser);
    }

    pub fn get_16pixel_high_background_sprites(&self) -> Vec<Sprite>
    {
        let flags_filter = |flags : byte| ((flags & 0b00100000) >> 5) == 1;
        let pattern_table_index_parser = |data : byte| (data & 0b11111110) as address;

        return self.get_sprites(flags_filter, pattern_table_index_parser);
    }

    pub fn get_8pixel_high_foreground_sprites(&self) -> Vec<Sprite>
    {
        let flags_filter = |flags : byte| ((flags & 0b00100000) >> 5) == 0;
        let pattern_table_index_parser = |data : byte| data as address;

        return self.get_sprites(flags_filter, pattern_table_index_parser);
    }

    pub fn get_16pixel_high_foreground_sprites(&self) -> Vec<Sprite>
    {
        let flags_filter = |flags : byte| ((flags & 0b00100000) >> 5) == 0;
        let pattern_table_index_parser = |data : byte| (data & 0b11111110) as address;

        return self.get_sprites(flags_filter, pattern_table_index_parser);
    }

    fn get_sprites(&self, flags_filter : fn(byte) -> bool, pattern_table_index_parser : fn(byte) -> address) -> Vec<Sprite>
    {
        let mut sprites : Vec<Sprite> = Vec::new();

        for index in (0..self.bytes.len()).step_by(4)
        {
            let flags = self.bytes[index+2];
            if flags_filter(flags)
            {
                let pattern_table_data = self.bytes[index+1];
                sprites.push(Sprite
                {
                    index: index as byte,
                    x: self.bytes[index+3],
                    y: self.bytes[index],
                    should_use_right_pattern_table: pattern_table_data & 0b00000001 == 1,
                    pattern_table_index: pattern_table_index_parser(pattern_table_data),
                    palette_index: flags & 0b00000011,
                    should_flip_horizontally: (flags & 0b01000000) >> 6 == 1,
                    should_flip_vertically: (flags & 0b10000000) >> 7 == 1,
                });
            }
        }

        return sprites;
    }
}
