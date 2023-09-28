use crate::system::cpu::CPU;
use crate::type_alias::{byte, word};
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
    pub address : word,
    pub value : byte,
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
                if let Some(value) = CPU::next_byte_from_rom(nes)
                {
                    return OpcodeArg { address:0, value };
                }
            }

            AddressingMode::Absolute =>
            {
                if let Some(address) = CPU::next_word_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::AbsoluteXIndexed =>
            {
                if let Some(base_address) = CPU::next_word_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as word);
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::AbsoluteYIndexed =>
            {
                if let Some(base_address) = CPU::next_word_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as word);
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPage =>
            {
                if let Some(address) = CPU::next_byte_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address as word)
                    {
                        return OpcodeArg { address:address as word, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPageXIndexed =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X);
                    if let Some(value) = nes.ram.get(address as word)
                    {
                        return OpcodeArg { address:address as word, value:*value };
                    }
                }
            }

            AddressingMode::ZeroPageYIndexed =>
            {
                if let Some(base_address) = CPU::next_byte_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y);
                    if let Some(value) = nes.ram.get(address as word)
                    {
                        return OpcodeArg { address:address as word, value:*value };
                    }
                }
            }

            AddressingMode::Indirect =>
            {
                if let Some(address) = CPU::next_word_from_rom(nes)
                {
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::IndirectX =>
            {
                if let Some(base_address) = CPU::next_word_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.X as word);
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::IndirectY =>
            {
                if let Some(base_address) = CPU::next_word_from_rom(nes)
                {
                    let address = base_address.wrapping_add(nes.cpu.Y as word);
                    if let Some(value) = nes.ram.get(address)
                    {
                        return OpcodeArg { address, value:*value };
                    }
                }
            }

            AddressingMode::Relative =>
            {
                if let Some(offset) = CPU::next_byte_from_rom(nes)
                {
                    return OpcodeArg { address:0, value:offset };
                }
            }
        };

        return OpcodeArg { address:0, value:0 };
    }

    pub fn next_byte_from_rom(nes : &mut System) -> Option<byte>
    {
        if let Some(value) = nes.rom.get(nes.cpu.program_counter)
        {
            nes.cpu.program_counter += 1;
            return Some(*value);
        }

        return None;
    }

    pub fn next_word_from_rom(nes : &mut System) -> Option<word>
    {
        if let (Some(low), Some(high)) = (nes.rom.get(nes.cpu.program_counter), nes.rom.get(nes.cpu.program_counter+1))
        {
            let value = ((*low as word) << 8) | (*high as word);
            nes.cpu.program_counter += 2;
            return Some(value);
        }

        return None;
    }
}
