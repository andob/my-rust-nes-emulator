use std::fmt::{Display, Formatter};
use crate::system::color;
use crate::system::ppu::pattern_tables::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};

pub struct TexturePixelMatrix
{
    pixels : Box<[[color; TILE_WIDTH_IN_PIXELS as usize]; TILE_HEIGHT_IN_PIXELS as usize]>
}

impl TexturePixelMatrix
{
    pub fn new() -> TexturePixelMatrix
    {
        return TexturePixelMatrix
        {
            pixels: Box::new([[0; TILE_WIDTH_IN_PIXELS as usize]; TILE_HEIGHT_IN_PIXELS as usize])
        };
    }

    #[inline]
    pub fn width(&self) -> usize
    {
        return TILE_WIDTH_IN_PIXELS as usize;
    }

    #[inline]
    pub fn height(&self) -> usize
    {
        return TILE_HEIGHT_IN_PIXELS as usize;
    }

    pub fn get(&self, x : usize, y : usize) -> color
    {
        let normalized_x = x % (TILE_WIDTH_IN_PIXELS as usize);
        let normalized_y = y % (TILE_HEIGHT_IN_PIXELS as usize);

        return self.pixels[normalized_y][normalized_x];
    }

    pub fn put(&mut self, x : usize, y : usize, pixel : color)
    {
        let normalized_x = x % (TILE_WIDTH_IN_PIXELS as usize);
        let normalized_y = y % (TILE_HEIGHT_IN_PIXELS as usize);

        self.pixels[normalized_y][normalized_x] = pixel;
    }

    pub fn flipped_vertically(&self) -> TexturePixelMatrix
    {
        let mut flipped = TexturePixelMatrix::new();

        for y in 0..TILE_HEIGHT_IN_PIXELS as usize
        {
            for x in 0..TILE_WIDTH_IN_PIXELS as usize
            {
                let flipped_y = (TILE_HEIGHT_IN_PIXELS as usize) - y - 1;
                flipped.pixels[flipped_y][x] = self.pixels[y][x];
            }
        }

        return flipped;
    }

    pub fn flipped_horizontally(&self) -> TexturePixelMatrix
    {
        let mut flipped = TexturePixelMatrix::new();

        for y in 0..TILE_HEIGHT_IN_PIXELS as usize
        {
            for x in 0..TILE_WIDTH_IN_PIXELS as usize
            {
                let flipped_x = (TILE_WIDTH_IN_PIXELS as usize) - x - 1;
                flipped.pixels[y][flipped_x] = self.pixels[y][x];
            }
        }

        return flipped;
    }
}

impl Clone for TexturePixelMatrix
{
    fn clone(&self) -> Self
    {
        let mut cloned = TexturePixelMatrix::new();

        for y in 0..TILE_HEIGHT_IN_PIXELS as usize
        {
            for x in 0..TILE_WIDTH_IN_PIXELS as usize
            {
                cloned.pixels[y][x] = self.pixels[y][x];
            }
        }

        return cloned;
    }
}

impl Display for TexturePixelMatrix
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        for y in 0..self.height()
        {
            for x in 0..self.width()
            {
                let pixel = self.get(x, y);
                write!(f, "{}", if pixel>0 {'▓'} else {'░'}).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
