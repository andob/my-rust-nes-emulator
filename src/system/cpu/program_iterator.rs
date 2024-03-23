use crate::address_from_high_low;
use crate::system::cpu::CPU;
use crate::system::{address, byte};
use crate::system::cpu::opcodes::Opcode;

#[derive(Debug, PartialEq, Eq)]
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
    Unknown,
}

const RAM_PAGE_SIZE : address = 256; //one page = 256 bytes

pub struct CPUProgramIterator {}
impl CPUProgramIterator
{
    //noinspection RsLift
    pub fn next_argument_from_rom(cpu : &mut CPU, opcode : &Opcode) -> (address, byte)
    {
        match opcode.addressing_mode
        {
            AddressingMode::Implied =>
            {
                return (0, cpu.A);
            }

            AddressingMode::Immediate =>
            {
                let value = CPUProgramIterator::next_byte_from_rom(cpu);
                return (0, value);
            }

            AddressingMode::Absolute =>
            {
                let address = CPUProgramIterator::next_address_from_rom(cpu);
                let value = cpu.bus.get(address);
                return (address, value);
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                let base_address = CPUProgramIterator::next_address_from_rom(cpu);
                let address = base_address.wrapping_add(cpu.X as address);
                if cpu.program_counter/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    cpu.clock.notify_page_boundary_crossed();
                }

                let value = cpu.bus.get(address);
                return (address, value);
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                let base_address = CPUProgramIterator::next_address_from_rom(cpu);
                let address = base_address.wrapping_add(cpu.Y as address);
                if cpu.program_counter/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    cpu.clock.notify_page_boundary_crossed();
                }

                let value = cpu.bus.get(address);
                return (address, value);
            }

            AddressingMode::ZeroPage =>
            {
                let address = CPUProgramIterator::next_byte_from_rom(cpu);
                let value = cpu.bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                let base_address = CPUProgramIterator::next_byte_from_rom(cpu);
                let address = base_address.wrapping_add(cpu.X);
                let value = cpu.bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                let base_address = CPUProgramIterator::next_byte_from_rom(cpu);
                let address = base_address.wrapping_add(cpu.Y);
                let value = cpu.bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::Indirect =>
            {
                let low_address = CPUProgramIterator::next_address_from_rom(cpu);
                let high_address = if low_address & 0x00FF == 0x00FF { low_address & 0xFF00 }
                else { low_address.wrapping_add(1) } as address;
                let low = cpu.bus.get(low_address);
                let high = cpu.bus.get(high_address);
                let address = address_from_high_low!(high, low);
                let value = cpu.bus.get(address as address);
                return (address, value);
            }

            AddressingMode::IndirectX =>
            {
                let base_base_address = CPUProgramIterator::next_byte_from_rom(cpu);
                let base_address = base_base_address.wrapping_add(cpu.X as byte);
                let low = cpu.bus.get(base_address as address);
                let high = cpu.bus.get(base_address.wrapping_add(1) as address);
                let address = address_from_high_low!(high, low);
                if cpu.program_counter/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    cpu.clock.notify_page_boundary_crossed();
                }

                let value = cpu.bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::IndirectY =>
            {
                let base_base_address = CPUProgramIterator::next_byte_from_rom(cpu);
                let low = cpu.bus.get(base_base_address as address);
                let high = cpu.bus.get(base_base_address.wrapping_add(1) as address);
                let base_address = address_from_high_low!(high, low);
                let address = base_address.wrapping_add(cpu.Y as address);
                if cpu.program_counter/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    cpu.clock.notify_page_boundary_crossed();
                }

                let value = cpu.bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::Relative =>
            {
                let offset = CPUProgramIterator::next_byte_from_rom(cpu);
                return (0, offset);
            }

            AddressingMode::Unknown =>
            {
                return (0, opcode.key);
            }
        };
    }

    pub fn next_byte_from_rom(cpu : &mut CPU) -> byte
    {
        let value = cpu.bus.get(cpu.program_counter);
        cpu.program_counter = cpu.program_counter.wrapping_add(1);
        return value;
    }

    pub fn next_address_from_rom(cpu : &mut CPU) -> address
    {
        let low = cpu.bus.get(cpu.program_counter);
        let high = cpu.bus.get(cpu.program_counter+1);
        let address = address_from_high_low!(high, low);
        cpu.program_counter = cpu.program_counter.wrapping_add(2);
        return address;
    }
}
