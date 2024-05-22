use std::ffi::{c_char, CString};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use nix::libc;
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::TILE_HEIGHT_IN_PIXELS;
use crate::system::ppu::PPU;
use crate::system::ppu::sprites::pixel_hit_matrix::PixelHitMatrix;
use crate::system::ppu::sprites::Sprite;
use crate::system::ppu::textures::Texture;

pub struct SpriteZeroHitDetector
{
    frame_id : usize,
    tmp : bool,
    background_pixel_hit_matrix : PixelHitMatrix,
    foreground_pixel_hit_matrix : PixelHitMatrix,
}

impl SpriteZeroHitDetector
{
    pub fn new(ppu : &mut PPU) -> SpriteZeroHitDetector
    {
        return SpriteZeroHitDetector
        {
            frame_id: ppu.frame_id,
            tmp: ppu.mask_flags.should_show_background, //todo remove this
            background_pixel_hit_matrix: PixelHitMatrix::new(),
            foreground_pixel_hit_matrix: PixelHitMatrix::new(),
        };
    }

    pub fn add_background_texture(&mut self, texture : &Texture, texture_destination_x : usize, texture_destination_y : usize)
    {
        self.background_pixel_hit_matrix.aggregate(&texture.pixel_matrix, texture_destination_x, texture_destination_y);
    }

    pub fn add_16pixel_high_sprite(&mut self, sprite : Sprite, top_texture : &Texture, bottom_texture : &Texture)
    {
        let top_texture_y = sprite.y as usize;
        let top_texture_matrix = match (sprite.should_flip_horizontally, sprite.should_flip_vertically)
        {
            (true, true) => { bottom_texture.pixel_matrix.flipped_horizontally().flipped_vertically() }
            (true, false) => { top_texture.pixel_matrix.flipped_horizontally() }
            (false, true) => { bottom_texture.pixel_matrix.flipped_vertically() }
            (false, false) => { top_texture.pixel_matrix.clone() }
        };

        let bottom_texture_y = (sprite.y as usize) + (TILE_HEIGHT_IN_PIXELS as usize);
        let bottom_texture_matrix = match (sprite.should_flip_horizontally, sprite.should_flip_vertically)
        {
            (true, true) => { top_texture.pixel_matrix.flipped_horizontally().flipped_vertically() }
            (true, false) => { bottom_texture.pixel_matrix.flipped_horizontally() }
            (false, true) => { top_texture.pixel_matrix.flipped_vertically() }
            (false, false) => { bottom_texture.pixel_matrix.clone() }
        };

        let pixel_hit_matrix =
            if sprite.is_sprite_zero { &mut self.foreground_pixel_hit_matrix }
            else { &mut self.background_pixel_hit_matrix };

        pixel_hit_matrix.aggregate(&top_texture_matrix, sprite.x as usize, top_texture_y);
        pixel_hit_matrix.aggregate(&bottom_texture_matrix, sprite.x as usize, bottom_texture_y);
    }

    pub fn add_8pixel_high_sprite(&mut self, sprite : Sprite, texture : &Texture)
    {
        let texture_pixel_matrix = match (sprite.should_flip_horizontally, sprite.should_flip_vertically)
        {
            (true, true) => { texture.pixel_matrix.flipped_horizontally().flipped_vertically() }
            (true, false) => { texture.pixel_matrix.flipped_horizontally() }
            (false, true) => { texture.pixel_matrix.flipped_vertically() }
            (false, false) => { texture.pixel_matrix.clone() }
        };

        let pixel_hit_matrix =
            if sprite.is_sprite_zero { &mut self.foreground_pixel_hit_matrix }
            else { &mut self.background_pixel_hit_matrix };

        pixel_hit_matrix.aggregate(&texture_pixel_matrix, sprite.x as usize, sprite.y as usize);
    }

    pub fn debug(&self)
    {
        let file_name = format!("./tmp/{}", self.frame_id);
        self.debug_dump_to_file(&file_name);

        unsafe
        {
            // let command = format!("nano {}", file_name);
            // let command_cstring = CString::new(command).unwrap();
            // libc::system(command_cstring.as_ptr() as *const c_char);
        }
    }

    fn debug_dump_to_file(&self, file_name : &String)
    {
        let mut file = OpenOptions::new()
            .create_new(!Path::new(file_name).exists())
            .write(true).append(false)
            .open(file_name).unwrap();

        write!(file, "BACKGROUND: (should_show={})\n{}\n", self.tmp, self.background_pixel_hit_matrix.to_string()).unwrap();
        write!(file, "FOREGROUND:\n{}\n", self.foreground_pixel_hit_matrix.to_string()).unwrap();
    }

    pub fn was_sprite_zero_hit(&self) -> bool
    {
        for y in 0..NES_DISPLAY_HEIGHT as usize
        {
            for x in 0..NES_DISPLAY_WIDTH as usize
            {
                let background_pixel = self.background_pixel_hit_matrix.get(x, y);
                let foreground_pixel = self.foreground_pixel_hit_matrix.get(x, y);
                if background_pixel && foreground_pixel
                {
                    return true;
                }
            }
        }

        return false;
    }
}
