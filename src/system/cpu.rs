use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::opcodes::build_opcodes_slice;
use crate::system::cpu::stack::CPUStack;
use crate::system::{address, byte, CPUDebugger};
use crate::system::apu_channels::CPUToAPUChannels;
use crate::system::cpu::clock::CPUClock;
use crate::system::cpu::bus::CPUBus;
use crate::system::cpu::interrupts::CPUInterrupts;
use crate::system::cpu::program_iterator::CPUProgramIterator;
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::debugger::LoggingOptions;
use crate::system::ppu_channels::CPUToPPUChannels;

mod opcodes;
mod program_iterator;
mod stack;
pub mod flags;
mod clock;
mod interrupts;
pub mod bus;
pub mod program_rom;

#[allow(non_snake_case)]
pub struct CPU
{
    pub A : byte, //Accumulator register
    pub X : byte, //X index register
    pub Y : byte, //Y index register
    pub stack : CPUStack,
    pub clock : CPUClock,
    pub program_counter : address,
    pub flags : CPUFlags,
    pub bus : CPUBus,
}

pub struct CPURunEnvironment
{
    pub debugger : CPUDebugger,
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
}

pub struct CPUChannelsToOtherSystems
{
    pub ppu_channels : CPUToPPUChannels,
    pub apu_channels : CPUToAPUChannels,
}

impl CPU
{
    pub fn new(program_rom : ProgramROM, channels : CPUChannelsToOtherSystems) -> CPU
    {
        return CPU
        {
            A: 0,
            X: 0,
            Y: 0,
            stack: CPUStack::new(),
            clock: CPUClock::new(),
            program_counter: 0,
            flags: CPUFlags::from_byte(0),
            bus: CPUBus::new(program_rom, channels),
        };
    }

    pub fn run(self : &mut CPU, env : CPURunEnvironment)
    {
        let cpu = self;
        CPUInterrupts::hardware_reset(cpu);

        let opcodes = build_opcodes_slice();

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return; }

            cpu.clock.notify_cpu_cycle_started();

            let opcode_key = CPUProgramIterator::next_byte_from_rom(cpu);
            let opcode = &opcodes[opcode_key as usize];
            let (address, value) = CPUProgramIterator::next_argument_from_rom(cpu, &opcode);

            if env.logging_options.is_cpu_opcode_logging_enabled
            {
                println!("[CPU] {} {:#06X} {:#04X}", opcode.name, address, value);
            }

            (opcode.lambda)(cpu, address, value);

            env.debugger.notify_cpu_state_to_watchers(cpu);

            cpu.clock.notify_cpu_cycle_stopped(&opcode, &env.logging_options);

            if cpu.bus.channels.ppu_channels.is_ppu_signaling_that_vblank_has_started()
            {
               CPUInterrupts::hardware_nmi(cpu);
            }

            if cpu.bus.channels.apu_channels.is_apu_signaling_that_frame_has_ended()
            {
                CPUInterrupts::hardware_irq(cpu);
            }
        }
    }
}
