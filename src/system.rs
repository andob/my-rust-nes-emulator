use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::cpu::{CPU, CPURunEnvironment};
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::debugger::{CPUDebugger, LoggingOptions};
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::rom::ROMParser;
use crate::system::test::Test;

mod cpu;
mod ram;
mod ppu;
mod debugger;
mod test;
mod rom;

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type address = u16;

#[macro_export]
macro_rules! address_from_high_low
{
    ($high : expr, $low : expr) => ((($high as address)<<8)|($low as address))
}

pub struct SystemStartArgs
{
    program_rom : ProgramROM,
    _character_rom : CharacterROM,
    logging_options : LoggingOptions,
    cpu_debugger : CPUDebugger,
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
            program_rom: program_rom, _character_rom: character_rom,
            logging_options: LoggingOptions::defaults(),
            cpu_debugger: CPUDebugger::new(),
            headless: false,
        };
    }
}

pub struct System {}
impl System
{
    pub fn test() -> Test { Test{} }

    pub fn start(args : SystemStartArgs) -> RunningSystem
    {
        let is_shutting_down = Arc::new(AtomicBool::new(false));

        let cpu_run_environment = CPURunEnvironment
        {
            debugger: args.cpu_debugger,
            logging_options: args.logging_options.clone(),
            is_shutting_down: is_shutting_down.clone(),
        };

        let join_handle = if args.headless
        {
            thread::spawn(move || CPU::new(args.program_rom).run(cpu_run_environment))
        }
        else
        {
            thread::spawn(move ||
            {
                thread::spawn(move || CPU::new(args.program_rom).run(cpu_run_environment)).join().unwrap();
            })
        };

        return RunningSystem { join_handle, is_shutting_down };
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

    pub fn join(self)
    {
        return self.join_handle.join().unwrap();
    }
}
