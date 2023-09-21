use crate::system::cpu::{AddressingMode, CPU};
use crate::system::System;

impl CPU
{
    pub fn next_argument_from_rom(nes : &mut System, addressing_mode : &AddressingMode) -> u8
    {
        match addressing_mode
        {
            AddressingMode::Implied => { return 0; }
            AddressingMode::Unknown => { return 0; }

            AddressingMode::Immediate =>
            {
                if let Some(value) = CPU::next_u8_from_rom(nes)
                {
                    return value;
                }
            }

            AddressingMode::Absolute =>
            {
                if let Some(address) = CPU::next_u16_from_rom(nes)
                {
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as u16);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as u16);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::ZeroPage =>
            {
                if let Some(address) = CPU::next_u8_from_rom(nes)
                {
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                if let Some(base_address) = CPU::next_u8_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                if let Some(base_address) = CPU::next_u8_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::Indirect =>
            {
                if let Some(address) = CPU::next_u16_from_rom(nes)
                {
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::IndirectX =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as u16);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::IndirectY =>
            {
                if let Some(base_address) = CPU::next_u16_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as u16);
                    if let Some(arg) = nes.ram.get(address as usize)
                    {
                        return *arg;
                    }
                }
            }

            AddressingMode::Relative =>
            {
                if let Some(offset) = CPU::next_u8_from_rom(nes)
                {
                    return offset;
                }
            }
        };

        return 0;
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
