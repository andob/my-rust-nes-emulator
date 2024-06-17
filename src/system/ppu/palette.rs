use crate::system::{address, byte, color};

const PALETTE_COLORS_SIZE : usize = 64;
const PALETTE_INDICES_SIZE : usize = 0x20;

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
        let mut colors = [0 as color; PALETTE_COLORS_SIZE];
        let colors_bytes = *include_bytes!("palette.pal");
        for i in 0..PALETTE_COLORS_SIZE
        {
            let red = colors_bytes[i*3] as color;
            let green = colors_bytes[i*3+1] as color;
            let blue = colors_bytes[i*3+2] as color;
            colors[i] = (0xFF << 24) | (red << 16) | (green << 8) | blue;
        }

        let mut indices = [0 as byte; PALETTE_INDICES_SIZE];
        for i in 0..indices.len() { indices[i] = i as byte; }
        return Palette { was_recently_changed:true, colors, indices };
    }

    pub fn get_index(&self, raw_address : address) -> byte
    {
        return self.indices[(raw_address as usize) % self.indices.len()];
    }

    pub fn get_color(&self, raw_address : address) -> color
    {
        let index = self.get_index(raw_address) as usize;
        return self.colors[index % self.colors.len()];
    }

    pub fn put_index(&mut self, raw_address : address, index : byte)
    {
        self.indices[(raw_address as usize) % self.indices.len()] = index;
        self.was_recently_changed = true;
    }

    pub fn was_recently_changed(&mut self) -> bool
    {
        let was_recently_changed_prev = self.was_recently_changed;
        self.was_recently_changed = false;
        return was_recently_changed_prev;
    }
}
