use crate::log_cpu_opcode;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::opcodes::build_opcodes_slice;
use crate::system::cpu::stack::CPUStack;
use crate::system::{address, byte, Debugger, System};
use crate::system::cpu::clock::CPUClock;
use crate::system::cpu::interrupts::CPUInterrupts;

mod opcodes;
mod program_iterator;
mod stack;
mod flags;
mod clock;
mod interrupts;

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
}

impl CPU
{
    pub fn new() -> CPU
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
        };
    }

    pub fn run(nes : &mut System, mut debugger : Box<dyn Debugger>)
    {
        CPUInterrupts::reset(nes);

        let opcodes = build_opcodes_slice();

        loop
        {
            nes.cpu.clock.notify_cpu_cycle_started();

            let opcode_key = CPU::next_byte_from_rom(nes);
            let opcode = &opcodes[opcode_key as usize];
            let (address, value) = CPU::next_argument_from_rom(nes, &opcode);

            //comment the following line to speed up CPU thread
            log_cpu_opcode!("[CPU] {} {:#06X} {:#04X}", opcode.name, address, value);

            debugger.before_cpu_opcode(nes);
            (opcode.lambda)(nes, address, value);
            debugger.after_cpu_opcode(nes);

            nes.cpu.clock.notify_cpu_cycle_stopped(&opcode);
        }
    }
}
