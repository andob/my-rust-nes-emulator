use crate::new_screen_pixel_matrix;
use crate::system::{address, byte, color};
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};
use crate::system::ppu::textures::{screen_pixel_matrix, Texture};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sprite
{
    pub x : byte,
    pub y : byte,
    pub should_use_right_pattern_table : bool,
    pub pattern_table_index : address,
    pub palette_index : byte, //todo how to use this?
    pub should_flip_horizontally : bool,
    pub should_flip_vertically : bool,
    pub is_sprite_zero : bool,
}

pub struct SpriteZeroHitDetector
{
    screen_pixel_matrix : screen_pixel_matrix
}

impl SpriteZeroHitDetector
{
    pub fn new() -> SpriteZeroHitDetector
    {
        return SpriteZeroHitDetector
        {
            screen_pixel_matrix: new_screen_pixel_matrix!()
        };
    }

    fn aggregate_texture(&mut self, texture : &Texture, texture_destination_x : usize, texture_destination_y : usize)
    {
        for texture_source_y in 0..TILE_WIDTH_IN_PIXELS as usize
        {
            for texture_source_x in 0..TILE_HEIGHT_IN_PIXELS as usize
            {
                let pixel = texture.pixel_matrix[texture_source_y][texture_source_x];

                let screen_destination_x = texture_destination_x + texture_source_x;
                let screen_destination_y = texture_destination_y + texture_source_y;
                if screen_destination_x<NES_DISPLAY_WIDTH as usize &&
                   screen_destination_y<NES_DISPLAY_HEIGHT as usize
                {
                    let monochrome_pixel = if pixel!=0 {1} else {0} as color;
                    self.screen_pixel_matrix[screen_destination_y][screen_destination_x] += monochrome_pixel;
                }
            }
        }
    }

    pub fn add_background_texture(&mut self, texture : &Texture, texture_destination_x : usize, texture_destination_y : usize)
    {
        self.aggregate_texture(texture, texture_destination_x, texture_destination_y);
    }

    pub fn add_16pixel_high_sprite(&mut self, sprite: Sprite, top_texture : &Texture, bottom_texture : &Texture)
    {
        let top_texture_y = sprite.y as usize;
        let bottom_texture_y = (sprite.y as usize) + (TILE_HEIGHT_IN_PIXELS as usize);

        if sprite.is_sprite_zero
        {
            //todo implement should_flip_horizontally and should_flip_vertically
            self.aggregate_texture(top_texture, sprite.x as usize, top_texture_y);
            self.aggregate_texture(bottom_texture, sprite.x as usize, bottom_texture_y);
        }
        else
        {
            //todo implement should_flip_horizontally and should_flip_vertically
            self.aggregate_texture(top_texture, sprite.x as usize, top_texture_y);
            self.aggregate_texture(bottom_texture, sprite.x as usize, bottom_texture_y);
        }
    }

    pub fn add_8pixel_high_sprite(&mut self, sprite : Sprite, texture : &Texture)
    {
        if sprite.is_sprite_zero
        {
            //todo implement should_flip_horizontally and should_flip_vertically
            self.aggregate_texture(texture, sprite.x as usize, sprite.y as usize);
        }
        else
        {
            //todo implement should_flip_horizontally and should_flip_vertically
            self.aggregate_texture(texture, sprite.x as usize, sprite.y as usize);
        }
    }

    pub fn was_sprite_zero_hit(&self) -> bool
    {
        //todo it's not working properly... make sprite zero hit test pass
        for y in 0..NES_DISPLAY_HEIGHT as usize
        {
            for x in 0..NES_DISPLAY_WIDTH as usize
            {
                if self.screen_pixel_matrix[y][x] >= 2
                {
                    return true;
                }
            }
        }

        return false;
    }
}
