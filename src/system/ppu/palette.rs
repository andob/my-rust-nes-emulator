use crate::system::{address, byte, color};
use rand::random;

const PALETTE_COLORS_SIZE : usize = 64;
const PALETTE_INDICES_SIZE : usize = 0x1F;

pub struct Palette
{
    was_recently_changed : bool,
    colors : [color; PALETTE_COLORS_SIZE],
    indices : [byte; PALETTE_INDICES_SIZE]
}

impl Palette
{
    pub fn new() -> Palette
    {
        let colors =
        [
            0xFF747474, 0xFF24188C, 0xFF0000A8, 0xFF44009C, 0xFF8C0074, 0xFFA80010, 0xFFA40000, 0xFF7C0800,
            0xFF402C00, 0xFF004400, 0xFF005000, 0xFF003C14, 0xFF183C5C, 0xFF000000, 0xFF000000, 0xFF000000,
            0xFFBCBCBC, 0xFF0070EC, 0xFF2038EC, 0xFF8000F0, 0xFFBC00BC, 0xFFE40058, 0xFFD82800, 0xFFC84C0C,
            0xFF887000, 0xFF009400, 0xFF00A800, 0xFF009038, 0xFF008088, 0xFF000000, 0xFF000000, 0xFF000000,
            0xFFFCFCFC, 0xFF3CBCFC, 0xFF5C94FC, 0xFFCC88FC, 0xFFF478FC, 0xFFFC74B4, 0xFFFC7460, 0xFFFC9838,
            0xFFF0BC3C, 0xFF80D010, 0xFF4CDC48, 0xFF58F898, 0xFF00E8D8, 0xFF787878, 0xFF000000, 0xFF000000,
            0xFFFCFCFC, 0xFFA8E4FC, 0xFFC4D4FC, 0xFFD4C8FC, 0xFFFCC4FC, 0xFFFCC4D8, 0xFFFCBCB0, 0xFFFCD8A8,
            0xFFFCE4A0, 0xFFE0FCA0, 0xFFA8F0BC, 0xFFB0FCCC, 0xFF9CFCF0, 0xFFC4C4C4, 0xFF000000, 0xFF000000,
        ] as [color; PALETTE_COLORS_SIZE];

        let mut indices = [0 as byte; PALETTE_INDICES_SIZE];
        for i in 0..PALETTE_INDICES_SIZE { indices[i] = i as byte; }
        return Palette { was_recently_changed:true, colors, indices };
    }

    pub fn get_index(&self, raw_address : address) -> byte
    {
        return self.indices[(raw_address as usize) % PALETTE_INDICES_SIZE];
    }

    pub fn get_color(&self, raw_address : address) -> color
    {
        let index = self.get_index(raw_address) as usize;
        return self.colors[index % self.colors.len()];
    }

    pub fn get_random_color(&self) -> color
    {
        return self.colors[random::<usize>() % self.colors.len()];
    }

    pub fn put_index(&mut self, raw_address : address, index : byte)
    {
        self.indices[(raw_address as usize) % PALETTE_INDICES_SIZE] = index;
        self.was_recently_changed = true;
    }

    pub fn was_recently_changed(&mut self) -> bool
    {
        let was_recently_changed_prev = self.was_recently_changed;
        self.was_recently_changed = false;
        return was_recently_changed_prev;
    }
}
