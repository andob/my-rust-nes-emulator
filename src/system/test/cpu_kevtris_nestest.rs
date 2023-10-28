use anyhow::Result;
use substring::Substring;
use std::collections::VecDeque;
use std::process;
use crate::log_test_result;
use crate::system::debugger::Debugger;
use crate::system::{address, byte, System};

pub fn test_cpu_with_kevtris_nestest() -> Result<()>
{
    let rom_bytes = *include_bytes!("cpu_kevtris_nestest/nestest.nes");

    let good_output_string = include_str!("cpu_kevtris_nestest/good_output.log").to_string();
    let good_output = parse_good_output(good_output_string)?;

    let debugger = KevtrisNestestDebugger::new(good_output);
    let mut nes = System::with_rom_bytes(Box::new(rom_bytes))?;
    nes.run_with_debugger(Box::new(debugger));

    return Ok(());
}

#[allow(non_snake_case)]
struct GoodOutputLine
{
    raw : String,
    A : byte,
    X : byte,
    Y : byte,
    stack_pointer : byte,
    program_counter : address,
    flags : byte,
}

fn parse_good_output(raw : String) -> Result<VecDeque<GoodOutputLine>>
{
    let raw_offset = raw.substring(raw.find('\n').unwrap_or_default(), raw.len()-1).trim();
    return raw.lines().zip(raw_offset.lines()).map(|(previous_line, next_line)| Ok(GoodOutputLine
    {
        raw: format!("{}\n{}", previous_line, next_line),
        A: byte::from_str_radix(&next_line[50..=51], 16)?,
        X: byte::from_str_radix(&next_line[55..=56], 16)?,
        Y: byte::from_str_radix(&next_line[60..=61], 16)?,
        stack_pointer: byte::from_str_radix(&next_line[71..=72], 16)?,
        program_counter: address::from_str_radix(&next_line[0..=3], 16)?,
        flags: byte::from_str_radix(&next_line[65..=66], 16)?,
    })).collect();
}

struct KevtrisNestestDebugger
{
    good_output : VecDeque<GoodOutputLine>,
    current_progress : usize,
    max_progress : usize,
}

impl KevtrisNestestDebugger
{
    fn new(good_output : VecDeque<GoodOutputLine>) -> KevtrisNestestDebugger
    {
        let max_progress = good_output.len();
        return KevtrisNestestDebugger { good_output, current_progress:0, max_progress };
    }
}

impl Debugger for KevtrisNestestDebugger
{
    fn before_cpu_opcode(&mut self, _nes : &mut System)
    {
        let percent = 1f32-((self.good_output.len() as f32)/(self.max_progress as f32));
        let previous_progress = self.current_progress;
        self.current_progress = (100f32*percent) as usize;
        if previous_progress != self.current_progress
        {
            log_test_result!("\n\n\n[CPU] PERCENT OF PASSED: {}%", self.current_progress);
        }
    }

    fn after_cpu_opcode(&mut self, nes : &mut System)
    {
        if self.good_output.is_empty()
        {
            log_test_result!("ALL TESTS PASSED!!!");
            process::exit(exitcode::OK);
        }

        if let Some(good_line) = self.good_output.pop_front()
        {
            if nes.cpu.A != good_line.A
            {
                panic!("[CPU] Test Failed! Wrong Accumulator!\nexpected={:#04X}, actual={:#04X}\n{}",
                   good_line.A, nes.cpu.A, good_line.raw);
            }

            if nes.cpu.X != good_line.X
            {
                panic!("[CPU] Test Failed! Wrong X Index!\nexpected={:#04X}, actual={:#04X}\n{}",
                   good_line.X, nes.cpu.X, good_line.raw);
            }

            if nes.cpu.Y != good_line.Y
            {
                panic!("[CPU] Test Failed! Wrong Y Index!\nexpected={:#04X}, actual={:#04X}\n{}",
                   good_line.Y, nes.cpu.Y, good_line.raw);
            }

            if nes.cpu.stack.get_pointer() != good_line.stack_pointer
            {
                panic!("[CPU] Test Failed! Wrong Stack Pointer!\nexpected={:#04X}, actual={:#04X}\n{}",
                   good_line.stack_pointer, nes.cpu.stack.get_pointer(), good_line.raw);
            }

            if nes.cpu.program_counter != good_line.program_counter
            {
                panic!("[CPU] Test Failed! Wrong Program Counter!\nexpected={:#06X}, actual={:#06X}\n{}",
                   good_line.program_counter, nes.cpu.program_counter, good_line.raw);
            }

            let expected_flags = nes.cpu.flags.clone_from_byte(good_line.flags);
            if nes.cpu.flags != expected_flags
            {
                panic!("[CPU] Test Failed! Wrong CPU Flags!\nexpected=[{:#04X}]{}, actual=[{:#04X}]{}\n{}",
                   good_line.flags, expected_flags, nes.cpu.flags.to_byte(), nes.cpu.flags, good_line.raw);
            }
        }
    }
}
