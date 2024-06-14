use std::fmt::{Display, Formatter};
use std::io::Write;
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};
use crate::system::ppu::textures::texture_pixel_matrix::TexturePixelMatrix;

const PIXEL_HIT_MATRIX_DEFAULT_HORIZONTAL_PADDING : usize = (TILE_WIDTH_IN_PIXELS as usize) * 2;
const PIXEL_HIT_MATRIX_DEFAULT_VERTICAL_PADDING : usize = (TILE_HEIGHT_IN_PIXELS as usize) * 2;

const PIXEL_HIT_MATRIX_DEFAULT_WIDTH : usize = (NES_DISPLAY_WIDTH as usize) + PIXEL_HIT_MATRIX_DEFAULT_HORIZONTAL_PADDING * 2;
const PIXEL_HIT_MATRIX_DEFAULT_HEIGHT : usize = (NES_DISPLAY_HEIGHT as usize) + PIXEL_HIT_MATRIX_DEFAULT_VERTICAL_PADDING * 2;

pub struct PixelHitMatrix
{
    width : usize, height : usize,
    horizontal_padding : usize,
    vertical_padding : usize,
    pixels : Box<[bool]>,
}

impl PixelHitMatrix
{
    pub fn new() -> PixelHitMatrix
    {
        let mut matrix = PixelHitMatrix
        {
            width: PIXEL_HIT_MATRIX_DEFAULT_WIDTH,
            height: PIXEL_HIT_MATRIX_DEFAULT_HEIGHT,
            horizontal_padding: PIXEL_HIT_MATRIX_DEFAULT_HORIZONTAL_PADDING,
            vertical_padding: PIXEL_HIT_MATRIX_DEFAULT_VERTICAL_PADDING,
            pixels: Box::new([false; PIXEL_HIT_MATRIX_DEFAULT_WIDTH*PIXEL_HIT_MATRIX_DEFAULT_HEIGHT]),
        };

        for x in 0..matrix.horizontal_padding
        {
            for y in 0..matrix.height
            {
                matrix.pixels[matrix.index(x, y)] = true;
                matrix.pixels[matrix.index(matrix.width-x-1, y)] = true;
            }
        }

        for x in 0..matrix.width
        {
            for y in 0..matrix.vertical_padding
            {
                matrix.pixels[matrix.index(x, y)] = true;
                matrix.pixels[matrix.index(x, matrix.height-y-1)] = true;
            }
        }

        return matrix;
    }

    #[inline]
    fn index(&self, x : usize, y : usize) -> usize
    {
        return x * self.height + y;
    }

    pub fn get(&self, x : usize, y : usize) -> bool
    {
        let normalized_x = x + self.horizontal_padding;
        let normalized_y = y + self.vertical_padding;
        let normalized_index = self.index(normalized_x, normalized_y);
        if normalized_index < self.pixels.len()
        {
            return self.pixels[normalized_index];
        }

        return false;
    }

    pub fn put(&mut self, x : usize, y : usize, value : bool)
    {
        let normalized_x = x + self.horizontal_padding;
        let normalized_y = y + self.vertical_padding;
        let normalized_index = self.index(normalized_x, normalized_y);
        if normalized_index < self.pixels.len()
        {
            self.pixels[normalized_index] = value;
        }
    }

    pub fn aggregate(&mut self, texture_pixel_matrix : &TexturePixelMatrix, destination_x : usize, destination_y : usize)
    {
        for pixel_y in 0..TILE_WIDTH_IN_PIXELS as usize
        {
            for pixel_x in 0..TILE_HEIGHT_IN_PIXELS as usize
            {
                let pixel = texture_pixel_matrix.get(pixel_x, pixel_y);

                let transposed_pixel_x = pixel_x + destination_x;
                let transposed_pixel_y = pixel_y + destination_y;

                let old_value = self.get(transposed_pixel_x, transposed_pixel_y);
                let new_value = old_value || (pixel!=0);

                self.put(transposed_pixel_x, transposed_pixel_y, new_value);
            }
        }
    }

    pub fn clear(&mut self)
    {
        for i in 0..self.pixels.len()
        {
            self.pixels[i] = false;
        }
    }
}

impl Display for PixelHitMatrix
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        for y in (0..self.height).step_by(2)
        {
            for x in 0..self.width
            {
                let top_pixel = self.pixels[self.index(x, y)];
                let bottom_pixel = self.pixels[self.index(x, y+1)];
                let character = if top_pixel && bottom_pixel {'█'}
                    else if top_pixel {'▀'} else if bottom_pixel {'▄'} else {' '};
                write!(f, "{}", character).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
