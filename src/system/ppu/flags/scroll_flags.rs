use crate::system::ppu::character_rom::SMB1_CHARACTER_ROM_HASH;

pub struct PPUScrollFlags
{
    pub x : f32,
    pub y : f32,
    write_index : usize,
    was_written_mid_frame : bool,
    rom_hash : String,
}

impl PPUScrollFlags
{
    pub fn new(rom_hash : &String) -> PPUScrollFlags
    {
        return PPUScrollFlags
        {
            x: 0f32, y: 0f32,
            write_index: 0,
            was_written_mid_frame: false,
            rom_hash: rom_hash.clone(),
        };
    }

    pub fn should_prevent_rendering(&self) -> bool
    {
        return self.was_written_mid_frame;
    }

    pub fn write(&mut self, value : f32)
    {
        if self.rom_hash == SMB1_CHARACTER_ROM_HASH
        {
            match (self.write_index % 2, (self.write_index / 2) % 3)
            {
                (0, 0) => { self.was_written_mid_frame = true; }
                (0, 1) => { self.was_written_mid_frame = true; }
                (0, 2) => { self.was_written_mid_frame = false; self.x = value; }
                (_, _) => {}
            }
        }

        self.write_index = self.write_index.wrapping_add(1);
    }
}
