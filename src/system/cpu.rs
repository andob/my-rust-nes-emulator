use crate::system::cpu::opcodes::build_opcodes_map;
use crate::system::System;

mod opcodes;
mod program_iterator;

#[allow(non_snake_case)]
pub struct CPU
{
    pub A : u8, //Accumulator register
    pub X : u8, //X index register
    pub Y : u8, //Y index register
    pub SP : u16, //Stack Pointer
    pub program_counter : usize,
    pub flags : CPUFlags,
}

pub struct CPUFlags
{
    pub negative : bool,
    pub overflow : bool,
    pub _break : bool,
    pub decimal : bool,
    pub interrupt : bool,
    pub zero : bool,
    pub carry : bool,
}

pub enum AddressingMode
{
    Implied,
    Immediate,
    Absolute,
    AbsoluteXIndexed,
    AbsoluteYIndexed,
    ZeroPage,
    ZeroPageXIndexed,
    ZeroPageYIndexed,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
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
            SP: 0x0100,
            program_counter: 0,
            flags: CPUFlags
            {
                negative: false,
                overflow: false,
                _break: false,
                decimal: false,
                interrupt: false,
                zero: false,
                carry: false,
            },
        };
    }

    pub fn run(nes : &mut System)
    {
        let opcodes = build_opcodes_map();

        nes.cpu.program_counter = 0;

        while let Some(opcode_key) = CPU::next_u8_from_rom(nes)
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
