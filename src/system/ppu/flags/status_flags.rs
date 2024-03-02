use crate::system::byte;

pub struct PPUStatusFlags
{
    pub has_vblank_started : bool,
    pub is_sprite_zero_hit : bool, //todo use this
    pub has_sprite_overflow : bool, //todo use this
}

impl PPUStatusFlags
{
    pub fn new() -> PPUStatusFlags
    {
        return PPUStatusFlags
        {
            has_vblank_started: false,
            is_sprite_zero_hit: false,
            has_sprite_overflow: false,
        }
    }

    pub fn to_byte(self : &PPUStatusFlags) -> byte
    {
        return ((self.has_vblank_started  as byte) << 7)
             | ((self.is_sprite_zero_hit  as byte) << 6)
             | ((self.has_sprite_overflow as byte) << 5);
    }

    pub fn from_byte(value : byte) -> PPUStatusFlags
    {
        return PPUStatusFlags
        {
            has_vblank_started:  (value & 0b10000000) >> 7 == 1,
            is_sprite_zero_hit:  (value & 0b01000000) >> 6 == 1,
            has_sprite_overflow: (value & 0b00100000) >> 5 == 1,
        }
    }
}

impl Clone for PPUStatusFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return PPUStatusFlags::from_byte(byte);
    }
}

impl Eq for PPUStatusFlags {}
impl PartialEq<PPUStatusFlags> for PPUStatusFlags
{
    fn eq(&self, other : &PPUStatusFlags) -> bool
    {
        return self.has_vblank_started == other.has_vblank_started &&
            self.is_sprite_zero_hit == other.is_sprite_zero_hit &&
            self.has_sprite_overflow == other.has_sprite_overflow;
    }
}
