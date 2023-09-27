use crate::system::cpu::CPU;
use crate::system::System;

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

pub struct OpcodeArg
{
    pub address : usize,
    pub value : u8,
}

impl CPU
{
    pub fn next_argument_from_rom(nes : &mut System, addressing_mode : &AddressingMode) -> OpcodeArg
    {
        match addressing_mode
        {
            AddressingMode::Implied =>
            {
                return OpcodeArg { address:0, value: nes.cpu.A };
            }

            AddressingMode::Immediate =>
            {
                if let Some(value) = CPU::next_u8_from_rom(nes)
                {
                    return OpcodeArg { address:0, value };
                }
            }

            AddressingMode::Absolute =>
            {
                if let Some(address) = CPU::next_u16_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as u16);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as u16);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPage =>
            {
                if let Some(address) = CPU::next_u8_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                if let Some(base_address) = CPU::next_u8_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                if let Some(base_address) = CPU::next_u8_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::Indirect =>
            {
                if let Some(address) = CPU::next_u16_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::IndirectX =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as u16);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::IndirectY =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as u16);
                    if let Some(value) = nes.ram.get(address as usize)
                    {
                        return OpcodeArg { address:address as usize, value:*value };
                    }
                }
            }

            AddressingMode::Relative =>
            {
                if let Some(offset) = CPU::next_u8_from_rom(nes)
                {
                    return OpcodeArg { address:0, value:offset };
                }
            }
        };

        return OpcodeArg { address:0, value:0 };
    }

    pub fn next_u8_from_rom(nes : &mut System) -> Option<u8>
    {
        if let Some(value) = nes.rom.get(nes.cpu.program_counter)
        {
            nes.cpu.program_counter += 1;
            return Some(*value);
        }

        return None;
    }

    pub fn next_u16_from_rom(nes : &mut System) -> Option<u16>
    {
        if let (Some(low), Some(high)) = (nes.rom.get(nes.cpu.program_counter), nes.rom.get(nes.cpu.program_counter+1))
        {
            let value = ((*low as u16) << 8) | (*high as u16);
            nes.cpu.program_counter += 2;
            return Some(value);
        }

        return None;
    }
}
