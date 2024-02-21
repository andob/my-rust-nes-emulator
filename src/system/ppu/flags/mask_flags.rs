use crate::system::byte;

#[derive(PartialEq, Eq)]
pub struct PPUMaskFlags
{
    pub should_emphasize_blue : bool, //todo use this
    pub should_emphasize_green : bool, //todo use this
    pub should_emphasize_red : bool, //todo use this
    pub should_show_sprites : bool, //todo use this
    pub should_render_background: bool, //todo use this
    pub should_show_sprites_in_leftmost_part_of_screen: bool, //todo use this
    pub should_show_background_in_leftmost_part_of_screen: bool, //todo use this
    pub should_render_as_grayscale : bool, //todo use this
}

impl PPUMaskFlags
{
    pub fn new() -> PPUMaskFlags
    {
        return PPUMaskFlags
        {
            should_emphasize_blue: false,
            should_emphasize_green: false,
            should_emphasize_red: false,
            should_show_sprites: false,
            should_render_background: false,
            should_show_sprites_in_leftmost_part_of_screen: false,
            should_show_background_in_leftmost_part_of_screen: false,
            should_render_as_grayscale: false,
        }
    }

    pub fn to_byte(self : &PPUMaskFlags) -> byte
    {
        return ((self.should_emphasize_blue                             as byte) << 7)
             | ((self.should_emphasize_green                            as byte) << 6)
             | ((self.should_emphasize_red                              as byte) << 5)
             | ((self.should_show_sprites                               as byte) << 4)
             | ((self.should_render_background as byte) << 3)
             | ((self.should_show_sprites_in_leftmost_part_of_screen    as byte) << 2)
             | ((self.should_show_background_in_leftmost_part_of_screen as byte) << 1)
             | ((self.should_render_as_grayscale                        as byte) << 0);
    }

    pub fn from_byte(value : byte) -> PPUMaskFlags
    {
        return PPUMaskFlags
        {
            should_emphasize_blue:                             (value & 0b10000000) >> 7 == 1,
            should_emphasize_green:                            (value & 0b01000000) >> 6 == 1,
            should_emphasize_red:                              (value & 0b00100000) >> 5 == 1,
            should_show_sprites:                               (value & 0b00010000) >> 4 == 1,
            should_render_background:                            (value & 0b00001000) >> 3 == 1,
            should_show_sprites_in_leftmost_part_of_screen:    (value & 0b00000100) >> 2 == 1,
            should_show_background_in_leftmost_part_of_screen: (value & 0b00000010) >> 1 == 1,
            should_render_as_grayscale:                        (value & 0b00000001) >> 0 == 1,
        }
    }
}

impl Clone for PPUMaskFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return PPUMaskFlags::from_byte(byte);
    }
}
