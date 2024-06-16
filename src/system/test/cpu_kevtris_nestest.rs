use std::collections::VecDeque;
use anyhow::{anyhow, Context, Result};
use substring::Substring;
use crate::codeloc;
use crate::system::{address, byte, System, SystemStartArgs};
use crate::system::cpu::flags::CPUFlags;
use crate::system::debugger::CPUState;

pub fn test_cpu_with_kevtris_nestest() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/cpu_kevtris_nestest.nes");

    let good_output_string = include_str!("cpu_kevtris_nestest/good_output.log").to_string();
    let mut good_output = parse_good_output(good_output_string).context(codeloc!())?;

    let (cpu_state_sender, cpu_state_receiver) = flume::unbounded::<CPUState>();

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;

    start_args.should_disable_video = true;
    start_args.should_disable_interrupt_vectors = true;
    start_args.cpu_debugger.cpu_state_watcher = Some(cpu_state_sender);

    let running_system = System::start(start_args).context(codeloc!())?;

    let mut previous_progress = 0u32;
    let max_progress = good_output.len() as f32;

    loop
    {
        let cpu_state = cpu_state_receiver.recv().context(codeloc!())?;

        let current_progress = ((1f32-(good_output.len() as f32)/max_progress)*100f32) as u32;
        if current_progress != previous_progress
        {
            println!("\n\n\n[CPU] PERCENT OF PASSED: {}%", current_progress);
            previous_progress = current_progress;
        }
        
        if let Some(good_line) = good_output.pop_front()
        {
            if let Err(error) = check_execution(&good_line, &cpu_state)
            {
                eprintln!("{:?}", error);
                running_system.shutdown();
                return Err(error.context(codeloc!()));
            }
        }
        else
        {
            println!("ALL TESTS PASSED!!!");
            running_system.shutdown();
            return Ok(());
        }
    }
}

#[allow(non_snake_case)]
struct GoodOutputLine
{
    raw : String,
    cpu_state : CPUState,
}

fn parse_good_output(raw : String) -> Result<VecDeque<GoodOutputLine>>
{
    let raw_offset = raw.substring(raw.find('\n').unwrap_or_default(), raw.len()-1).trim();
    return raw.lines().zip(raw_offset.lines()).map(|(previous_line, next_line)| Ok(GoodOutputLine
    {
        raw: format!("{}\n{}", previous_line, next_line),
        cpu_state: CPUState
        {
            A: byte::from_str_radix(&next_line[50..=51], 16).context(codeloc!())?,
            X: byte::from_str_radix(&next_line[55..=56], 16).context(codeloc!())?,
            Y: byte::from_str_radix(&next_line[60..=61], 16).context(codeloc!())?,
            stack_pointer: byte::from_str_radix(&next_line[71..=72], 16).context(codeloc!())?,
            program_counter: address::from_str_radix(&next_line[0..=3], 16).context(codeloc!())?,
            flags: CPUFlags::from_byte(byte::from_str_radix(&next_line[65..=66], 16).context(codeloc!())?),
        },
    })).collect();
}

fn check_execution(expected : &GoodOutputLine, actual_cpu_state : &CPUState) -> Result<()>
{
    if expected.cpu_state != *actual_cpu_state
    {
        if actual_cpu_state.A != expected.cpu_state.A
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong Accumulator!\nexpected={:#04X}, actual={:#04X}\n{}",
               expected.cpu_state.A, actual_cpu_state.A, expected.raw));
        }

        if actual_cpu_state.X != expected.cpu_state.X
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong X Index!\nexpected={:#04X}, actual={:#04X}\n{}",
               expected.cpu_state.X, actual_cpu_state.X, expected.raw));
        }

        if actual_cpu_state.Y != expected.cpu_state.Y
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong Y Index!\nexpected={:#04X}, actual={:#04X}\n{}",
               expected.cpu_state.Y, actual_cpu_state.Y, expected.raw));
        }

        if actual_cpu_state.stack_pointer != expected.cpu_state.stack_pointer
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong Stack Pointer!\nexpected={:#04X}, actual={:#04X}\n{}",
               expected.cpu_state.stack_pointer, actual_cpu_state.stack_pointer, expected.raw));
        }

        if actual_cpu_state.program_counter != expected.cpu_state.program_counter
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong Program Counter!\nexpected={:#06X}, actual={:#06X}\n{}",
               expected.cpu_state.program_counter, actual_cpu_state.program_counter, expected.raw));
        }

        if expected.cpu_state.flags != actual_cpu_state.flags
        {
            return Err(anyhow!("[CPU] Test Failed! Wrong CPU Flags!\nexpected={}, actual={}\n{}",
               expected.cpu_state.flags, actual_cpu_state.flags, expected.raw));
        }
    }
    return Ok(());
}
