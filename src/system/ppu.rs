use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use itertools::Itertools;
use sdl2::event::{Event, WindowEvent};
use crate::codeloc;
use crate::system::{address, byte};
use crate::system::debugger::LoggingOptions;
use crate::system::joystick::Joystick;
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::clock::PPUClock;
use crate::system::ppu::flags::control_flags::PPUControlFlags;
use crate::system::ppu::flags::mask_flags::PPUMaskFlags;
use crate::system::ppu::flags::status_flags::PPUStatusFlags;
use crate::system::ppu::metrics::WindowMetrics;
use crate::system::ppu::oam::PPUOAM;
use crate::system::ppu::pattern_table::PatternTables;
use crate::system::ppu::rendering::PPURenderingPipeline;
use crate::system::ppu_channels::PPUToCPUChannels;

pub mod character_rom;
pub mod bus;
pub mod oam;
mod palette;
mod pattern_table;
mod flags;
mod vram;
mod metrics;
mod communication;
mod rendering;
mod clock;

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
    pub window_metrics : WindowMetrics,
    pub clock : PPUClock,
    is_second_scroll_write : bool,
    is_second_bus_pointer_write : bool,
    bus_pointer : address,
    oam_pointer : address,
}

pub struct PPURunEnvironment
{
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
    pub should_disable_video : bool,
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
            window_metrics: WindowMetrics::new(),
            clock: PPUClock::new(),
            is_second_scroll_write: false,
            is_second_bus_pointer_write: false,
            bus_pointer: 0,
            oam_pointer: 0,
        };
    }

    pub fn run(self : &mut PPU, env : PPURunEnvironment) -> Result<()>
    {
        if env.should_disable_video { return Ok(()) }
        let ppu = self;

        let sdl = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let video_subsystem = sdl.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let window = video_subsystem.window("Emulator", ppu.window_metrics.get_window_width(), ppu.window_metrics.get_window_height())
            .position_centered().resizable().opengl().build().context(codeloc!())?;
        let (opengl_driver_index, _) = sdl2::render::drivers().find_position(|d| d.name=="opengl").unwrap();
        let mut canvas = window.into_canvas().index(opengl_driver_index as u32).build().context(codeloc!())?;
        let texture_creator = canvas.texture_creator();
        let mut pattern_tables = PatternTables::new(&texture_creator).context(codeloc!())?;

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return Ok(()) }

            let ppu_clock_tick_result = ppu.clock.tick();
            if ppu_clock_tick_result.should_notify_vblank_status_ended
            {
                ppu.status_flags.has_vblank_started = false;
            }
            else if ppu_clock_tick_result.should_notify_vblank_status_started
            {
                if ppu.bus.palette.was_recently_changed()
                {
                    pattern_tables.left.refresh_textures(&ppu.bus).context(codeloc!())?;
                    pattern_tables.right.refresh_textures(&ppu.bus).context(codeloc!())?;
                }

                let pipeline = PPURenderingPipeline::start(&ppu, &pattern_tables, &mut canvas);
                pipeline.render_oam_background_sprites(&mut canvas);
                pipeline.render_nametable_background(&mut canvas);
                pipeline.render_oam_foreground_sprites(&mut canvas);
                pipeline.commit_rendering(&mut canvas);

                ppu.status_flags.has_vblank_started = true;
                ppu.cpu_channels.signal_vblank();
            }

            ppu.handle_read_commands_from_cpu();
            ppu.handle_write_commands_from_cpu();

            let mut event_pump = sdl.event_pump().map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
            for event in event_pump.poll_iter()
            {
                match event
                {
                    Event::KeyDown { keycode: Some(keycode), .. } => { ppu.joystick.on_key_down(keycode); }
                    Event::KeyUp { keycode: Some(keycode), .. } => { ppu.joystick.on_key_up(keycode); }
                    Event::Window { win_event: WindowEvent::Resized(w, h), .. } => { ppu.window_metrics.on_window_resized(w, h); }
                    Event::Quit { .. } => { env.is_shutting_down.store(true, Ordering::Relaxed); return Ok(()); }
                    _ => {}
                }
            }
        }
    }
}
