use chrono::Local;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::system::address;
use crate::system::debugger::LoggingOptions;
use crate::system::ppu::bus::{NAMETABLE0_START_ADDRESS, NAMETABLE1_START_ADDRESS};
use crate::system::ppu::metrics::{NES_DISPLAY_HEIGHT, NES_DISPLAY_WIDTH};
use crate::system::ppu::pattern_tables::{PatternTables, TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS};
use crate::system::ppu::{PPU, PPURunEnvironment};
use crate::system::ppu::sprites::Sprite;
use crate::system::ppu::sprites::sprite_zero_hit_detector::SpriteZeroHitDetector;

pub struct PPURenderingPipeline<'a>
{
    ppu : &'a mut PPU,
    env : &'a PPURunEnvironment,
    pattern_tables : &'a PatternTables<'a>,
    sprite_zero_hit_detector : &'a mut SpriteZeroHitDetector,
}

impl <'a> PPURenderingPipeline<'a>
{
    pub fn start
    (
        ppu : &'a mut PPU,
        env : &'a PPURunEnvironment,
        pattern_tables : &'a PatternTables,
        sprite_zero_hit_detector : &'a mut SpriteZeroHitDetector,
        canvas_option : &mut Option<&mut WindowCanvas>,
    ) -> PPURenderingPipeline<'a>
    {
        if let Some(canvas) = canvas_option
        {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
        }

        sprite_zero_hit_detector.clear();

        return PPURenderingPipeline { ppu, env, pattern_tables, sprite_zero_hit_detector };
    }

    pub fn end(self, canvas_option : &mut Option<&mut WindowCanvas>)
    {
        if let Some(canvas) = canvas_option
        {
            canvas.present();
        }
    }

    pub fn render_background_from_nametables(&mut self, canvas_option : &mut Option<&mut WindowCanvas>)
    {
        if !self.env.debugger.should_render_background { return; }
        if !self.ppu.mask_flags.should_show_background { return; }

        let first_nametable_address = self.ppu.control_flags.base_nametable_address;
        let first_nametable_projection_offset = (0 as address, 0 as address);
        self.render_background_from_nametable(canvas_option, first_nametable_address, first_nametable_projection_offset);

        let second_nametable_address = if self.ppu.control_flags.base_nametable_address
            == NAMETABLE0_START_ADDRESS { NAMETABLE1_START_ADDRESS } else { NAMETABLE0_START_ADDRESS };
        let second_nametable_projection_offset = (NES_DISPLAY_WIDTH, 0 as address);
        self.render_background_from_nametable(canvas_option, second_nametable_address, second_nametable_projection_offset);
    }

    fn render_background_from_nametable(&mut self, canvas_option : &mut Option<&mut WindowCanvas>,
                                        nametable_address : address, projection_offset : (address, address))
    {
        let (scale_x, scale_y) = self.ppu.window_metrics.get_scale();

        let projection_offset_x = projection_offset.0 / (TILE_WIDTH_IN_PIXELS as address);
        let projection_offset_y = projection_offset.1 / (TILE_HEIGHT_IN_PIXELS as address);

        let number_of_rows = NES_DISPLAY_WIDTH / (TILE_WIDTH_IN_PIXELS as address);
        let number_of_columns = NES_DISPLAY_HEIGHT / (TILE_HEIGHT_IN_PIXELS as address);

        for y_index in 0..number_of_columns
        {
            for x_index in 0..number_of_rows
            {
                let pattern_table_index =
                    if self.env.debugger.should_debug_pattern_table { y_index * number_of_rows + x_index }
                    else { self.ppu.bus.get(nametable_address + y_index * number_of_rows + x_index) as address };

                let pattern_table_base_address = self.ppu.control_flags.base_pattern_table_address_for_background;
                let pattern = self.pattern_tables.get(pattern_table_base_address, pattern_table_index);

                //todo implement scrolling
                let unscaled_x = (((x_index + projection_offset_x) as f32) - self.ppu.scroll_x) * (TILE_WIDTH_IN_PIXELS as f32);
                let unscaled_y = (((y_index + projection_offset_y) as f32) - self.ppu.scroll_y) * (TILE_HEIGHT_IN_PIXELS as f32);

                let scaled_width = (TILE_WIDTH_IN_PIXELS as f32) * scale_x;
                let scaled_height = (TILE_HEIGHT_IN_PIXELS as f32) * scale_y;
                let scaled_x = unscaled_x * scale_x;
                let scaled_y = unscaled_y * scale_y;

                if let Some(canvas) = canvas_option
                {
                    let coords = Rect::new(scaled_x as i32, scaled_y as i32, scaled_width as u32, scaled_height as u32);
                    canvas.copy(pattern.sdl(), None, Some(coords)).unwrap_or_default();
                }

                self.sprite_zero_hit_detector.add_background_texture(pattern, unscaled_x as usize, unscaled_y as usize);
            }
        }
    }

    pub fn render_background_sprites_from_oam(&mut self, canvas_option : &mut Option<&mut WindowCanvas>)
    {
        if !self.env.debugger.should_render_sprites { return; }
        if !self.ppu.mask_flags.should_show_sprites { return; }

        let sprites =
            if self.ppu.control_flags.should_use_16pixel_high_sprites
                { self.ppu.oam.get_16pixel_high_background_sprites() }
            else { self.ppu.oam.get_8pixel_high_background_sprites() };

        self.render_sprites_from_oam(canvas_option, sprites);
    }

    pub fn render_foreground_sprites_from_oam(&mut self, canvas_option : &mut Option<&mut WindowCanvas>)
    {
        if !self.env.debugger.should_render_sprites { return; }
        if !self.ppu.mask_flags.should_show_sprites { return; }

        let sprites =
            if self.ppu.control_flags.should_use_16pixel_high_sprites
                { self.ppu.oam.get_16pixel_high_foreground_sprites() }
            else { self.ppu.oam.get_8pixel_high_foreground_sprites() };

        self.render_sprites_from_oam(canvas_option, sprites);
    }

    fn render_sprites_from_oam(&mut self, canvas_option : &mut Option<&mut WindowCanvas>, sprites : Vec<Sprite>)
    {
        let (scale_x, scale_y) = self.ppu.window_metrics.get_scale();
        if self.ppu.control_flags.should_use_16pixel_high_sprites
        {
            for sprite in sprites
            {
                let top_pattern = if sprite.should_use_right_pattern_table
                    { self.pattern_tables.right.get(sprite.pattern_table_index) }
                else { self.pattern_tables.left.get(sprite.pattern_table_index) };

                let bottom_pattern = if sprite.should_use_right_pattern_table
                    { self.pattern_tables.right.get(sprite.pattern_table_index+1) }
                else { self.pattern_tables.left.get(sprite.pattern_table_index+1) };

                let width = (TILE_WIDTH_IN_PIXELS as f32) * scale_x;
                let height = (TILE_HEIGHT_IN_PIXELS as f32) * scale_y;
                let x = (sprite.x as f32) * scale_x;
                let top_y = (sprite.y as f32) * scale_y;
                let bottom_y = top_y + height;

                if let Some(canvas) = canvas_option
                {
                    let top_coords = Rect::new(x as i32, top_y as i32, width as u32, height as u32);
                    canvas.copy_ex(top_pattern.sdl(), None, Some(top_coords), 0f64, None,
                        sprite.should_flip_horizontally, sprite.should_flip_vertically).unwrap_or_default();

                    let bottom_coords = Rect::new(x as i32, bottom_y as i32, width as u32, height as u32);
                    canvas.copy_ex(bottom_pattern.sdl(), None, Some(bottom_coords), 0f64, None,
                        sprite.should_flip_horizontally, sprite.should_flip_vertically).unwrap_or_default();
                }

                self.sprite_zero_hit_detector.add_16pixel_high_sprite(sprite, top_pattern, bottom_pattern);
            }
        }
        else
        {
            for sprite in sprites
            {
                let pattern_table_base_address = self.ppu.control_flags.base_pattern_table_address_for_foreground;
                let pattern = self.pattern_tables.get(pattern_table_base_address, sprite.pattern_table_index);

                let width = (TILE_WIDTH_IN_PIXELS as f32) * scale_x;
                let height = (TILE_HEIGHT_IN_PIXELS as f32) * scale_y;
                let x = (sprite.x as f32) * scale_x;
                let y = (sprite.y as f32) * scale_y;

                if let Some(canvas) = canvas_option
                {
                    let coords = Rect::new(x as i32, y as i32, width as u32, height as u32);
                    canvas.copy_ex(pattern.sdl(), None, Some(coords), 0f64, None,
                        sprite.should_flip_horizontally, sprite.should_flip_vertically).unwrap_or_default();
                }

                self.sprite_zero_hit_detector.add_8pixel_high_sprite(sprite, pattern);
            }
        }
    }

    pub fn detect_sprite_zero_hit(&mut self, logging_options : &LoggingOptions)
    {
        if !self.ppu.mask_flags.should_show_sprites
        {
            self.ppu.status_flags.is_sprite_zero_hit = false;
        }
        else if self.sprite_zero_hit_detector.was_sprite_zero_hit()
        {
            self.ppu.status_flags.is_sprite_zero_hit = true;
        }

        if self.env.debugger.should_debug_sprite_zero_hit
        {
            self.sprite_zero_hit_detector.debug(self.ppu);
        }

        if logging_options.is_ppu_sprite_zero_hit_logging_enabled && self.ppu.status_flags.is_sprite_zero_hit
        {
            println!("[PPU] Sprite Zero Hit at {}!", Local::now().format("%H:%M:%S"));
        }
    }
}
