use anyhow::{anyhow, Context, Result};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, Texture as SDLTexture, TextureCreator};
use sdl2::video::WindowContext;
use crate::codeloc;
use crate::system::{byte, color};
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};

#[macro_export]
macro_rules! new_screen_pixel_matrix { () => { [[0; NES_DISPLAY_WIDTH as usize]; NES_DISPLAY_HEIGHT as usize] } }
pub type screen_pixel_matrix = [[color; NES_DISPLAY_WIDTH as usize]; NES_DISPLAY_HEIGHT as usize];

#[macro_export]
macro_rules! new_texture_pixel_matrix { () => { [[0; TILE_WIDTH_IN_PIXELS as usize]; TILE_HEIGHT_IN_PIXELS as usize] } }
pub type texture_pixel_matrix = [[color; TILE_WIDTH_IN_PIXELS as usize]; TILE_HEIGHT_IN_PIXELS as usize];

pub struct Texture<'a>
{
    sdl : Box<SDLTexture<'a>>,
    pub pixel_matrix : texture_pixel_matrix,
}

impl <'a> Texture<'a>
{
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<Texture<'a>>
    {
        let format = PixelFormatEnum::RGBA8888;
        let width = TILE_WIDTH_IN_PIXELS as u32;
        let height = TILE_HEIGHT_IN_PIXELS as u32;
        let mut sdl_texture = texture_creator.create_texture_streaming(format, width, height).context(codeloc!())?;
        sdl_texture.set_blend_mode(BlendMode::Blend);

        let texture = Texture
        {
            sdl: Box::new(sdl_texture),
            pixel_matrix: new_texture_pixel_matrix!(),
        };

        return Ok(texture);
    }

    pub fn sdl(&self) -> &SDLTexture<'a> { &*self.sdl }

    #[inline]
    pub fn with_lock<CALLBACK>(&mut self, callback : CALLBACK) -> Result<()>
    where CALLBACK : FnOnce(&mut texture_pixel_matrix) -> ()
    {
        return self.sdl.with_lock(None, |buffer : &mut[u8], pitch : usize|
        {
            callback(&mut self.pixel_matrix);

            for y in 0..TILE_HEIGHT_IN_PIXELS
            {
                for x in 0..TILE_WIDTH_IN_PIXELS
                {
                    let pixel = self.pixel_matrix[y as usize][x as usize];
                    let offset = (y as usize) * pitch + (x as usize) * 4;

                    buffer[offset+0] = ((pixel>>24)&0xFF) as byte;
                    buffer[offset+1] = ((pixel>>16)&0xFF) as byte;
                    buffer[offset+2] = ((pixel>>08)&0xFF) as byte;
                    buffer[offset+3] = ((pixel>>00)&0xFF) as byte;
                }
            }
        }).map_err(|msg|anyhow!(msg.clone()));
    }
}
