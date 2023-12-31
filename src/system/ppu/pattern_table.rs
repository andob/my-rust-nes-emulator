use std::ops::Range;
use anyhow::{anyhow, Context, Result};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use crate::codeloc;
use crate::system::{address, byte, color};
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;

const NUMBER_OF_TILES_IN_PATTERN_TABLE : address = 255;
const TILE_SIZE_IN_BYTES : address = 16;
const TILE_WIDTH_IN_PIXELS : address = 8;
const TILE_HEIGHT_IN_PIXELS : address = 8;
const TILE_PLANE_SIZE_IN_BYTES : address = 8;
type tile_plane = [byte; TILE_PLANE_SIZE_IN_BYTES as usize];

pub struct PatternTable<'a>
{
    address_range : Range<address>,
    texture_creator : &'a TextureCreator<WindowContext>,
    textures : Box<[Box<Texture<'a>>]>,
}

impl <'a> PatternTable<'a>
{
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>, address_range : Range<address>) -> Result<PatternTable>
    {
        let mut textures : Vec<Box<Texture>> = Vec::new();
        for _tile_index in 0..NUMBER_OF_TILES_IN_PATTERN_TABLE
        {
            let texture = texture_creator.create_texture_streaming(
                /*format*/ PixelFormatEnum::RGBA8888,
                /*width*/ TILE_WIDTH_IN_PIXELS as u32,
                /*height*/ TILE_HEIGHT_IN_PIXELS as u32,
            ).context(codeloc!())?;

            textures.push(Box::new(texture));
        }

        return Ok(PatternTable
        {
            address_range: address_range,
            texture_creator: texture_creator,
            textures: textures.into_boxed_slice(),
        });
    }

    pub fn refresh_textures(self : &mut PatternTable<'a>, ppu_bus : &PPUBus) -> Result<()>
    {
        for tile_index in 0..NUMBER_OF_TILES_IN_PATTERN_TABLE
        {
            self.textures[tile_index as usize].with_lock(None, |buffer : &mut[u8], pitch : usize|
            {
                let tile_address = self.address_range.start + tile_index * TILE_SIZE_IN_BYTES;
                let (plane1, plane2) = ppu_bus.character_rom.get_tile_planes(tile_address);

                for y in 0..TILE_HEIGHT_IN_PIXELS
                {
                    for x in 0..TILE_WIDTH_IN_PIXELS
                    {
                        let plane1_pixel = (plane1[y as usize] >> (TILE_WIDTH_IN_PIXELS-x-1)) & 0b00000001 != 0;
                        let plane2_pixel = (plane2[y as usize] >> (TILE_WIDTH_IN_PIXELS-x-1)) & 0b00000001 != 0;

                        let pixel = match (plane1_pixel, plane2_pixel)
                        {
                            (true, true) => ppu_bus.palette.get_color(3 as address),
                            (true, false) => ppu_bus.palette.get_color(1 as address),
                            (false, true) => ppu_bus.palette.get_color(2 as address),
                            (false, false) => 0 as color, //transparent
                        };

                        let offset = (y as usize) * pitch + (x as usize) * 4;
                        buffer[offset+0] = ((pixel>>24)&0xFF) as byte;
                        buffer[offset+1] = ((pixel>>16)&0xFF) as byte;
                        buffer[offset+2] = ((pixel>>08)&0xFF) as byte;
                        buffer[offset+3] = ((pixel>>00)&0xFF) as byte;
                    }
                }
            }).map_err(|msg|anyhow!(msg.clone())).context(codeloc!())?;
        }

        return Ok(());
    }

    pub fn len(&self) -> usize
    {
        return NUMBER_OF_TILES_IN_PATTERN_TABLE as usize;
    }

    pub fn get(&self, index : address) -> &Texture<'a>
    {
        return &self.textures[(index % NUMBER_OF_TILES_IN_PATTERN_TABLE) as usize];
    }
}

impl CharacterROM
{
    pub fn get_tile_planes(&self, start_raw_address : address) -> (tile_plane, tile_plane)
    {
        let mut first_plane : tile_plane = [0; TILE_PLANE_SIZE_IN_BYTES as usize];
        let mut second_plane : tile_plane = [0; TILE_PLANE_SIZE_IN_BYTES as usize];

        for offset in 0..TILE_PLANE_SIZE_IN_BYTES
        {
            first_plane[offset as usize] = self.get(start_raw_address + offset);
            second_plane[offset as usize] = self.get(start_raw_address + TILE_PLANE_SIZE_IN_BYTES + offset);
        }

        return (first_plane, second_plane);
    }
}
