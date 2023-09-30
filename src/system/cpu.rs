use std::thread;
use std::time::Duration;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::opcodes::build_opcodes_map;
use crate::system::cpu::stack::CPUStack;
use crate::system::{address, byte, Debugger, System};

mod opcodes;
mod program_iterator;
mod stack;
mod flags;

#[allow(non_snake_case)]
pub struct CPU
{
    pub A : byte, //Accumulator register
    pub X : byte, //X index register
    pub Y : byte, //Y index register
    pub stack : CPUStack,
    pub program_counter : address,
    pub flags : CPUFlags,
    pub lag_factor_in_nanos : u64,
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
            program_counter: 0,
            flags: CPUFlags
            {
                negative: false,
                overflow: false,
                reserved: false,
                _break: true,
                decimal: false,
                interrupt: false,
                zero: false,
                carry: false,
            },
            lag_factor_in_nanos: 1,
        };
    }

    pub fn run(nes : &mut System, debugger : Box<dyn Debugger>)
    {
        let opcodes = build_opcodes_map();

        while let Some(opcode_key) = CPU::next_byte_from_rom(nes)
        {
            let opcode_program_counter = nes.cpu.program_counter-1;
            if let Some(opcode) = opcodes.get(&opcode_key)
            {
                let (address, value) = CPU::next_argument_from_rom(nes, &opcode.addressing_mode);

                let opcode_description = format!("{:#06X} {} {:#06X} {:#04X}",
                     opcode_program_counter, opcode.name, address, value);
                debugger.before_cpu_tick(nes, &opcode_description);

                (opcode.lambda)(nes, address, value);

                thread::sleep(Duration::from_micros((opcode.expected_time as u64)*nes.cpu.lag_factor_in_nanos));
                debugger.after_cpu_tick(nes, &opcode_description);
            }
            else
            {
                panic!("Unknown opcode {:#04X}", opcode_key);
            }
        }
    }
}
