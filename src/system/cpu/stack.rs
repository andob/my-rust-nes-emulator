use crate::system::System;
use crate::type_alias::{byte, word};

const STACK_TOP_ADDRESS : word = 0x01FF;
const STACK_BOTTOM_ADDRESS : word = 0x0100;

pub struct CPUStack
{
    pointer : word
}

impl CPUStack
{
    pub fn new() -> CPUStack
    {
        return CPUStack { pointer: STACK_TOP_ADDRESS };
    }

    pub fn push(nes : &mut System, value : byte)
    {
        nes.ram.put(nes.cpu.stack.pointer, value);

        let new_stack_pointer = nes.cpu.stack.pointer-1;
        if new_stack_pointer >= STACK_BOTTOM_ADDRESS && new_stack_pointer <= STACK_TOP_ADDRESS
        {
            nes.cpu.stack.pointer = new_stack_pointer;
        }
    }

    pub fn pop(nes : &mut System) -> byte
    {
        let new_stack_pointer = nes.cpu.stack.pointer+1;
        if new_stack_pointer >= STACK_BOTTOM_ADDRESS && new_stack_pointer <= STACK_TOP_ADDRESS
        {
            if let Some(value) = nes.ram.get(new_stack_pointer)
            {
                nes.cpu.stack.pointer = new_stack_pointer;
                return *value;
            }
        }
        return 0;
    }

    pub fn get_pointer(self : &CPUStack) -> byte
    {
        return (self.pointer & 0xFF) as byte;
    }

    pub fn set_pointer(self : &mut CPUStack, raw_pointer : byte)
    {
        let new_pointer = STACK_BOTTOM_ADDRESS | raw_pointer as word;
        if new_pointer >= STACK_BOTTOM_ADDRESS && new_pointer <= STACK_TOP_ADDRESS
        {
            self.pointer = new_pointer;
        }
    }
}
