use crate::system::{address, byte};

pub mod pixel_hit_matrix;
pub mod sprite_zero_hit_detector;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Sprite
{
    pub index : byte,
    pub x : byte,
    pub y : byte,
    pub should_use_right_pattern_table : bool,
    pub pattern_table_index : address,
    pub palette_index : byte, //todo how to use this?
    pub should_flip_horizontally : bool,
    pub should_flip_vertically : bool,
}
