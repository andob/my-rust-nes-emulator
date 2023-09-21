use std::collections::HashMap;
use maplit2::hashmap;
use crate::system::cpu::{AddressingMode, CPU};

pub struct Opcode
{
    pub name : String,
    pub addressing_mode : AddressingMode,
    pub lambda : fn(&mut CPU, u8) -> (),
    pub expected_time : u8,
}

macro_rules! opcode
{
    ($name : expr, $expected_time : expr, $addressing_mode : expr) =>
    {
        {
            Opcode
            {
                name: stringify!($name).to_uppercase(),
                lambda: |cpu, arg| $name(cpu,arg),
                expected_time: $expected_time,
                addressing_mode: $addressing_mode,
            }
        }
    }
}

pub fn build_opcodes_map() -> HashMap<u8, Opcode>
{
    return hashmap!
    {
        0x69u8 => opcode!(adc, 2, AddressingMode::Immediate),
        0x6Du8 => opcode!(adc, 4, AddressingMode::Absolute),
        0x7Du8 => opcode!(adc, 4, AddressingMode::AbsoluteXIndexed),
        0x79u8 => opcode!(adc, 4, AddressingMode::AbsoluteYIndexed),
        0x65u8 => opcode!(adc, 3, AddressingMode::ZeroPage),
        0x75u8 => opcode!(adc, 4, AddressingMode::ZeroPageXIndexed),
        0x61u8 => opcode!(adc, 6, AddressingMode::IndirectX),
        0x71u8 => opcode!(adc, 5, AddressingMode::IndirectY),
    };
}

fn adc(cpu : &mut CPU, arg : u8)
{
    //add with carry
}
