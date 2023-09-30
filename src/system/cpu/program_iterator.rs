use crate::system::cpu::CPU;
use crate::system::{address, byte, System};

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
    pub fn next_argument_from_rom(nes : &mut System, addressing_mode : &AddressingMode) -> (address, byte)
    {
        match addressing_mode
        {
            AddressingMode::Implied =>
            {
                return (0, nes.cpu.A);
            }

            AddressingMode::Immediate =>
            {
                if let Some(value) = CPU::next_byte_from_rom(nes)
                {
                    return (0, value);
                }
            }

            AddressingMode::Absolute =>
            {
                if let Some(address) = CPU::next_address_from_rom(nes)
                {
                    let value = nes.ram.get(address);
                    return (address, value);
                }
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                if let Some(base_address) = CPU::next_address_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as address);
                    let value = nes.ram.get(address);
                    return (address, value);
                }
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                if let Some(base_address) = CPU::next_address_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as address);
                    let value = nes.ram.get(address);
                    return (address, value);
                }
            }

            AddressingMode::ZeroPage =>
            {
                if let Some(address) = CPU::next_byte_from_rom(nes)
                {
                    let value = nes.ram.get(address as address);
                    return (address as address, value);
                }
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X);
                    let value = nes.ram.get(address as address);
                    return (address as address, value);
                }
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y);
                    let value = nes.ram.get(address as address);
                    return (address as address, value);
                }
            }

            AddressingMode::Indirect =>
            {
                if let Some(address) = CPU::next_address_from_rom(nes)
                {
                    let value = nes.ram.get(address);
                    return (address, value);
                }
            }

            AddressingMode::IndirectX =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let pointer = base_address.wrapping_add(nes.cpu.X as byte);
                    let low = nes.ram.get(pointer as address);
                    let high = nes.ram.get((pointer as address).wrapping_add(1));
                    let address = ((high as address)<<8) | (low as address);
                    let value = nes.ram.get(address as address);
                    return (address as address, value);
                }
            }

            AddressingMode::IndirectY =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let pointer = base_address.wrapping_add(nes.cpu.Y as byte);
                    let low = nes.ram.get(pointer as address);
                    let high = nes.ram.get(pointer as address);
                    let address = ((high as address)<<8) | (low as address);
                    let value = nes.ram.get(address as address);
                    return (address as address, value);
                }
            }

            AddressingMode::Relative =>
            {
                if let Some(offset) = CPU::next_byte_from_rom(nes)
                {
                    return (0, offset);
                }
            }
        };

        return (0, 0);
    }

    pub fn next_byte_from_rom(nes : &mut System) -> Option<byte>
    {
        if let Some(value) = nes.rom.get(nes.cpu.program_counter)
        {
            nes.cpu.program_counter += 1;
            return Some(value);
        }

        return None;
    }

    pub fn next_address_from_rom(nes : &mut System) -> Option<address>
    {
        if let (Some(low), Some(high)) = (nes.rom.get(nes.cpu.program_counter), nes.rom.get(nes.cpu.program_counter+1))
        {
            let value = ((high as address) << 8) | (low as address);
            nes.cpu.program_counter += 2;
            return Some(value);
        }

        return None;
    }
}
