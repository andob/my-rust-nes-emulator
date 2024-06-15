use std::fmt::{Display, Formatter};
use std::io::Write;
use crate::system::color;
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};
use crate::system::ppu::textures::texture_pixel_matrix::TexturePixelMatrix;

const PIXEL_HIT_MATRIX_HORIZONTAL_PADDING : usize = (TILE_WIDTH_IN_PIXELS as usize) * 2;
const PIXEL_HIT_MATRIX_VERTICAL_PADDING : usize = (TILE_HEIGHT_IN_PIXELS as usize) * 2;

const PIXEL_HIT_MATRIX_WIDTH : usize = (NES_DISPLAY_WIDTH as usize) + PIXEL_HIT_MATRIX_HORIZONTAL_PADDING * 2;
const PIXEL_HIT_MATRIX_HEIGHT : usize = (NES_DISPLAY_HEIGHT as usize) + PIXEL_HIT_MATRIX_VERTICAL_PADDING * 2;

pub struct PixelHitMatrix
{
    pixels : Box<[[bool; PIXEL_HIT_MATRIX_WIDTH]; PIXEL_HIT_MATRIX_HEIGHT]>,
}

impl PixelHitMatrix
{
    pub fn new() -> PixelHitMatrix
    {
        return PixelHitMatrix
        {
            pixels: Box::new([[false; PIXEL_HIT_MATRIX_WIDTH]; PIXEL_HIT_MATRIX_HEIGHT]),
        };
    }

    pub fn fill_padding(&mut self)
    {
        for y in 0..PIXEL_HIT_MATRIX_HEIGHT
        {
            for x in 0..PIXEL_HIT_MATRIX_HORIZONTAL_PADDING
            {
                self.pixels[y][x] = true;
                self.pixels[y][PIXEL_HIT_MATRIX_WIDTH-x-1] = true;
            }
        }

        for y in 0..PIXEL_HIT_MATRIX_VERTICAL_PADDING
        {
            for x in 0..PIXEL_HIT_MATRIX_WIDTH
            {
                self.pixels[y][x] = true;
                self.pixels[PIXEL_HIT_MATRIX_HEIGHT-y-1][x] = true;
            }
        }
    }

    pub fn get(&self, x : usize, y : usize) -> bool
    {
        if x < PIXEL_HIT_MATRIX_WIDTH && y < PIXEL_HIT_MATRIX_HEIGHT
        {
            return self.pixels[y][x];
        }

        return false;
    }

    pub fn put(&mut self, x : usize, y : usize, value : bool)
    {
        if x < PIXEL_HIT_MATRIX_WIDTH && y < PIXEL_HIT_MATRIX_HEIGHT
        {
            self.pixels[y][x] = value;
        }
    }

    pub fn aggregate(&mut self, texture_pixel_matrix : &TexturePixelMatrix, destination_x : usize, destination_y : usize)
    {
        for pixel_y in 0..TILE_HEIGHT_IN_PIXELS as usize
        {
            let transposed_pixel_y = pixel_y + destination_y;
            if transposed_pixel_y < PIXEL_HIT_MATRIX_HEIGHT
            {
                let pixel_hit_vector = &mut self.pixels[transposed_pixel_y];

                for pixel_x in 0..TILE_WIDTH_IN_PIXELS as usize
                {
                    let pixel = texture_pixel_matrix.get(pixel_x, pixel_y);

                    let transposed_pixel_x = pixel_x + destination_x;
                    if transposed_pixel_x < PIXEL_HIT_MATRIX_WIDTH
                    {
                        let old_value = pixel_hit_vector[transposed_pixel_x];
                        let new_value = old_value || (pixel!=0);

                        // self.pixels[transposed_pixel_y][transposed_pixel_x] = new_value;
                        pixel_hit_vector[transposed_pixel_x] = new_value;
                    }
                }
            }
        }
    }

    pub fn clear(&mut self)
    {
        for y in 0..PIXEL_HIT_MATRIX_HEIGHT
        {
            for x in 0..PIXEL_HIT_MATRIX_WIDTH
            {
                self.pixels[y][x] = false;
            }
        }
    }
}

impl Display for PixelHitMatrix
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        for y in (0..PIXEL_HIT_MATRIX_HEIGHT).step_by(2)
        {
            for x in 0..PIXEL_HIT_MATRIX_WIDTH
            {
                let top_pixel = self.pixels[y][x];
                let bottom_pixel = self.pixels[y+1][x];
                let character = if top_pixel && bottom_pixel {'█'}
                    else if top_pixel {'▀'} else if bottom_pixel {'▄'} else {' '};
                write!(f, "{}", character).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
