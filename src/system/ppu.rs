use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::codeloc;
use crate::system::byte;
use crate::system::channels::{CPUToPPUCommTarget, PPUToCPUChannels};
use crate::system::debugger::{LoggingOptions, PPUDebugger};
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::flags::control_flags::PPUControlFlags;
use crate::system::ppu::flags::mask_flags::PPUMaskFlags;
use crate::system::ppu::flags::status_flags::PPUStatusFlags;
use crate::system::ppu::oam::PPUOAM;
use crate::system::ppu::pattern_table::PatternTable;

pub mod character_rom;
pub mod bus;
pub mod oam;
mod palette;
mod pattern_table;
mod flags;

pub struct PPU
{
    pub status_flags : PPUStatusFlags,
    pub control_flags : PPUControlFlags,
    pub mask_flags : PPUMaskFlags,
    pub scroll_x : byte,
    pub scroll_y : byte,
    pub bus : PPUBus,
    pub oam : PPUOAM,
    pub cpu_channels : PPUToCPUChannels,
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
    pub fn new(character_rom : CharacterROM, channels : PPUToCPUChannels) -> PPU
    {
        return PPU
        {
            status_flags: PPUStatusFlags::new(),
            control_flags: PPUControlFlags::new(),
            mask_flags: PPUMaskFlags::new(),
            scroll_x: 0,
            scroll_y: 0,
            bus: PPUBus::new(character_rom),
            oam: PPUOAM::new(),
            cpu_channels: channels,
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
        let mut window = video_subsystem.window("Emulator", (screen_size.w/4) as u32, (screen_size.h/4) as u32)
            .position_centered().opengl().build().context(codeloc!())?;
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
            canvas.present();

            if let Ok(target) = ppu.cpu_channels.get_read_command_from_cpu()
            {
                ppu.cpu_channels.respond_to_read_command_from_cpu(target, match target
                {
                    CPUToPPUCommTarget::ControlFlags => { ppu.control_flags.to_byte() }
                    CPUToPPUCommTarget::MaskFlags => { ppu.mask_flags.to_byte() }
                    CPUToPPUCommTarget::StatusFlags => { ppu.status_flags.to_byte() }
                    CPUToPPUCommTarget::OAMAddress => {0} //todo implement this
                    CPUToPPUCommTarget::OAMData => {0} //todo implement this
                    CPUToPPUCommTarget::ScrollPosition => {0} //todo implement this
                    CPUToPPUCommTarget::BusAddress => {0} //todo implement this
                    CPUToPPUCommTarget::BusData => {0} //todo implement this
                    CPUToPPUCommTarget::OAM_DMA => {0} //todo implement this
                    CPUToPPUCommTarget::Unknown => {0} //todo implement this
                });
            }

            match ppu.cpu_channels.get_write_command_from_cpu()
            {
                Ok((CPUToPPUCommTarget::ControlFlags, value)) => { ppu.control_flags = PPUControlFlags::from_byte(value); }
                Ok((CPUToPPUCommTarget::MaskFlags, value)) => { ppu.mask_flags = PPUMaskFlags::from_byte(value); }
                Ok((CPUToPPUCommTarget::StatusFlags, value)) => { ppu.status_flags = PPUStatusFlags::from_byte(value); }
                Ok((CPUToPPUCommTarget::OAMAddress, value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::OAMData, value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::ScrollPosition, value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::BusAddress, value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::BusData, value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::OAM_DMA, value)) => {} //todo implement this
                _ => {}
            }

            thread::sleep(Duration::from_secs(1));

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
