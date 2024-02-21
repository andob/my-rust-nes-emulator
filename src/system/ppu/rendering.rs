use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::system::address;
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::oam::PPUOAMSpriteDescriptor;
use crate::system::ppu::pattern_table::PatternTables;
use crate::system::ppu::PPU;

pub struct PPURenderingPipeline<'a>
{
    ppu : &'a PPU,
    pattern_tables : &'a PatternTables<'a>,
}

impl <'a> PPURenderingPipeline<'a>
{
    pub fn start(ppu : &'a PPU, pattern_tables : &'a PatternTables, canvas : &mut WindowCanvas) -> PPURenderingPipeline<'a>
    {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        return PPURenderingPipeline { ppu, pattern_tables };
    }

    pub fn commit_rendering(self, canvas : &mut WindowCanvas)
    {
        canvas.present();
    }

    pub fn render_nametable_background(&self, canvas : &mut WindowCanvas)
    {
        let (scale_x, scale_y) = self.ppu.window_metrics.get_scale();
        let number_of_columns = NES_DISPLAY_HEIGHT / (self.ppu.control_flags.sprite_height as address);
        let number_of_rows = NES_DISPLAY_WIDTH / (self.ppu.control_flags.sprite_width as address);

        for y_index in 0..number_of_columns
        {
            for x_index in 0..=number_of_rows
            {
                let nametable_address = self.ppu.control_flags.base_nametable_address + y_index * number_of_rows + x_index;
                let pattern_table_index = self.ppu.bus.get(nametable_address) as address;
                let pattern = self.pattern_tables.get(self.ppu.control_flags.base_pattern_table_address, pattern_table_index);

                let width = (self.ppu.control_flags.sprite_width as f32) * scale_x;
                let height = (self.ppu.control_flags.sprite_height as f32) * scale_y;
                let x = (x_index as f32) * width;
                let y = (y_index as f32) * height;

                let coords = Rect::new(x as i32, y as i32, width as u32, height as u32);
                canvas.copy(pattern, None, Some(coords)).unwrap_or_default();
            }
        }
    }

    pub fn render_oam_background_sprites(&self, canvas : &mut WindowCanvas)
    {
        self.render_oam_sprites(canvas, self.ppu.oam.get_background_sprites());
    }

    pub fn render_oam_foreground_sprites(&self, canvas : &mut WindowCanvas)
    {
        self.render_oam_sprites(canvas, self.ppu.oam.get_foreground_sprites());
    }

    fn render_oam_sprites(&self, canvas : &mut WindowCanvas, sprites : Vec<PPUOAMSpriteDescriptor>)
    {
        let (scale_x, scale_y) = self.ppu.window_metrics.get_scale();
        for sprite in sprites
        {
            let top_pattern = if sprite.should_use_right_pattern_table
                { self.pattern_tables.right.get(sprite.pattern_table_index) }
            else { self.pattern_tables.left.get(sprite.pattern_table_index) };

            let bottom_pattern = if sprite.should_use_right_pattern_table
                { self.pattern_tables.right.get(sprite.pattern_table_index+1) }
                else { self.pattern_tables.left.get(sprite.pattern_table_index+1) };

            let width = (self.ppu.control_flags.sprite_width as f32) * scale_x;
            let height = (self.ppu.control_flags.sprite_height as f32) * scale_y;
            let x = (sprite.x as f32) * scale_x;
            let top_y = (sprite.y as f32) * scale_y;
            let bottom_y = top_y + height;

            let top_coords = Rect::new(x as i32, top_y as i32, width as u32, height as u32);
            canvas.copy(top_pattern, None, Some(top_coords)).unwrap_or_default();

            let bottom_coords = Rect::new(x as i32, bottom_y as i32, width as u32, height as u32);
            canvas.copy(bottom_pattern, None, Some(bottom_coords)).unwrap_or_default();
        }
    }
}
