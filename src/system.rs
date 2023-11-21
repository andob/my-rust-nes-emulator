#![allow(non_camel_case_types)]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::channels::create_system_channels;
use crate::system::cpu::{CPU, CPURunEnvironment};
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::debugger::{CPUDebugger, LoggingOptions, PPUDebugger};
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::ppu::{PPU, PPURunEnvironment};
use crate::system::rom::ROMParser;
use crate::system::test::Test;

mod cpu;
mod ram;
mod ppu;
mod debugger;
mod test;
mod rom;
mod channels;

pub type byte = u8;
pub type mapper = u8;
pub type address = u16;
pub type color = u32;

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
    ppu_debugger : PPUDebugger,
    headless : bool,
}

impl SystemStartArgs
{
    pub fn with_rom_bytes(rom_data : Box<[byte]>) -> Result<SystemStartArgs>
    {
        let (program_rom, character_rom) = ROMParser::parse(rom_data).context(codeloc!())?;

        return Ok(SystemStartArgs::with_parsed_rom(program_rom, character_rom));
    }

    pub fn with_parsed_rom(program_rom : ProgramROM, character_rom : CharacterROM) -> SystemStartArgs
    {
        return SystemStartArgs
        {
            program_rom: program_rom, character_rom: character_rom,
            logging_options: LoggingOptions::defaults(),
            cpu_debugger: CPUDebugger::new(),
            ppu_debugger: PPUDebugger::new(),
            headless: false,
        };
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
        };

        let ppu_run_environment = PPURunEnvironment
        {
            debugger: args.ppu_debugger,
            logging_options: args.logging_options.clone(),
            headless: args.headless,
            is_shutting_down: is_shutting_down.clone(),
        };

        let join_handle = thread::spawn(move ||
        {
            let channels = create_system_channels(args.logging_options.clone());

            let join_sub_handles =
            [
                thread::spawn(move || CPU::new(args.program_rom, channels.cpu_to_ppu_channels).run(cpu_run_environment)),
                thread::spawn(move || PPU::new(args.character_rom, channels.ppu_to_cpu_channels).run(ppu_run_environment).unwrap()),
            ];

            for join_sub_handle in join_sub_handles
            {
                join_sub_handle.join().unwrap();
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
