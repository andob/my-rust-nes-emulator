use crate::address_from_high_low;
use crate::system::{address, byte};
use crate::system::cpu::CPU;

const STACK_TOP_ADDRESS : address = 0x0200;
const STACK_BOTTOM_ADDRESS : address = 0x0100;

pub struct CPUStack
{
    pointer : address
}

impl CPUStack
{
    pub fn new() -> CPUStack
    {
        return CPUStack { pointer: STACK_TOP_ADDRESS };
    }

    pub fn push_byte(cpu : &mut CPU, value : byte)
    {
        cpu.bus.put(cpu.stack.pointer, value);

        let new_stack_pointer = cpu.stack.pointer-1;
        if new_stack_pointer >= STACK_BOTTOM_ADDRESS && new_stack_pointer <= STACK_TOP_ADDRESS
        {
            cpu.stack.pointer = new_stack_pointer;
        }
    }

    pub fn push_address(cpu : &mut CPU, address : address)
    {
        CPUStack::push_byte(cpu, ((address>>8)&0xFF) as byte);
        CPUStack::push_byte(cpu, (address&0xFF) as byte);
    }

    pub fn pop_byte(cpu : &mut CPU) -> Option<byte>
    {
        let new_stack_pointer = cpu.stack.pointer+1;
        if new_stack_pointer >= STACK_BOTTOM_ADDRESS && new_stack_pointer <= STACK_TOP_ADDRESS
        {
            let value = cpu.bus.get(new_stack_pointer);
            cpu.stack.pointer = new_stack_pointer;
            return Some(value);
        }
        return None;
    }

    pub fn pop_address(cpu : &mut CPU) -> Option<address>
    {
        if let Some(low) = CPUStack::pop_byte(cpu)
        {
            if let Some(high) = CPUStack::pop_byte(cpu)
            {
                return Some(address_from_high_low!(high, low));
            }
        }
        return None;
    }

    pub fn get_pointer(self : &CPUStack) -> byte
    {
        return (self.pointer & 0x00FF) as byte;
    }

    pub fn set_pointer(self : &mut CPUStack, raw_pointer : byte)
    {
        let new_pointer = STACK_BOTTOM_ADDRESS | raw_pointer as address;
        if new_pointer >= STACK_BOTTOM_ADDRESS && new_pointer <= STACK_TOP_ADDRESS
        {
            self.pointer = new_pointer;
        }
    }
}
