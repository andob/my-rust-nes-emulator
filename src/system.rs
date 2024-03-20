#![allow(non_camel_case_types)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use anyhow::{Context, Result};
use maplit2::hashmap;
use crate::codeloc;
use crate::system::apu::{APU, APURunEnvironment};
use crate::system::cpu::{CPU, CPUChannelsToOtherSystems, CPURunEnvironment};
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::debugger::{CPUDebugger, LoggingOptions};
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::{PPU, PPURunEnvironment};
use crate::system::rom::ROMParser;
use crate::system::test::Test;

mod cpu;
mod ppu;
mod debugger;
mod test;
mod rom;
pub mod apu_channels;
pub mod ppu_channels;
mod apu;
mod joystick;

pub type byte = u8;
pub type address = u16;
pub type mapper = u8;
pub type color = u32;

pub const DEFAULT_CHANNEL_SIZE : usize = 32;

#[macro_export]
macro_rules! address_from_high_low
{
    ($high : expr, $low : expr) => ((($high as address)<<8)|($low as address))
}

pub struct SystemStartArgs
{
    program_rom : ProgramROM,
    character_rom : CharacterROM,
    logging_options : LoggingOptions,
    cpu_debugger : CPUDebugger,
    should_disable_audio : bool,
    should_disable_video : bool,
    should_disable_interrupt_vectors : bool,
}

impl SystemStartArgs
{
    pub fn with_rom_bytes(rom_data : Box<[byte]>) -> Result<SystemStartArgs>
    {
        let parsed_rom = ROMParser::parse(rom_data).context(codeloc!())?;

        return Ok(SystemStartArgs
        {
            program_rom: parsed_rom.program_rom,
            character_rom: parsed_rom.character_rom,
            logging_options: LoggingOptions::defaults(),
            cpu_debugger: CPUDebugger::new(),
            should_disable_audio: false,
            should_disable_video: false,
            should_disable_interrupt_vectors: false,
        });
    }
}

pub struct System {}
impl System
{
    pub fn test() -> Test { Test{} }

    pub fn start(args : SystemStartArgs) -> Result<RunningSystem>
    {
        let is_shutting_down = Arc::new(AtomicBool::new(false));

        let cpu_run_environment = CPURunEnvironment
        {
            debugger: args.cpu_debugger,
            logging_options: args.logging_options.clone(),
            is_shutting_down: is_shutting_down.clone(),
            should_disable_interrupt_vectors: args.should_disable_interrupt_vectors,
        };

        let ppu_run_environment = PPURunEnvironment
        {
            logging_options: args.logging_options.clone(),
            is_shutting_down: is_shutting_down.clone(),
            should_disable_video: args.should_disable_video,
        };

        let apu_run_environment = APURunEnvironment
        {
            logging_options: args.logging_options.clone(),
            is_shutting_down: is_shutting_down.clone(),
            should_disable_audio: args.should_disable_audio,
        };

        let (cpu_to_ppu_channels, ppu_to_cpu_channels) =
            System::create_ppu_system_channels(args.logging_options.clone());

        let (cpu_to_apu_channels, apu_to_cpu_channels) =
            System::create_apu_system_channels(args.logging_options.clone());

        let cpu_to_other_systems_channels = CPUChannelsToOtherSystems
            { ppu_channels: cpu_to_ppu_channels, apu_channels: cpu_to_apu_channels };

        let join_handle = thread::spawn(move ||
        {
            let join_sub_handles = hashmap!
            {
                "CPU" => thread::spawn(move || CPU::new(args.program_rom, cpu_to_other_systems_channels).run(cpu_run_environment)),
                "PPU" => thread::spawn(move || PPU::new(args.character_rom, ppu_to_cpu_channels).run(ppu_run_environment).unwrap()),
                "APU" => thread::spawn(move || APU::new(apu_to_cpu_channels).run(apu_run_environment).unwrap()),
            };

            for (thread_name, join_sub_handle) in join_sub_handles
            {
                if args.logging_options.is_system_threads_shutdown_logging_enabled
                {
                    println!("[SYS] Awaiting {} thread for its shutdown...", thread_name);
                }

                join_sub_handle.join().unwrap();

                if args.logging_options.is_system_threads_shutdown_logging_enabled
                {
                    println!("[SYS] {} thread was shutdown!", thread_name);
                }
            }
        });

        return Ok(RunningSystem { join_handle, is_shutting_down });
    }
}

pub struct RunningSystem
{
    join_handle : JoinHandle<()>,
    is_shutting_down : Arc<AtomicBool>,
}

impl RunningSystem
{
    pub fn shutdown(self)
    {
        self.is_shutting_down.store(true, Ordering::Relaxed);
    }

    pub fn await_termination(self)
    {
        return self.join_handle.join().unwrap();
    }
}
