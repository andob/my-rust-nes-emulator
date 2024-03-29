use std::ops::Range;
use anyhow::{Context, Result};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::codeloc;
use crate::system::{address, byte, color};
use crate::system::ppu::bus::{PATTERN_TABLE0_END_ADDRESS, PATTERN_TABLE0_START_ADDRESS, PATTERN_TABLE1_END_ADDRESS, PATTERN_TABLE1_START_ADDRESS, PPUBus};
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::textures::{texture_pixel_matrix, Texture};

const NUMBER_OF_TILES_IN_PATTERN_TABLE : address = 255;
const TILE_SIZE_IN_BYTES : address = 16;
pub const TILE_WIDTH_IN_PIXELS : address = 8;
pub const TILE_HEIGHT_IN_PIXELS : address = 8;
const TILE_PLANE_SIZE_IN_BYTES : address = 8;
type tile_plane = [byte; TILE_PLANE_SIZE_IN_BYTES as usize];

pub struct PatternTable<'a>
{
    address_range : Range<address>,
    texture_creator : &'a TextureCreator<WindowContext>,
    textures : Box<[Texture<'a>]>,
}

impl <'a> PatternTable<'a>
{
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>, address_range : Range<address>) -> Result<PatternTable>
    {
        let mut textures : Vec<Texture> = Vec::new();
        for _tile_index in 0..NUMBER_OF_TILES_IN_PATTERN_TABLE
        {
            let texture = Texture::new(texture_creator).context(codeloc!())?;
            textures.push(texture);
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
            self.textures[tile_index as usize].with_lock(|pixels : &mut texture_pixel_matrix|
            {
                let tile_address = self.address_range.start + tile_index * TILE_SIZE_IN_BYTES;
                let (plane1, plane2) = ppu_bus.character_rom.get_tile_planes(tile_address);

                for y in 0..TILE_HEIGHT_IN_PIXELS
                {
                    for x in 0..TILE_WIDTH_IN_PIXELS
                    {
                        let plane1_pixel = (plane1[y as usize] >> (TILE_WIDTH_IN_PIXELS-x-1)) & 0b00000001 != 0;
                        let plane2_pixel = (plane2[y as usize] >> (TILE_WIDTH_IN_PIXELS-x-1)) & 0b00000001 != 0;

                        pixels[y as usize][x as usize] = match (plane1_pixel, plane2_pixel)
                        {
                            (true, true) => ppu_bus.palette.get_color(3 as address),
                            (true, false) => ppu_bus.palette.get_color(1 as address),
                            (false, true) => ppu_bus.palette.get_color(2 as address),
                            (false, false) => 0 as color, //transparent
                        };
                    }
                }
            }).context(codeloc!())?;
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

pub struct PatternTables<'a>
{
    pub left : PatternTable<'a>,
    pub right : PatternTable<'a>,
}

impl <'a> PatternTables<'a>
{
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<PatternTables<'a>>
    {
        let left_address_range = PATTERN_TABLE0_START_ADDRESS..PATTERN_TABLE0_END_ADDRESS;
        let left = PatternTable::new(texture_creator, left_address_range).context(codeloc!())?;

        let right_address_range = PATTERN_TABLE1_START_ADDRESS..PATTERN_TABLE1_END_ADDRESS;
        let right = PatternTable::new(texture_creator, right_address_range).context(codeloc!())?;

        return Ok(PatternTables { left, right });
    }

    pub fn get(&self, base_address : address, relative_address : address) -> &Texture<'a>
    {
        return if self.left.address_range.contains(&base_address) { self.left.get(relative_address) }
        else if self.right.address_range.contains(&base_address) { self.right.get(relative_address) }
        else { self.left.get(relative_address) }
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
