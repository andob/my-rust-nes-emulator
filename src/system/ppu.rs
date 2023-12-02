use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::pixels::Color;
use crate::codeloc;
use crate::system::byte;
use crate::system::debugger::{LoggingOptions, PPUDebugger};
use crate::system::joystick::Joystick;
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::flags::control_flags::PPUControlFlags;
use crate::system::ppu::flags::mask_flags::PPUMaskFlags;
use crate::system::ppu::flags::status_flags::PPUStatusFlags;
use crate::system::ppu::oam::PPUOAM;
use crate::system::ppu::pattern_table::PatternTable;
use crate::system::ppu_channels::{CPUToPPUCommTarget, PPUToCPUChannels};

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
    pub joystick : Joystick,
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
            joystick: Joystick::new(),
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

        let mut left_pattern_table = PatternTable::new(&texture_creator,
            PPUBus::get_left_pattern_table_address_range()).context(codeloc!())?;

        let mut right_pattern_table = PatternTable::new(&texture_creator,
            PPUBus::get_right_pattern_table_address_range()).context(codeloc!())?;

        let fps = 16666667u128; //60FPS
        let mut clock_total_elapsed = 0u128;
        let mut clock_tick = Instant::now();

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return Ok(()); }

            //todo implement a proper, more precise render clock + VBLANK algorithm
            clock_total_elapsed += clock_tick.elapsed().as_nanos();
            let should_render = clock_total_elapsed >= fps;
            if should_render
            {
                clock_tick = Instant::now();
                clock_total_elapsed = 0;

                if !env.headless && ppu.bus.palette.was_recently_changed()
                {
                    left_pattern_table.refresh_textures(&ppu.bus).context(codeloc!())?;
                    right_pattern_table.refresh_textures(&ppu.bus).context(codeloc!())?;
                }

                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                canvas.present();

                ppu.status_flags.has_vblank_started = true;
                ppu.cpu_channels.signal_vblank();
            }

            if let Ok(target) = ppu.cpu_channels.get_read_command_from_cpu()
            {
                ppu.cpu_channels.respond_to_read_command_from_cpu(target, match target
                {
                    CPUToPPUCommTarget::ControlFlags => ppu.control_flags.to_byte(),
                    CPUToPPUCommTarget::MaskFlags => ppu.mask_flags.to_byte(),
                    CPUToPPUCommTarget::StatusFlags => ppu.status_flags.to_byte(),
                    CPUToPPUCommTarget::OAMAddress => 0, //todo implement this
                    CPUToPPUCommTarget::OAMData => 0, //todo implement this
                    CPUToPPUCommTarget::ScrollPosition => 0, //todo implement this
                    CPUToPPUCommTarget::BusAddress => 0, //todo implement this
                    CPUToPPUCommTarget::BusData => 0, //todo implement this
                    CPUToPPUCommTarget::OAM_DMA => 0, //todo implement this
                    CPUToPPUCommTarget::Joystick => ppu.joystick.read_pressed_key(),
                    CPUToPPUCommTarget::Unknown => 0, //todo implement this
                });
            }

            match ppu.cpu_channels.get_write_command_from_cpu()
            {
                Ok((CPUToPPUCommTarget::ControlFlags, value)) => { ppu.control_flags = PPUControlFlags::from_byte(value); }
                Ok((CPUToPPUCommTarget::MaskFlags, value)) => { ppu.mask_flags = PPUMaskFlags::from_byte(value); }
                Ok((CPUToPPUCommTarget::OAMAddress, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::OAMData, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::ScrollPosition, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::BusAddress, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::BusData, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::OAM_DMA, _value)) => {} //todo implement this
                Ok((CPUToPPUCommTarget::Joystick, value)) => { ppu.joystick.set_strobe_enabled(value&1==1); }
                _ => {}
            }

            let mut event_pump = sdl.event_pump().map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
            for event in event_pump.poll_iter()
            {
                match event
                {
                    Event::KeyDown { keycode: Some(keycode), .. } => { ppu.joystick.on_key_down(keycode); }
                    Event::KeyUp { keycode: Some(keycode), .. } => { ppu.joystick.on_key_up(keycode); }
                    Event::Quit { .. } => { env.is_shutting_down.store(true, Ordering::Relaxed); return Ok(()); }
                    _ => {}
                }
            }
        }
    }
}
