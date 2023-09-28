use std::collections::HashMap;
use maplit2::hashmap;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::program_iterator::AddressingMode;
use crate::system::cpu::stack::CPUStack;
use crate::system::System;
use crate::type_alias::{byte, word};

pub struct Opcode
{
    pub name : String,
    pub addressing_mode : AddressingMode,
    pub lambda : fn(&mut System, word, byte) -> (),
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
                lambda: |nes,address,value| $name(nes,address,value),
                expected_time: $expected_time,
                addressing_mode: $addressing_mode,
            }
        }
    }
}

pub fn build_opcodes_map() -> HashMap<byte, Opcode>
{
    return hashmap!
    {
        0x69 => opcode!(adc, 2, AddressingMode::Immediate),
        0x65 => opcode!(adc, 3, AddressingMode::ZeroPage),
        0x75 => opcode!(adc, 4, AddressingMode::ZeroPageXIndexed),
        0x6D => opcode!(adc, 4, AddressingMode::Absolute),
        0x7D => opcode!(adc, 4, AddressingMode::AbsoluteXIndexed),
        0x79 => opcode!(adc, 4, AddressingMode::AbsoluteYIndexed),
        0x61 => opcode!(adc, 6, AddressingMode::IndirectX),
        0x71 => opcode!(adc, 5, AddressingMode::IndirectY),
        0x29 => opcode!(and, 2, AddressingMode::Immediate),
        0x25 => opcode!(and, 3, AddressingMode::ZeroPage),
        0x35 => opcode!(and, 4, AddressingMode::ZeroPageXIndexed),
        0x2D => opcode!(and, 4, AddressingMode::Absolute),
        0x3D => opcode!(and, 4, AddressingMode::AbsoluteXIndexed),
        0x39 => opcode!(and, 4, AddressingMode::AbsoluteYIndexed),
        0x21 => opcode!(and, 6, AddressingMode::IndirectX),
        0x31 => opcode!(and, 5, AddressingMode::IndirectY),
        0x0A => opcode!(asl, 2, AddressingMode::Implied),
        0x06 => opcode!(asl, 5, AddressingMode::ZeroPage),
        0x16 => opcode!(asl, 6, AddressingMode::ZeroPageXIndexed),
        0x0E => opcode!(asl, 6, AddressingMode::Absolute),
        0x1E => opcode!(asl, 7, AddressingMode::AbsoluteXIndexed),
        0x24 => opcode!(bit, 3, AddressingMode::ZeroPage),
        0x2C => opcode!(bit, 4, AddressingMode::Absolute),
        0x10 => opcode!(bpl, 2, AddressingMode::Relative),
        0x30 => opcode!(bmi, 2, AddressingMode::Relative),
        0x50 => opcode!(bvc, 2, AddressingMode::Relative),
        0x70 => opcode!(bvs, 2, AddressingMode::Relative),
        0x90 => opcode!(bcc, 2, AddressingMode::Relative),
        0xB0 => opcode!(bcs, 2, AddressingMode::Relative),
        0xD0 => opcode!(bne, 2, AddressingMode::Relative),
        0xF0 => opcode!(beq, 2, AddressingMode::Relative),
        0x00 => opcode!(brk, 7, AddressingMode::Implied),
        0xC5 => opcode!(cmp, 3, AddressingMode::ZeroPage),
        0xD5 => opcode!(cmp, 4, AddressingMode::ZeroPageXIndexed),
        0xCD => opcode!(cmp, 4, AddressingMode::Absolute),
        0xDD => opcode!(cmp, 4, AddressingMode::AbsoluteXIndexed),
        0xD9 => opcode!(cmp, 4, AddressingMode::AbsoluteYIndexed),
        0xC1 => opcode!(cmp, 6, AddressingMode::IndirectX),
        0xD1 => opcode!(cmp, 5, AddressingMode::IndirectY),
        0xE0 => opcode!(cpx, 2, AddressingMode::Immediate),
        0xE4 => opcode!(cpx, 3, AddressingMode::ZeroPage),
        0xEC => opcode!(cpx, 4, AddressingMode::Absolute),
        0xC0 => opcode!(cpy, 2, AddressingMode::Immediate),
        0xC4 => opcode!(cpy, 3, AddressingMode::ZeroPage),
        0xCC => opcode!(cpy, 4, AddressingMode::Absolute),
        0xC6 => opcode!(dec, 5, AddressingMode::ZeroPage),
        0xD6 => opcode!(dec, 6, AddressingMode::ZeroPageXIndexed),
        0xCE => opcode!(dec, 6, AddressingMode::Absolute),
        0xDE => opcode!(dec, 7, AddressingMode::AbsoluteXIndexed),
        0x49 => opcode!(eor, 2, AddressingMode::Immediate),
        0x45 => opcode!(eor, 3, AddressingMode::ZeroPage),
        0x55 => opcode!(eor, 4, AddressingMode::ZeroPageXIndexed),
        0x4D => opcode!(eor, 4, AddressingMode::Absolute),
        0x5D => opcode!(eor, 4, AddressingMode::AbsoluteXIndexed),
        0x59 => opcode!(eor, 4, AddressingMode::AbsoluteYIndexed),
        0x41 => opcode!(eor, 6, AddressingMode::IndirectX),
        0x51 => opcode!(eor, 5, AddressingMode::IndirectY),
        0x18 => opcode!(clc, 2, AddressingMode::Implied),
        0x38 => opcode!(sec, 2, AddressingMode::Implied),
        0x58 => opcode!(cli, 2, AddressingMode::Implied),
        0x78 => opcode!(sei, 2, AddressingMode::Implied),
        0xB8 => opcode!(clv, 2, AddressingMode::Implied),
        0xD8 => opcode!(cld, 2, AddressingMode::Implied),
        0xF8 => opcode!(sed, 2, AddressingMode::Implied),
        0xE6 => opcode!(inc, 5, AddressingMode::ZeroPage),
        0xF6 => opcode!(inc, 6, AddressingMode::ZeroPageXIndexed),
        0xEE => opcode!(inc, 6, AddressingMode::Absolute),
        0xFE => opcode!(inc, 7, AddressingMode::AbsoluteXIndexed),
        0x4C => opcode!(jmp, 3, AddressingMode::Absolute),
        0x6C => opcode!(jmp, 5, AddressingMode::Indirect),
        0x20 => opcode!(jsr, 6, AddressingMode::Absolute),
        0xA9 => opcode!(lda, 2, AddressingMode::Immediate),
        0xA5 => opcode!(lda, 3, AddressingMode::ZeroPage),
        0xB5 => opcode!(lda, 4, AddressingMode::ZeroPageXIndexed),
        0xAD => opcode!(lda, 4, AddressingMode::Absolute),
        0xBD => opcode!(lda, 4, AddressingMode::AbsoluteXIndexed),
        0xB9 => opcode!(lda, 4, AddressingMode::AbsoluteYIndexed),
        0xA1 => opcode!(lda, 6, AddressingMode::IndirectX),
        0xB1 => opcode!(lda, 5, AddressingMode::IndirectY),
        0xA2 => opcode!(ldx, 2, AddressingMode::Immediate),
        0xA6 => opcode!(ldx, 3, AddressingMode::ZeroPage),
        0xB6 => opcode!(ldx, 4, AddressingMode::ZeroPageYIndexed),
        0xAE => opcode!(ldx, 4, AddressingMode::Absolute),
        0xBE => opcode!(ldx, 4, AddressingMode::AbsoluteYIndexed),
        0xA0 => opcode!(ldy, 2, AddressingMode::Immediate),
        0xA4 => opcode!(ldy, 3, AddressingMode::ZeroPage),
        0xB4 => opcode!(ldy, 4, AddressingMode::ZeroPageXIndexed),
        0xAC => opcode!(ldy, 4, AddressingMode::Absolute),
        0xBC => opcode!(ldy, 4, AddressingMode::AbsoluteXIndexed),
        0x4A => opcode!(lsr, 2, AddressingMode::Implied),
        0x46 => opcode!(lsr, 5, AddressingMode::ZeroPage),
        0x56 => opcode!(lsr, 6, AddressingMode::ZeroPageXIndexed),
        0x4E => opcode!(lsr, 6, AddressingMode::Absolute),
        0x5E => opcode!(lsr, 7, AddressingMode::AbsoluteXIndexed),
        0xEA => opcode!(nop, 2, AddressingMode::Implied),
        0x09 => opcode!(ora, 2, AddressingMode::Immediate),
        0x05 => opcode!(ora, 3, AddressingMode::ZeroPage),
        0x15 => opcode!(ora, 4, AddressingMode::ZeroPageXIndexed),
        0x0D => opcode!(ora, 4, AddressingMode::Absolute),
        0x1D => opcode!(ora, 4, AddressingMode::AbsoluteXIndexed),
        0x19 => opcode!(ora, 4, AddressingMode::AbsoluteYIndexed),
        0x01 => opcode!(ora, 6, AddressingMode::IndirectX),
        0x11 => opcode!(ora, 5, AddressingMode::IndirectY),
        0xAA => opcode!(tax, 2, AddressingMode::Implied),
        0x8A => opcode!(txa, 2, AddressingMode::Implied),
        0xCA => opcode!(dex, 2, AddressingMode::Implied),
        0xE8 => opcode!(inx, 2, AddressingMode::Implied),
        0xA8 => opcode!(tay, 2, AddressingMode::Implied),
        0x98 => opcode!(tya, 2, AddressingMode::Implied),
        0x88 => opcode!(dey, 2, AddressingMode::Implied),
        0xC8 => opcode!(iny, 2, AddressingMode::Implied),
        0x2A => opcode!(rol, 2, AddressingMode::Implied),
        0x26 => opcode!(rol, 5, AddressingMode::ZeroPage),
        0x36 => opcode!(rol, 6, AddressingMode::ZeroPageXIndexed),
        0x2E => opcode!(rol, 6, AddressingMode::Absolute),
        0x3E => opcode!(rol, 7, AddressingMode::AbsoluteXIndexed),
        0x6A => opcode!(ror, 2, AddressingMode::Implied),
        0x66 => opcode!(ror, 5, AddressingMode::ZeroPage),
        0x76 => opcode!(ror, 6, AddressingMode::ZeroPageXIndexed),
        0x6E => opcode!(ror, 6, AddressingMode::Absolute),
        0x7E => opcode!(ror, 7, AddressingMode::AbsoluteXIndexed),
        0x40 => opcode!(rti, 6, AddressingMode::Implied),
        0x60 => opcode!(rts, 6, AddressingMode::Implied),
        0xE9 => opcode!(sbc, 2, AddressingMode::Immediate),
        0xE5 => opcode!(sbc, 3, AddressingMode::ZeroPage),
        0xF5 => opcode!(sbc, 4, AddressingMode::ZeroPageXIndexed),
        0xED => opcode!(sbc, 4, AddressingMode::Absolute),
        0xFD => opcode!(sbc, 4, AddressingMode::AbsoluteXIndexed),
        0xF9 => opcode!(sbc, 4, AddressingMode::AbsoluteYIndexed),
        0xE1 => opcode!(sbc, 6, AddressingMode::IndirectX),
        0xF1 => opcode!(sbc, 5, AddressingMode::IndirectY),
        0x85 => opcode!(sta, 3, AddressingMode::ZeroPage),
        0x95 => opcode!(sta, 4, AddressingMode::ZeroPageXIndexed),
        0x8D => opcode!(sta, 4, AddressingMode::Absolute),
        0x9D => opcode!(sta, 5, AddressingMode::AbsoluteXIndexed),
        0x99 => opcode!(sta, 5, AddressingMode::AbsoluteYIndexed),
        0x81 => opcode!(sta, 6, AddressingMode::IndirectX),
        0x91 => opcode!(sta, 6, AddressingMode::IndirectY),
        0x9A => opcode!(txs, 2, AddressingMode::Implied),
        0xBA => opcode!(tsx, 2, AddressingMode::Implied),
        0x48 => opcode!(pha, 3, AddressingMode::Implied),
        0x68 => opcode!(pla, 4, AddressingMode::Implied),
        0x08 => opcode!(php, 3, AddressingMode::Implied),
        0x28 => opcode!(plp, 4, AddressingMode::Implied),
        0x86 => opcode!(stx, 3, AddressingMode::ZeroPage),
        0x96 => opcode!(stx, 4, AddressingMode::ZeroPageYIndexed),
        0x8E => opcode!(stx, 4, AddressingMode::Absolute),
        0x84 => opcode!(sty, 3, AddressingMode::ZeroPage),
        0x94 => opcode!(sty, 4, AddressingMode::ZeroPageXIndexed),
        0x8C => opcode!(sty, 4, AddressingMode::Absolute),
    };
}

macro_rules! is_negative
{
    ($arg : expr) => {{ ($arg)>>7==1 }}
}

fn adc(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn and(nes : &mut System, _address : word, value : byte)
{
    //logical AND value with Accumulator
    let new_value = nes.cpu.A & value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn asl(nes : &mut System, address : word, value : byte)
{
    //arithmetic shift left
    nes.cpu.flags.carry = (value & 0b10000000) >> 7 == 1;
    let new_value = value << 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn bit(nes : &mut System, _address : word, value : byte)
{
    //test bits
    nes.cpu.flags.negative = (value & 0b10000000) >> 7 == 1;
    nes.cpu.flags.overflow = (value & 0b01000000) >> 6 == 1;
    nes.cpu.flags.zero = (value & nes.cpu.A) == 0;
}

fn bpl(nes : &mut System, _address : word, offset : byte)
{
    //branch on result plus
    if !nes.cpu.flags.negative
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bmi(nes : &mut System, _address : word, offset : byte)
{
    //branch on result minus
    if nes.cpu.flags.negative
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bvc(nes : &mut System, _address : word, offset : byte)
{
    //branch on overflow clear
    if !nes.cpu.flags.overflow
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bvs(nes : &mut System, _address : word, offset : byte)
{
    //branch on overflow set
    if nes.cpu.flags.overflow
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bcc(nes : &mut System, _address : word, offset : byte)
{
    //branch on carry clear
    if !nes.cpu.flags.carry
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bcs(nes : &mut System, _address : word, offset : byte)
{
    //branch on carry set
    if nes.cpu.flags.carry
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn bne(nes : &mut System, _address : word, offset : byte)
{
    //branch on result not zero
    if !nes.cpu.flags.zero
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn beq(nes : &mut System, _address : word, offset : byte)
{
    //branch on result zero
    if nes.cpu.flags.zero
    {
        nes.cpu.program_counter += offset as word;
    }
}

fn brk(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn cmp(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn cpx(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn cpy(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn dec(nes : &mut System, address : word, value : byte)
{
    //decrement memory
    let new_value = value-1;
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn eor(nes : &mut System, _address : word, value : byte)
{
    //logical XOR value with Accumulator
    let new_value = nes.cpu.A ^ value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn clc(nes : &mut System, _address : word, _value : byte)
{
    //clear carry flag
    nes.cpu.flags.carry = false;
}

fn sec(nes : &mut System, _address : word, _value : byte)
{
    //set carry flag
    nes.cpu.flags.carry = true;
}

fn cli(nes : &mut System, _address : word, _value : byte)
{
    //clear interrupt disable flag
    nes.cpu.flags.interrupt = false;
}

fn sei(nes : &mut System, _address : word, _value : byte)
{
    //set interrupt disable flag
    nes.cpu.flags.interrupt = true;
}

fn clv(nes : &mut System, _address : word, _value : byte)
{
    //clear overflow flag
    nes.cpu.flags.overflow = false;
}

fn cld(nes : &mut System, _address : word, _value : byte)
{
    //clear decimal mode flag
    nes.cpu.flags.decimal = false;
}

fn sed(nes : &mut System, _address : word, _value : byte)
{
    //set decimal mode flag
    nes.cpu.flags.decimal = true;
}

fn inc(nes : &mut System, address : word, value : byte)
{
    //increment memory
    let new_value = value+1;
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn jmp(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn jsr(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn lda(nes : &mut System, _address : word, value : byte)
{
    //load memory into Accumulator
    nes.cpu.A = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldx(nes : &mut System, _address : word, value : byte)
{
    //load memory into register X
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldy(nes : &mut System, _address : word, value: byte)
{
    //load memory into register Y
    nes.cpu.Y = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn lsr(nes : &mut System, address : word, value : byte)
{
    //logical shift right
    nes.cpu.flags.carry = value & 0b00000001 == 1;
    let new_value = value >> 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn nop(_nes : &mut System, _address : word, _value : byte)
{
    //no operation!
}

fn ora(nes : &mut System, _address : word, value : byte)
{
    //logical OR value with Accumulator
    let new_value = nes.cpu.A | value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tax(nes : &mut System, _address : word, _value : byte)
{
    //transfer Accumulator into register X
    let new_value = nes.cpu.A;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn txa(nes : &mut System, _address : word, _value : byte)
{
    //transfer register X into Accumulator
    let new_value = nes.cpu.X;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dex(nes : &mut System, _address : word, _value : byte)
{
    //decrement register X
    let new_value = nes.cpu.X-1;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn inx(nes : &mut System, _address : word, _value : byte)
{
    //increment register X
    let new_value = nes.cpu.X+1;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tay(nes : &mut System, _address : word, _value : byte)
{
    //transfer Accumulator into register Y
    let new_value = nes.cpu.A;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tya(nes : &mut System, _address : word, _value : byte)
{
    //transfer register Y into Accumulator
    let new_value = nes.cpu.Y;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dey(nes : &mut System, _address : word, _value : byte)
{
    //decrement register Y
    let new_value = nes.cpu.Y-1;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn iny(nes : &mut System, _address : word, _value : byte)
{
    //increment register Y
    let new_value = nes.cpu.Y+1;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn rol(nes : &mut System, address : word, value : byte)
{
    //rotate left
    let bit = (value & 0b10000000) >> 7;
    nes.cpu.flags.carry = bit==1;
    let new_value = (value << 1) | bit;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn ror(nes : &mut System, address : word, value : byte)
{
    //rotate right
    let bit = value & 0b00000001;
    nes.cpu.flags.carry = bit==1;
    let new_value = (value >> 1) | (bit << 7);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn rti(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn rts(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn sbc(_nes : &mut System, _address : word, _value : byte)
{
    todo!()
}

fn sta(nes : &mut System, address : word, _value : byte)
{
    //store Accumulator into memory
    nes.ram.put(address, nes.cpu.A);
}

fn txs(nes : &mut System, _address : word, _value : byte)
{
    //transfer register X into Stack Pointer
    nes.cpu.stack.set_pointer(nes.cpu.X);
}

fn tsx(nes : &mut System, _address : word, _value : byte)
{
    //transfer Stack Pointer into register X
    let new_value = nes.cpu.stack.get_pointer();
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn pha(nes : &mut System, _address : word, _value : byte)
{
    //push Accumulator on Stack
    CPUStack::push(nes, nes.cpu.A);
}

fn pla(nes : &mut System, _address : word, _value : byte)
{
    //pop Stack into Accumulator
    let new_value = CPUStack::pop(nes);
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn php(nes : &mut System, _address : word, _value : byte)
{
    //push Processor Flags on Stack
    let byte = nes.cpu.flags.to_byte();
    CPUStack::push(nes, byte);
}

fn plp(nes : &mut System, _address : word, _value : byte)
{
    //pull Stack into Processor Flags
    let byte = CPUStack::pop(nes);
    nes.cpu.flags = CPUFlags::from_byte(byte);
}

fn stx(nes : &mut System, address : word, _value : byte)
{
    //store register X into memory
    nes.ram.put(address, nes.cpu.A);
}

fn sty(nes : &mut System, address : word, _value : byte)
{
    //store register Y into memory
    nes.ram.put(address, nes.cpu.A);
}

