use std::collections::HashMap;
use maplit2::hashmap;
use crate::system::cpu::AddressingMode;
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

fn is_negative(arg : u8) -> bool
{
    return arg>>7==1;
}

fn adc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn and(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn asl(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bit(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bpl(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bmi(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bvc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bvs(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bcc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bcs(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn bne(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn beq(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
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

fn dec(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn eor(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn clc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn sec(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn cli(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn sei(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn clv(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn cld(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn sed(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn inc(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
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
    nes.cpu.flags.negative = is_negative(value);
}

fn ldx(nes : &mut System, _address : usize, value : u8)
{
    //load memory into register X
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn ldy(nes : &mut System, _address : usize, value: u8)
{
    //load memory into register Y
    nes.cpu.Y = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn lsr(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn nop(_nes : &mut System, _address : usize, _value : u8)
{
    //no operation!
}

fn ora(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn tax(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Accumulator into register X
    let value = nes.cpu.A;
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn txa(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register X into Accumulator
    let value = nes.cpu.X;
    nes.cpu.A = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn dex(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn inx(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn tay(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Accumulator into register Y
    let value = nes.cpu.A;
    nes.cpu.Y = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn tya(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register Y into Accumulator
    let value = nes.cpu.Y;
    nes.cpu.A = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);
}

fn dey(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn iny(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn rol(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn ror(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
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
    nes.ram[address] = nes.cpu.A;
}

fn txs(nes : &mut System, _address : usize, _value : u8)
{
    //transfer register X into Stack Pointer
    //todo nes.cpu.SP = nes.cpu.X;
}

fn tsx(nes : &mut System, _address : usize, _value : u8)
{
    //transfer Stack Pointer into register X
    /*todo let value = nes.cpu.SP;
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative(value);*/
}

fn pha(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn pla(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn php(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn plp(_nes : &mut System, _address : usize, _value : u8)
{
    todo!()
}

fn stx(nes : &mut System, address : usize, _value : u8)
{
    //store register X into memory
    nes.ram[address] = nes.cpu.X;
}

fn sty(nes : &mut System, address : usize, _value : u8)
{
    //store register Y into memory
    nes.ram[address] = nes.cpu.Y;
}

