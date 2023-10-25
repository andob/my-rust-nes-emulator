use crate::system::cpu::CPU;
use crate::system::{address, byte, System};
use crate::system::cpu::opcodes::Opcode;
use crate::system::ram::RAM_PAGE_SIZE;

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

impl CPU
{
    pub fn next_argument_from_rom(nes : &mut System, opcode : &Opcode) -> (address, byte)
    {
        match opcode.addressing_mode
        {
            AddressingMode::Implied =>
            {
                return (0, nes.cpu.A);
            }

            AddressingMode::Immediate =>
            {
                let value = CPU::next_byte_from_rom(nes);
                return (0, value);
            }

            AddressingMode::Absolute =>
            {
                let address = CPU::next_address_from_rom(nes);
                let value = nes.cpu_bus.get(address);
                return (address, value);
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                let base_address = CPU::next_address_from_rom(nes);
                let address = base_address.wrapping_add(nes.cpu.X as address);
                if base_address/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    nes.cpu.clock.notify_page_boundary_crossed();
                }

                let value = nes.cpu_bus.get(address);
                return (address, value);
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                let base_address = CPU::next_address_from_rom(nes);
                let address = base_address.wrapping_add(nes.cpu.Y as address);
                if base_address/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    nes.cpu.clock.notify_page_boundary_crossed();
                }

                let value = nes.cpu_bus.get(address);
                return (address, value);
            }

            AddressingMode::ZeroPage =>
            {
                let address = CPU::next_byte_from_rom(nes);
                let value = nes.cpu_bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                let base_address = CPU::next_byte_from_rom(nes);
                let address = base_address.wrapping_add(nes.cpu.X);
                let value = nes.cpu_bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                let base_address = CPU::next_byte_from_rom(nes);
                let address = base_address.wrapping_add(nes.cpu.Y);
                let value = nes.cpu_bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::Indirect =>
            {
                let address = CPU::next_address_from_rom(nes);
                let value = nes.cpu_bus.get(address);
                return (address, value);
            }

            AddressingMode::IndirectX =>
            {
                let base_address = CPU::next_byte_from_rom(nes);
                let pointer = base_address.wrapping_add(nes.cpu.X as byte);
                let low = nes.cpu_bus.get(pointer as address);
                let high = nes.cpu_bus.get(pointer.wrapping_add(1) as address);
                let address = ((high as address)<<8) | (low as address);
                if (base_address as address)/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    nes.cpu.clock.notify_page_boundary_crossed();
                }

                let value = nes.cpu_bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::IndirectY =>
            {
                let base_address = CPU::next_byte_from_rom(nes);
                let pointer = base_address.wrapping_add(nes.cpu.Y as byte);
                let low = nes.cpu_bus.get(pointer as address);
                let high = nes.cpu_bus.get(pointer.wrapping_add(1) as address);
                let address = ((high as address)<<8) | (low as address);
                if (base_address as address)/RAM_PAGE_SIZE != address/RAM_PAGE_SIZE
                {
                    nes.cpu.clock.notify_page_boundary_crossed();
                }

                let value = nes.cpu_bus.get(address as address);
                return (address as address, value);
            }

            AddressingMode::Relative =>
            {
                let offset = CPU::next_byte_from_rom(nes);
                return (0, offset);
            }

            AddressingMode::Unknown =>
            {
                return (0, opcode.key);
            }
        };
    }

    pub fn next_byte_from_rom(nes : &mut System) -> byte
    {
        let value = nes.cpu_bus.program_rom.get(nes.cpu.program_counter);
        nes.cpu.program_counter += 1;
        return value;
    }

    pub fn next_address_from_rom(nes : &mut System) -> address
    {
        let low = nes.cpu_bus.program_rom.get(nes.cpu.program_counter);
        let high = nes.cpu_bus.program_rom.get(nes.cpu.program_counter+1);
        let value = ((high as address) << 8) | (low as address);
        nes.cpu.program_counter += 2;
        return value;
    }
}
