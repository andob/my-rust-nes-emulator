use std::collections::HashMap;
use maplit2::hashmap;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::program_iterator::AddressingMode;
use crate::system::cpu::stack::CPUStack;
use crate::system::System;

pub struct Opcode
{
    pub name : String,
    pub addressing_mode : AddressingMode,
    pub lambda : fn(&mut System, usize, u8) -> (),
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

pub fn build_opcodes_map() -> HashMap<u8, Opcode>
{
    return hashmap!
    {
        0x69u8 => opcode!(adc, 2, AddressingMode::Immediate),
        0x65u8 => opcode!(adc, 3, AddressingMode::ZeroPage),
        0x75u8 => opcode!(adc, 4, AddressingMode::ZeroPageXIndexed),
        0x6Du8 => opcode!(adc, 4, AddressingMode::Absolute),
        0x7Du8 => opcode!(adc, 4, AddressingMode::AbsoluteXIndexed),
        0x79u8 => opcode!(adc, 4, AddressingMode::AbsoluteYIndexed),
        0x61u8 => opcode!(adc, 6, AddressingMode::IndirectX),
        0x71u8 => opcode!(adc, 5, AddressingMode::IndirectY),
        0x29u8 => opcode!(and, 2, AddressingMode::Immediate),
        0x25u8 => opcode!(and, 3, AddressingMode::ZeroPage),
        0x35u8 => opcode!(and, 4, AddressingMode::ZeroPageXIndexed),
        0x2Du8 => opcode!(and, 4, AddressingMode::Absolute),
        0x3Du8 => opcode!(and, 4, AddressingMode::AbsoluteXIndexed),
        0x39u8 => opcode!(and, 4, AddressingMode::AbsoluteYIndexed),
        0x21u8 => opcode!(and, 6, AddressingMode::IndirectX),
        0x31u8 => opcode!(and, 5, AddressingMode::IndirectY),
        0x0Au8 => opcode!(asl, 2, AddressingMode::Implied),
        0x06u8 => opcode!(asl, 5, AddressingMode::ZeroPage),
        0x16u8 => opcode!(asl, 6, AddressingMode::ZeroPageXIndexed),
        0x0Eu8 => opcode!(asl, 6, AddressingMode::Absolute),
        0x1Eu8 => opcode!(asl, 7, AddressingMode::AbsoluteXIndexed),
        0x24u8 => opcode!(bit, 3, AddressingMode::ZeroPage),
        0x2Cu8 => opcode!(bit, 4, AddressingMode::Absolute),
        0x10u8 => opcode!(bpl, 2, AddressingMode::Relative),
        0x30u8 => opcode!(bmi, 2, AddressingMode::Relative),
        0x50u8 => opcode!(bvc, 2, AddressingMode::Relative),
        0x70u8 => opcode!(bvs, 2, AddressingMode::Relative),
        0x90u8 => opcode!(bcc, 2, AddressingMode::Relative),
        0xB0u8 => opcode!(bcs, 2, AddressingMode::Relative),
        0xD0u8 => opcode!(bne, 2, AddressingMode::Relative),
        0xF0u8 => opcode!(beq, 2, AddressingMode::Relative),
        0x00u8 => opcode!(brk, 7, AddressingMode::Implied),
        0xC5u8 => opcode!(cmp, 3, AddressingMode::ZeroPage),
        0xD5u8 => opcode!(cmp, 4, AddressingMode::ZeroPageXIndexed),
        0xCDu8 => opcode!(cmp, 4, AddressingMode::Absolute),
        0xDDu8 => opcode!(cmp, 4, AddressingMode::AbsoluteXIndexed),
        0xD9u8 => opcode!(cmp, 4, AddressingMode::AbsoluteYIndexed),
        0xC1u8 => opcode!(cmp, 6, AddressingMode::IndirectX),
        0xD1u8 => opcode!(cmp, 5, AddressingMode::IndirectY),
        0xE0u8 => opcode!(cpx, 2, AddressingMode::Immediate),
        0xE4u8 => opcode!(cpx, 3, AddressingMode::ZeroPage),
        0xECu8 => opcode!(cpx, 4, AddressingMode::Absolute),
        0xC0u8 => opcode!(cpy, 2, AddressingMode::Immediate),
        0xC4u8 => opcode!(cpy, 3, AddressingMode::ZeroPage),
        0xCCu8 => opcode!(cpy, 4, AddressingMode::Absolute),
        0xC6u8 => opcode!(dec, 5, AddressingMode::ZeroPage),
        0xD6u8 => opcode!(dec, 6, AddressingMode::ZeroPageXIndexed),
        0xCEu8 => opcode!(dec, 6, AddressingMode::Absolute),
        0xDEu8 => opcode!(dec, 7, AddressingMode::AbsoluteXIndexed),
        0x49u8 => opcode!(eor, 2, AddressingMode::Immediate),
        0x45u8 => opcode!(eor, 3, AddressingMode::ZeroPage),
        0x55u8 => opcode!(eor, 4, AddressingMode::ZeroPageXIndexed),
        0x4Du8 => opcode!(eor, 4, AddressingMode::Absolute),
        0x5Du8 => opcode!(eor, 4, AddressingMode::AbsoluteXIndexed),
        0x59u8 => opcode!(eor, 4, AddressingMode::AbsoluteYIndexed),
        0x41u8 => opcode!(eor, 6, AddressingMode::IndirectX),
        0x51u8 => opcode!(eor, 5, AddressingMode::IndirectY),
        0x18u8 => opcode!(clc, 2, AddressingMode::Implied),
        0x38u8 => opcode!(sec, 2, AddressingMode::Implied),
        0x58u8 => opcode!(cli, 2, AddressingMode::Implied),
        0x78u8 => opcode!(sei, 2, AddressingMode::Implied),
        0xB8u8 => opcode!(clv, 2, AddressingMode::Implied),
        0xD8u8 => opcode!(cld, 2, AddressingMode::Implied),
        0xF8u8 => opcode!(sed, 2, AddressingMode::Implied),
        0xE6u8 => opcode!(inc, 5, AddressingMode::ZeroPage),
        0xF6u8 => opcode!(inc, 6, AddressingMode::ZeroPageXIndexed),
        0xEEu8 => opcode!(inc, 6, AddressingMode::Absolute),
        0xFEu8 => opcode!(inc, 7, AddressingMode::AbsoluteXIndexed),
        0x4Cu8 => opcode!(jmp, 3, AddressingMode::Absolute),
        0x6Cu8 => opcode!(jmp, 5, AddressingMode::Indirect),
        0x20u8 => opcode!(jsr, 6, AddressingMode::Absolute),
        0xA9u8 => opcode!(lda, 2, AddressingMode::Immediate),
        0xA5u8 => opcode!(lda, 3, AddressingMode::ZeroPage),
        0xB5u8 => opcode!(lda, 4, AddressingMode::ZeroPageXIndexed),
        0xADu8 => opcode!(lda, 4, AddressingMode::Absolute),
        0xBDu8 => opcode!(lda, 4, AddressingMode::AbsoluteXIndexed),
        0xB9u8 => opcode!(lda, 4, AddressingMode::AbsoluteYIndexed),
        0xA1u8 => opcode!(lda, 6, AddressingMode::IndirectX),
        0xB1u8 => opcode!(lda, 5, AddressingMode::IndirectY),
        0xA2u8 => opcode!(ldx, 2, AddressingMode::Immediate),
        0xA6u8 => opcode!(ldx, 3, AddressingMode::ZeroPage),
        0xB6u8 => opcode!(ldx, 4, AddressingMode::ZeroPageYIndexed),
        0xAEu8 => opcode!(ldx, 4, AddressingMode::Absolute),
        0xBEu8 => opcode!(ldx, 4, AddressingMode::AbsoluteYIndexed),
        0xA0u8 => opcode!(ldy, 2, AddressingMode::Immediate),
        0xA4u8 => opcode!(ldy, 3, AddressingMode::ZeroPage),
        0xB4u8 => opcode!(ldy, 4, AddressingMode::ZeroPageXIndexed),
        0xACu8 => opcode!(ldy, 4, AddressingMode::Absolute),
        0xBCu8 => opcode!(ldy, 4, AddressingMode::AbsoluteXIndexed),
        0x4Au8 => opcode!(lsr, 2, AddressingMode::Implied),
        0x46u8 => opcode!(lsr, 5, AddressingMode::ZeroPage),
        0x56u8 => opcode!(lsr, 6, AddressingMode::ZeroPageXIndexed),
        0x4Eu8 => opcode!(lsr, 6, AddressingMode::Absolute),
        0x5Eu8 => opcode!(lsr, 7, AddressingMode::AbsoluteXIndexed),
        0xEAu8 => opcode!(nop, 2, AddressingMode::Implied),
        0x09u8 => opcode!(ora, 2, AddressingMode::Immediate),
        0x05u8 => opcode!(ora, 3, AddressingMode::ZeroPage),
        0x15u8 => opcode!(ora, 4, AddressingMode::ZeroPageXIndexed),
        0x0Du8 => opcode!(ora, 4, AddressingMode::Absolute),
        0x1Du8 => opcode!(ora, 4, AddressingMode::AbsoluteXIndexed),
        0x19u8 => opcode!(ora, 4, AddressingMode::AbsoluteYIndexed),
        0x01u8 => opcode!(ora, 6, AddressingMode::IndirectX),
        0x11u8 => opcode!(ora, 5, AddressingMode::IndirectY),
        0xAAu8 => opcode!(tax, 2, AddressingMode::Implied),
        0x8Au8 => opcode!(txa, 2, AddressingMode::Implied),
        0xCAu8 => opcode!(dex, 2, AddressingMode::Implied),
        0xE8u8 => opcode!(inx, 2, AddressingMode::Implied),
        0xA8u8 => opcode!(tay, 2, AddressingMode::Implied),
        0x98u8 => opcode!(tya, 2, AddressingMode::Implied),
        0x88u8 => opcode!(dey, 2, AddressingMode::Implied),
        0xC8u8 => opcode!(iny, 2, AddressingMode::Implied),
        0x2Au8 => opcode!(rol, 2, AddressingMode::Implied),
        0x26u8 => opcode!(rol, 5, AddressingMode::ZeroPage),
        0x36u8 => opcode!(rol, 6, AddressingMode::ZeroPageXIndexed),
        0x2Eu8 => opcode!(rol, 6, AddressingMode::Absolute),
        0x3Eu8 => opcode!(rol, 7, AddressingMode::AbsoluteXIndexed),
        0x6Au8 => opcode!(ror, 2, AddressingMode::Implied),
        0x66u8 => opcode!(ror, 5, AddressingMode::ZeroPage),
        0x76u8 => opcode!(ror, 6, AddressingMode::ZeroPageXIndexed),
        0x6Eu8 => opcode!(ror, 6, AddressingMode::Absolute),
        0x7Eu8 => opcode!(ror, 7, AddressingMode::AbsoluteXIndexed),
        0x40u8 => opcode!(rti, 6, AddressingMode::Implied),
        0x60u8 => opcode!(rts, 6, AddressingMode::Implied),
        0xE9u8 => opcode!(sbc, 2, AddressingMode::Immediate),
        0xE5u8 => opcode!(sbc, 3, AddressingMode::ZeroPage),
        0xF5u8 => opcode!(sbc, 4, AddressingMode::ZeroPageXIndexed),
        0xEDu8 => opcode!(sbc, 4, AddressingMode::Absolute),
        0xFDu8 => opcode!(sbc, 4, AddressingMode::AbsoluteXIndexed),
        0xF9u8 => opcode!(sbc, 4, AddressingMode::AbsoluteYIndexed),
        0xE1u8 => opcode!(sbc, 6, AddressingMode::IndirectX),
        0xF1u8 => opcode!(sbc, 5, AddressingMode::IndirectY),
        0x85u8 => opcode!(sta, 3, AddressingMode::ZeroPage),
        0x95u8 => opcode!(sta, 4, AddressingMode::ZeroPageXIndexed),
        0x8Du8 => opcode!(sta, 4, AddressingMode::Absolute),
        0x9Du8 => opcode!(sta, 5, AddressingMode::AbsoluteXIndexed),
        0x99u8 => opcode!(sta, 5, AddressingMode::AbsoluteYIndexed),
        0x81u8 => opcode!(sta, 6, AddressingMode::IndirectX),
        0x91u8 => opcode!(sta, 6, AddressingMode::IndirectY),
        0x9Au8 => opcode!(txs, 2, AddressingMode::Implied),
        0xBAu8 => opcode!(tsx, 2, AddressingMode::Implied),
        0x48u8 => opcode!(pha, 3, AddressingMode::Implied),
        0x68u8 => opcode!(pla, 4, AddressingMode::Implied),
        0x08u8 => opcode!(php, 3, AddressingMode::Implied),
        0x28u8 => opcode!(plp, 4, AddressingMode::Implied),
        0x86u8 => opcode!(stx, 3, AddressingMode::ZeroPage),
        0x96u8 => opcode!(stx, 4, AddressingMode::ZeroPageYIndexed),
        0x8Eu8 => opcode!(stx, 4, AddressingMode::Absolute),
        0x84u8 => opcode!(sty, 3, AddressingMode::ZeroPage),
        0x94u8 => opcode!(sty, 4, AddressingMode::ZeroPageXIndexed),
        0x8Cu8 => opcode!(sty, 4, AddressingMode::Absolute),
    };
}

macro_rules! is_negative
{
    ($arg : expr) => {{ ($arg)>>7==1 }}
}

fn adc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn and(nes : &mut System, _address : usize, value : u8)
{
    //logical AND value with Accumulator
    let new_value = nes.cpu.A & value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn asl(nes : &mut System, address : usize, value : u8)
{
    //arithmetic shift left
    nes.cpu.flags.carry = (value & 0b10000000) >> 7 == 1;
    let new_value = value << 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn bit(nes : &mut System, _address : usize, value : u8)
{
    //test bits
    nes.cpu.flags.negative = (value & 0b10000000) >> 7 == 1;
    nes.cpu.flags.overflow = (value & 0b01000000) >> 6 == 1;
    nes.cpu.flags.zero = (value & nes.cpu.A) == 0;
}

fn bpl(nes : &mut System, _address : usize, offset : u8)
{
    //branch on result plus
    if !nes.cpu.flags.negative
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bmi(nes : &mut System, _address : usize, offset : u8)
{
    //branch on result minus
    if nes.cpu.flags.negative
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bvc(nes : &mut System, _address : usize, offset : u8)
{
    //branch on overflow clear
    if !nes.cpu.flags.overflow
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bvs(nes : &mut System, _address : usize, offset : u8)
{
    //branch on overflow set
    if nes.cpu.flags.overflow
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bcc(nes : &mut System, _address : usize, offset : u8)
{
    //branch on carry clear
    if !nes.cpu.flags.carry
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bcs(nes : &mut System, _address : usize, offset : u8)
{
    //branch on carry set
    if nes.cpu.flags.carry
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn bne(nes : &mut System, _address : usize, offset : u8)
{
    //branch on result not zero
    if !nes.cpu.flags.zero
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn beq(nes : &mut System, _address : usize, offset : u8)
{
    //branch on result zero
    if nes.cpu.flags.zero
    {
        nes.cpu.program_counter += offset as usize;
    }
}

fn brk(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn cmp(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn cpx(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn cpy(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn dec(nes : &mut System, address : usize, value : u8)
{
    //decrement memory
    let new_value = value-1;
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn eor(nes : &mut System, _address : usize, value : u8)
{
    //logical XOR value with Accumulator
    let new_value = nes.cpu.A ^ value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn clc(nes : &mut System, _address : usize, _value : u8)
{
    //clear carry flag
    nes.cpu.flags.carry = false;
}

fn sec(nes : &mut System, _address : usize, _value : u8)
{
    //set carry flag
    nes.cpu.flags.carry = true;
}

fn cli(nes : &mut System, _address : usize, _value : u8)
{
    //clear interrupt disable flag
    nes.cpu.flags.interrupt = false;
}

fn sei(nes : &mut System, _address : usize, _value : u8)
{
    //set interrupt disable flag
    nes.cpu.flags.interrupt = true;
}

fn clv(nes : &mut System, _address : usize, _value : u8)
{
    //clear overflow flag
    nes.cpu.flags.overflow = false;
}

fn cld(nes : &mut System, _address : usize, _value : u8)
{
    //clear decimal mode flag
    nes.cpu.flags.decimal = false;
}

fn sed(nes : &mut System, _address : usize, _value : u8)
{
    //set decimal mode flag
    nes.cpu.flags.decimal = true;
}

fn inc(nes : &mut System, address : usize, value : u8)
{
    //increment memory
    let new_value = value+1;
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn jmp(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn jsr(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn lda(nes : &mut System, _address : usize, value: u8)
{
    //load memory into Accumulator
    nes.cpu.A = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldx(nes : &mut System, _address : usize, value : u8)
{
    //load memory into register X
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldy(nes : &mut System, _address : usize, value: u8)
{
    //load memory into register Y
    nes.cpu.Y = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn lsr(nes : &mut System, address : usize, value : u8)
{
    //logical shift right
    nes.cpu.flags.carry = value & 0b00000001 == 1;
    let new_value = value >> 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn nop(_nes : &mut System, _address : usize, _value : u8)
{
    //no operation!
}

fn ora(nes : &mut System, _address : usize, value : u8)
{
    //logical OR value with Accumulator
    let new_value = nes.cpu.A | value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tax(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Accumulator into register X
    let new_value = nes.cpu.A;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn txa(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register X into Accumulator
    let new_value = nes.cpu.X;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dex(nes : &mut System, _address : usize, _value : u8)
{
    //decrement register X
    let new_value = nes.cpu.X-1;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn inx(nes : &mut System, _address : usize, _value : u8)
{
    //increment register X
    let new_value = nes.cpu.X+1;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tay(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Accumulator into register Y
    let new_value = nes.cpu.A;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tya(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register Y into Accumulator
    let new_value = nes.cpu.Y;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dey(nes : &mut System, _address : usize, _value : u8)
{
    //decrement register Y
    let new_value = nes.cpu.Y-1;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn iny(nes : &mut System, _address : usize, _value : u8)
{
    //increment register Y
    let new_value = nes.cpu.Y+1;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn rol(nes : &mut System, address : usize, value : u8)
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

fn ror(nes : &mut System, address : usize, value : u8)
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

fn rti(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn rts(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn sbc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn sta(nes : &mut System, address : usize, _value : u8)
{
    //store Accumulator into memory
    nes.ram.put(address, nes.cpu.A);
}

fn txs(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register X into Stack Pointer
    nes.cpu.stack.set_pointer(nes.cpu.X);
}

fn tsx(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Stack Pointer into register X
    let new_value = nes.cpu.stack.get_pointer();
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn pha(nes : &mut System, _address : usize, _value : u8)
{
    //push Accumulator on Stack
    CPUStack::push(nes, nes.cpu.A);
}

fn pla(nes : &mut System, _address : usize, _value : u8)
{
    //pop Stack into Accumulator
    let new_value = CPUStack::pop(nes);
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn php(nes : &mut System, _address : usize, _value : u8)
{
    //push Processor Flags on Stack
    let byte = nes.cpu.flags.to_byte();
    CPUStack::push(nes, byte);
}

fn plp(nes : &mut System, _address : usize, _value : u8)
{
    //pull Stack into Processor Flags
    let byte = CPUStack::pop(nes);
    nes.cpu.flags = CPUFlags::from_byte(byte);
}

fn stx(nes : &mut System, address : usize, _value : u8)
{
    //store register X into memory
    nes.ram.put(address, nes.cpu.A);
}

fn sty(nes : &mut System, address : usize, _value : u8)
{
    //store register Y into memory
    nes.ram.put(address, nes.cpu.A);
}

