use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::codeloc;
use crate::system::debugger::{LoggingOptions, PPUDebugger};
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::oam::PPUOAM;
use crate::system::ppu::pattern_table::PatternTable;

pub mod character_rom;
pub mod bus;
pub mod oam;
mod palette;
mod pattern_table;

pub struct PPU
{
    pub bus : PPUBus,
    pub oam : PPUOAM,
}

pub struct PPURunEnvironment
{
    pub debugger : PPUDebugger,
    pub logging_options : LoggingOptions,
    pub headless : bool,
    pub is_shutting_down : Arc<AtomicBool>,
}

impl PPU
{
    pub fn new(character_rom : CharacterROM) -> PPU
    {
        return PPU
        {
            bus: PPUBus::new(character_rom),
            oam: PPUOAM::new(),
        };
    }

    pub fn run(self : &mut PPU, env : PPURunEnvironment) -> Result<()>
    {
        let ppu = self;

        let opengl_driver_index = sdl2::render::drivers().enumerate()
            .find(|(_index, driver)| driver.name=="opengl")
            .map(|(index, _driver)| index).unwrap_or_default() as u32;

        let sdl = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let video_subsystem = sdl.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let screen_size = video_subsystem.current_display_mode(0).map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let mut window = video_subsystem.window("Emulator", screen_size.w as u32, screen_size.h as u32)
            .position_centered().maximized().fullscreen().opengl().build().context(codeloc!())?;
        if env.headless { window.hide(); }
        let mut canvas = window.into_canvas().index(opengl_driver_index).build().context(codeloc!())?;

        let texture_creator = canvas.texture_creator();
        let mut left_pattern_table = PatternTable::new(&texture_creator, PPUBus::get_left_pattern_table_address_range())?;
        let mut right_pattern_table = PatternTable::new(&texture_creator, PPUBus::get_right_pattern_table_address_range())?;

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return Ok(()); }

            if ppu.bus.palette.was_recently_changed()
            {
                left_pattern_table.refresh_textures( &ppu.bus)?;
                right_pattern_table.refresh_textures(&ppu.bus)?;
            }

            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            if !env.headless && env.debugger.should_render_pattern_tables_for_debugging
            {
                for i in 0..left_pattern_table.len()
                {
                    let texture = left_pattern_table.get(i);
                    let dest = Rect::new((i*80) as i32, 20, 80, 80);
                    canvas.copy(texture, None, Some(dest))
                        .map_err(|msg|anyhow!(msg.clone())).context(codeloc!())?;
                }
            }

            canvas.present();

            let mut event_pump = sdl.event_pump().map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
            for event in event_pump.poll_iter()
            {
                match event
                {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        env.is_shutting_down.store(true, Ordering::Relaxed);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
