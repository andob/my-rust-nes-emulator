use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::opcodes::build_opcodes_map;
use crate::system::cpu::stack::CPUStack;
use crate::system::System;
use crate::type_alias::{byte, word};

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
    pub program_counter : word,
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
            program_counter: 0,
            flags: CPUFlags::from_byte(0),
        };
    }

    pub fn run(nes : &mut System)
    {
        let opcodes = build_opcodes_map();

        nes.cpu.program_counter = 0;

        while let Some(opcode_key) = CPU::next_byte_from_rom(nes)
        {
            if let Some(opcode) = opcodes.get(&opcode_key)
            {
                let argument = CPU::next_argument_from_rom(nes, &opcode.addressing_mode);
                println!("{} {:#04X}", opcode.name, argument.value);
                (opcode.lambda)(nes, argument.address, argument.value);
            }
            else
            {
                println!("Unknown opcode {}", opcode_key);
            }
        }
    }
}
