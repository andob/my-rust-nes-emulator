use anyhow::{anyhow, Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use itertools::Itertools;
use sdl2::event::{Event, WindowEvent};
use crate::codeloc;
use crate::system::address;
use crate::system::debugger::{LoggingOptions, PPUDebugger};
use crate::system::input::InputSubsystem;
use crate::system::ppu::bus::PPUBus;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::clock::PPUClock;
use crate::system::ppu::flags::bus_pointer_latch::PPUBusPointerLatch;
use crate::system::ppu::flags::control_flags::PPUControlFlags;
use crate::system::ppu::flags::mask_flags::PPUMaskFlags;
use crate::system::ppu::flags::scroll_flags::PPUScrollFlags;
use crate::system::ppu::flags::status_flags::PPUStatusFlags;
use crate::system::ppu::metrics::WindowMetrics;
use crate::system::ppu::oam::PPUOAM;
use crate::system::ppu::pattern_tables::PatternTables;
use crate::system::ppu::rendering::PPURenderingPipeline;
use crate::system::ppu_channels::PPUToCPUChannels;

pub mod character_rom;
pub mod bus;
pub mod oam;
mod palette;
mod pattern_tables;
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
    pub scroll : PPUScrollFlags,
    pub bus : PPUBus,
    pub oam : PPUOAM,
    pub input_subsystem : InputSubsystem,
    pub cpu_channels : PPUToCPUChannels,
    pub window_metrics : WindowMetrics,
    pub clock : PPUClock,
    bus_pointer : PPUBusPointerLatch,
    oam_pointer : address,
}

pub struct PPURunEnvironment
{
    pub debugger : PPUDebugger,
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
    pub should_disable_video : bool,
    pub window_title : String,
}

impl PPU
{
    pub fn new(character_rom : CharacterROM, channels : PPUToCPUChannels) -> PPU
    {
        let character_rom_hash = character_rom.hash();

        return PPU
        {
            status_flags: PPUStatusFlags::new(),
            control_flags: PPUControlFlags::new(),
            mask_flags: PPUMaskFlags::new(),
            scroll: PPUScrollFlags::new(&character_rom_hash),
            bus: PPUBus::new(character_rom),
            oam: PPUOAM::new(),
            input_subsystem: InputSubsystem::new(),
            cpu_channels: channels,
            window_metrics: WindowMetrics::new(),
            clock: PPUClock::new(&character_rom_hash),
            bus_pointer: PPUBusPointerLatch::new(),
            oam_pointer: 0,
        };
    }

    pub fn run(self : &mut PPU, mut env : PPURunEnvironment) -> Result<()>
    {
        if env.should_disable_video { return Ok(()) }
        let ppu = self;

        let sdl = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
        let video_subsystem = sdl.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;

        let window = video_subsystem.window(env.window_title.as_str(),
            ppu.window_metrics.get_window_width(), ppu.window_metrics.get_window_height())
            .position_centered().resizable().opengl().build().context(codeloc!())?;

        let (opengl_driver_index, _) = sdl2::render::drivers().find_position(|d| d.name=="opengl").unwrap();
        let mut canvas = window.into_canvas().index(opengl_driver_index as u32).accelerated().build().context(codeloc!())?;

        let texture_creator = canvas.texture_creator();
        let mut pattern_tables = PatternTables::new(&texture_creator).context(codeloc!())?;

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return Ok(()) }

            let ppu_clock_tick_result = ppu.clock.tick();
            if ppu_clock_tick_result.should_notify_visible_scanline_reached()
            {
                //todo implement sprite zero hit algorithm
                if ppu.mask_flags.should_show_sprites && ppu.mask_flags.should_show_background
                {
                    ppu.status_flags.is_sprite_zero_hit = true;
                }
            }
            else if ppu_clock_tick_result.should_notify_vblank_started()
            {
                if ppu.bus.palette.was_recently_changed()
                {
                    pattern_tables.left.refresh_textures(&ppu.bus).context(codeloc!())?;
                    pattern_tables.right.refresh_textures(&ppu.bus).context(codeloc!())?;
                }

                if let Some(mut pipeline) = PPURenderingPipeline::start(ppu, &env, &pattern_tables, &mut canvas)
                {
                    pipeline.render_background_sprites_from_oam();
                    pipeline.render_background_from_nametables();
                    pipeline.render_foreground_sprites_from_oam();
                    pipeline.end();
                }

                ppu.status_flags.has_vblank_started = true;
                if ppu.control_flags.is_nmi_enabled
                {
                    ppu.cpu_channels.signal_vblank();
                }
            }
            else if ppu_clock_tick_result.should_notify_vblank_ended()
            {
                ppu.status_flags.has_vblank_started = false;
                ppu.status_flags.is_sprite_zero_hit = false;
            }

            ppu.handle_read_commands_from_cpu();
            ppu.handle_write_commands_from_cpu();

            ppu.input_subsystem.handle_physical_joystick_events();

            let mut sdl_event_pump = sdl.event_pump().map_err(|e|anyhow!(e.clone())).context(codeloc!())?;
            for event in sdl_event_pump.poll_iter()
            {
                match event
                {
                    Event::KeyDown { keycode: Some(keycode), .. } => { ppu.input_subsystem.handle_physical_keyboard_keydown(keycode, &mut env); }
                    Event::KeyUp { keycode: Some(keycode), .. } => { ppu.input_subsystem.handle_physical_keyboard_keyup(keycode, &mut env); }
                    Event::Window { win_event: WindowEvent::Resized(w, h), .. } => { ppu.window_metrics.on_window_resized(w, h); }
                    Event::Quit { .. } => { env.is_shutting_down.store(true, Ordering::Relaxed); return Ok(()); }
                    _ => {}
                }
            }
        }
    }
}
