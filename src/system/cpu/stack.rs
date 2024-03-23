use crate::address_from_high_low;
use crate::system::{address, byte};
use crate::system::cpu::CPU;

const STACK_TOP_ADDRESS : address = 0x01FF;
const STACK_BOTTOM_ADDRESS : address = 0x0100;

pub struct CPUStack {}
impl CPUStack
{
    pub fn push_byte(cpu : &mut CPU, value : byte)
    {
        cpu.bus.put(cpu.stack_pointer, value);

        cpu.stack_pointer =
            if cpu.stack_pointer-1 >= STACK_BOTTOM_ADDRESS
                { cpu.stack_pointer-1 }
            else { STACK_TOP_ADDRESS }
    }

    pub fn push_address(cpu : &mut CPU, address : address)
    {
        CPUStack::push_byte(cpu, ((address>>8)&0xFF) as byte);
        CPUStack::push_byte(cpu, (address&0xFF) as byte);
    }

    pub fn pop_byte(cpu : &mut CPU) -> byte
    {
        cpu.stack_pointer =
            if cpu.stack_pointer+1 <= STACK_TOP_ADDRESS
                { cpu.stack_pointer+1 }
            else { STACK_BOTTOM_ADDRESS };

        let value = cpu.bus.get(cpu.stack_pointer);
        return value;
    }

    pub fn pop_address(cpu : &mut CPU) -> address
    {
        let low = CPUStack::pop_byte(cpu);
        let high = CPUStack::pop_byte(cpu);
        let address = address_from_high_low!(high, low);
        return address;
    }

    pub fn get_pointer(cpu : &CPU) -> byte
    {
        return (cpu.stack_pointer & 0x00FF) as byte;
    }

    pub fn set_pointer(cpu : &mut CPU, raw_pointer : byte)
    {
        let new_pointer = STACK_BOTTOM_ADDRESS | raw_pointer as address;
        if new_pointer >= STACK_BOTTOM_ADDRESS && new_pointer <= STACK_TOP_ADDRESS
        {
            cpu.stack_pointer = new_pointer;
        }
    }
}
