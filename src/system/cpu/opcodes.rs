use crate::system::cpu::CPU;
use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::program_iterator::AddressingMode;
use crate::system::cpu::stack::CPUStack;
use crate::system::cpu::clock::ExpectedDuration;
use crate::system::cpu::interrupts::CPUInterrupts;
use crate::system::{address, byte};

pub struct Opcode
{
    pub key : byte,
    pub name : String,
    pub addressing_mode : AddressingMode,
    pub lambda : fn(&mut CPU, address, byte) -> (),
    pub expected_duration : ExpectedDuration,
}

macro_rules! opcode
{
    ($key : expr, $name : expr, $expected_duration : expr, $addressing_mode : expr) =>
    {
        Opcode
        {
            key: $key, lambda: $name,
            name: stringify!($name).to_uppercase(),
            expected_duration: $expected_duration,
            addressing_mode: $addressing_mode,
        }
    }
}

pub fn build_opcodes_slice() -> Box<[Opcode]>
{
    let mut opcodes = vec![
        opcode!(0x69, adc, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x65, adc, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x75, adc, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x6D, adc, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x7D, adc, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x79, adc, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0x61, adc, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x71, adc, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0x29, and, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x25, and, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x35, and, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x2D, and, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x3D, and, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x39, and, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0x21, and, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x31, and, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0x0A, asl, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x06, asl, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x16, asl, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x0E, asl, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x1E, asl, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x24, bit, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x2C, bit, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x10, bpl, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0x30, bmi, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0x50, bvc, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0x70, bvs, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0x90, bcc, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0xB0, bcs, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0xD0, bne, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0xF0, beq, ExpectedDuration::bra, AddressingMode::Relative),
        opcode!(0x00, brk, ExpectedDuration::_7,  AddressingMode::Implied),
        opcode!(0xC9, cmp, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xC5, cmp, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xD5, cmp, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xCD, cmp, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xDD, cmp, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xD9, cmp, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xC1, cmp, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0xD1, cmp, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0xE0, cpx, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xE4, cpx, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xEC, cpx, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xC0, cpy, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xC4, cpy, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xCC, cpy, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xC6, dec, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0xD6, dec, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xCE, dec, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0xDE, dec, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x49, eor, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x45, eor, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x55, eor, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x4D, eor, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x5D, eor, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x59, eor, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0x41, eor, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x51, eor, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0x18, clc, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x38, sec, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x58, cli, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x78, sei, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xB8, clv, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xD8, cld, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xF8, sed, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xE6, inc, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0xF6, inc, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xEE, inc, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0xFE, inc, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x4C, jmp, ExpectedDuration::_3,  AddressingMode::Absolute),
        opcode!(0x6C, jmp, ExpectedDuration::_5,  AddressingMode::Indirect),
        opcode!(0x20, jsr, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0xA9, lda, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xA5, lda, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xB5, lda, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xAD, lda, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xBD, lda, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xB9, lda, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xA1, lda, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0xB1, lda, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0xA2, ldx, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xA6, ldx, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xB6, ldx, ExpectedDuration::_4,  AddressingMode::ZeroPageYIndexed),
        opcode!(0xAE, ldx, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xBE, ldx, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xA0, ldy, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xA4, ldy, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xB4, ldy, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xAC, ldy, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xBC, ldy, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x4A, lsr, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x46, lsr, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x56, lsr, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x4E, lsr, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x5E, lsr, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0xEA, nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x09, ora, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x05, ora, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x15, ora, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x0D, ora, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x1D, ora, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x19, ora, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0x01, ora, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x11, ora, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0xAA, tax, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x8A, txa, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xCA, dex, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xE8, inx, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xA8, tay, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x98, tya, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x88, dey, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xC8, iny, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x2A, rol, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x26, rol, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x36, rol, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x2E, rol, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x3E, rol, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x6A, ror, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x66, ror, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x76, ror, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x6E, ror, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x7E, ror, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x40, rti, ExpectedDuration::_6,  AddressingMode::Implied),
        opcode!(0x60, rts, ExpectedDuration::_6,  AddressingMode::Implied),
        opcode!(0xE9, sbc, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xE5, sbc, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xF5, sbc, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xED, sbc, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xFD, sbc, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xF9, sbc, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xE1, sbc, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0xF1, sbc, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0x85, sta, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x95, sta, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x8D, sta, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x9D, sta, ExpectedDuration::_5,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x99, sta, ExpectedDuration::_5,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x81, sta, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x91, sta, ExpectedDuration::_6,  AddressingMode::IndirectY),
        opcode!(0x9A, txs, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xBA, tsx, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x48, pha, ExpectedDuration::_3,  AddressingMode::Implied),
        opcode!(0x68, pla, ExpectedDuration::_4,  AddressingMode::Implied),
        opcode!(0x08, php, ExpectedDuration::_3,  AddressingMode::Implied),
        opcode!(0x28, plp, ExpectedDuration::_4,  AddressingMode::Implied),
        opcode!(0x86, stx, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x96, stx, ExpectedDuration::_4,  AddressingMode::ZeroPageYIndexed),
        opcode!(0x8E, stx, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x84, sty, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x94, sty, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x8C, sty, ExpectedDuration::_4,  AddressingMode::Absolute),

        opcode!(0x0B, unofficial_aac, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x2B, unofficial_aac, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x87, unofficial_aax, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x97, unofficial_aax, ExpectedDuration::_4,  AddressingMode::ZeroPageYIndexed),
        opcode!(0x83, unofficial_aax, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0x8F, unofficial_aax, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x6B, unofficial_aar, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x4B, unofficial_asr, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xAB, unofficial_atx, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x9F, unofficial_axa, ExpectedDuration::_5,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x93, unofficial_axa, ExpectedDuration::_6,  AddressingMode::IndirectY),
        opcode!(0xCB, unofficial_axs, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xC7, unofficial_dcp, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0xD7, unofficial_dcp, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xCF, unofficial_dcp, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0xDF, unofficial_dcp, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0xDB, unofficial_dcp, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0xC3, unofficial_dcp, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0xD3, unofficial_dcp, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0x04, unofficial_nop, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x14, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x34, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x44, unofficial_nop, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x54, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x64, unofficial_nop, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0x74, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x80, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x82, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x89, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xC2, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xD4, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xE2, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0xF4, unofficial_nop, ExpectedDuration::_4,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x1A, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x3A, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x5A, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x7A, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xDA, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0xFA, unofficial_nop, ExpectedDuration::_2,  AddressingMode::Implied),
        opcode!(0x0C, unofficial_nop, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0x1C, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x3C, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x5C, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0x7C, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xDC, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xFC, unofficial_nop, ExpectedDuration::_4p, AddressingMode::AbsoluteXIndexed),
        opcode!(0xE7, unofficial_isc, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0xF7, unofficial_isc, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0xEF, unofficial_isc, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0xFF, unofficial_isc, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0xFB, unofficial_isc, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0xE3, unofficial_isc, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0xF3, unofficial_isc, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0x02, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x12, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x22, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x32, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x42, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x52, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x62, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x72, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0x92, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0xB2, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0xD2, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0xF2, unofficial_hlt, ExpectedDuration::_2,  AddressingMode::Unknown),
        opcode!(0xBB, unofficial_lar, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xA7, unofficial_lax, ExpectedDuration::_3,  AddressingMode::ZeroPage),
        opcode!(0xB7, unofficial_lax, ExpectedDuration::_4,  AddressingMode::ZeroPageYIndexed),
        opcode!(0xAF, unofficial_lax, ExpectedDuration::_4,  AddressingMode::Absolute),
        opcode!(0xBF, unofficial_lax, ExpectedDuration::_4p, AddressingMode::AbsoluteYIndexed),
        opcode!(0xA3, unofficial_lax, ExpectedDuration::_6,  AddressingMode::IndirectX),
        opcode!(0xB3, unofficial_lax, ExpectedDuration::_5p, AddressingMode::IndirectY),
        opcode!(0x27, unofficial_rla, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x37, unofficial_rla, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x2F, unofficial_rla, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x3F, unofficial_rla, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x3B, unofficial_rla, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x23, unofficial_rla, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0x33, unofficial_rla, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0x67, unofficial_rra, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x77, unofficial_rra, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x6F, unofficial_rra, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x7F, unofficial_rra, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x7B, unofficial_rra, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x63, unofficial_rra, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0x73, unofficial_rra, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0xEB, unofficial_sbc, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x07, unofficial_slo, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x17, unofficial_slo, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x0F, unofficial_slo, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x1F, unofficial_slo, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x1B, unofficial_slo, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x03, unofficial_slo, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0x13, unofficial_slo, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0x47, unofficial_sre, ExpectedDuration::_5,  AddressingMode::ZeroPage),
        opcode!(0x57, unofficial_sre, ExpectedDuration::_6,  AddressingMode::ZeroPageXIndexed),
        opcode!(0x4F, unofficial_sre, ExpectedDuration::_6,  AddressingMode::Absolute),
        opcode!(0x5F, unofficial_sre, ExpectedDuration::_7,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x5B, unofficial_sre, ExpectedDuration::_7,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x43, unofficial_sre, ExpectedDuration::_8,  AddressingMode::IndirectX),
        opcode!(0x53, unofficial_sre, ExpectedDuration::_8,  AddressingMode::IndirectY),
        opcode!(0x9E, unofficial_sxa, ExpectedDuration::_5,  AddressingMode::AbsoluteYIndexed),
        opcode!(0x9C, unofficial_sya, ExpectedDuration::_5,  AddressingMode::AbsoluteXIndexed),
        opcode!(0x8B, unofficial_xaa, ExpectedDuration::_2,  AddressingMode::Immediate),
        opcode!(0x9B, unofficial_xas, ExpectedDuration::_5,  AddressingMode::AbsoluteYIndexed),
    ];

    (0x00..=0xFFu8).into_iter() //find unimplemented opcodes
        .filter(|key| opcodes.iter().find(|opcode| opcode.key==*key).is_none())
        .for_each(|key| { panic!("CPU Opcode {:#04X} is not implemented!", key) });

    (0x00..=0xFFu8).into_iter() //find duplicates
        .filter(|key| opcodes.iter().filter(|opcode| opcode.key==*key).count()>=2)
        .for_each(|key| { panic!("CPU Opcode {:#04X} is implemented twice!", key) });

    opcodes.sort_by(|o1, o2| byte::cmp(&o1.key, &o2.key));
    return opcodes.into_boxed_slice();
}

macro_rules! isneg { ($arg : expr) => { ($arg)>0x7F } }

fn adc(cpu : &mut CPU, _address : address, value : byte)
{
    //add with carry
    let (temp_value, did_first_overflow_occur) = cpu.A.overflowing_add(value);
    let (new_value, did_second_overflow_occur) = temp_value.overflowing_add(cpu.flags.carry as u8);
    let old_accumulator = cpu.A;
    cpu.A = new_value;
    cpu.flags.carry = did_first_overflow_occur | did_second_overflow_occur;
    cpu.flags.overflow = (!(isneg!(old_accumulator) ^ isneg!(value))) && (isneg!(old_accumulator) ^ isneg!(new_value));
    cpu.flags.negative = isneg!(new_value);
    cpu.flags.zero = new_value==0;
}

fn and(cpu : &mut CPU, _address : address, value : byte)
{
    //logical AND value with Accumulator
    let new_value = cpu.A & value;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn asl(cpu : &mut CPU, address : address, value : byte)
{
    //arithmetic shift left
    cpu.flags.carry = (value & 0b10000000) >> 7 == 1;
    let new_value = value << 1;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
    if address > 0 { cpu.bus.put(address, new_value); }
    else { cpu.A = new_value; }
}

fn bit(cpu : &mut CPU, _address : address, value : byte)
{
    //test bits
    cpu.flags.negative = (value & 0b10000000) >> 7 == 1;
    cpu.flags.overflow = (value & 0b01000000) >> 6 == 1;
    cpu.flags.zero = (value & cpu.A) == 0;
}

fn branch(cpu : &mut CPU, signed_offset : byte)
{
    let abs_offset = ((signed_offset as i8) as i16).abs() as address;
    if isneg!(signed_offset)
    {
        cpu.program_counter = cpu.program_counter.wrapping_sub(abs_offset);
        cpu.clock.notify_branch_taken();
    }
    else
    {
        cpu.program_counter = cpu.program_counter.wrapping_add(abs_offset);
        cpu.clock.notify_branch_taken();
    }
}

fn bpl(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on result plus
    if !cpu.flags.negative
    {
        branch(cpu, offset);
    }
}

fn bmi(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on result minus
    if cpu.flags.negative
    {
        branch(cpu, offset);
    }
}

fn bvc(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on overflow clear
    if !cpu.flags.overflow
    {
        branch(cpu, offset);
    }
}

fn bvs(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on overflow set
    if cpu.flags.overflow
    {
        branch(cpu, offset);
    }
}

fn bcc(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on carry clear
    if !cpu.flags.carry
    {
        branch(cpu, offset);
    }
}

fn bcs(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on carry set
    if cpu.flags.carry
    {
        branch(cpu, offset);
    }
}

fn bne(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on result not zero
    if !cpu.flags.zero
    {
        branch(cpu, offset);
    }
}

fn beq(cpu : &mut CPU, _address : address, offset : byte)
{
    //branch on result zero
    if cpu.flags.zero
    {
        branch(cpu, offset);
    }
}

fn brk(cpu : &mut CPU, _address : address, _value : byte)
{
    //break!
    CPUInterrupts::software_irq(cpu);
}

fn cmp(cpu : &mut CPU, _address : address, value : byte)
{
    //substract Accumulator minus memory, set flags
    let (left, right) = (cpu.A as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = new_value<0;
    cpu.flags.carry = value<=cpu.A;
}

fn cpx(cpu : &mut CPU, _address : address, value : byte)
{
    //substract X minus memory, set flags
    let (left, right) = (cpu.X as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = new_value<0;
    cpu.flags.carry = value<=cpu.X;
}

fn cpy(cpu : &mut CPU, _address : address, value : byte)
{
    //substract Accumulator minus memory, set flags
    let (left, right) = (cpu.Y as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = new_value<0;
    cpu.flags.carry = value<=cpu.Y;
}

fn dec(cpu : &mut CPU, address : address, value : byte)
{
    //decrement memory
    let new_value = value.wrapping_sub(1);
    cpu.bus.put(address, new_value);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn eor(cpu : &mut CPU, _address : address, value : byte)
{
    //logical XOR value with Accumulator
    let new_value = cpu.A ^ value;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn clc(cpu : &mut CPU, _address : address, _value : byte)
{
    //clear carry flag
    cpu.flags.carry = false;
}

fn sec(cpu : &mut CPU, _address : address, _value : byte)
{
    //set carry flag
    cpu.flags.carry = true;
}

fn cli(cpu : &mut CPU, _address : address, _value : byte)
{
    //clear interrupt disable flag
    cpu.flags.interrupt = false;
}

fn sei(cpu : &mut CPU, _address : address, _value : byte)
{
    //set interrupt disable flag
    cpu.flags.interrupt = true;
}

fn clv(cpu : &mut CPU, _address : address, _value : byte)
{
    //clear overflow flag
    cpu.flags.overflow = false;
}

fn cld(cpu : &mut CPU, _address : address, _value : byte)
{
    //clear decimal mode flag
    cpu.flags.decimal = false;
}

fn sed(cpu : &mut CPU, _address : address, _value : byte)
{
    //set decimal mode flag
    cpu.flags.decimal = true;
}

fn inc(cpu : &mut CPU, address : address, value : byte)
{
    //increment memory
    let new_value = value.wrapping_add(1);
    cpu.bus.put(address, new_value);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn jmp(cpu : &mut CPU, address : address, _value : byte)
{
    //goto / jump
    cpu.program_counter = address;
}

fn jsr(cpu : &mut CPU, address : address, _value : byte)
{
    //jump to subroutine
    CPUStack::push_address(cpu, cpu.program_counter-1);
    cpu.program_counter = address;
}

fn lda(cpu : &mut CPU, _address : address, value : byte)
{
    //load memory into Accumulator
    cpu.A = value;
    cpu.flags.zero = value==0;
    cpu.flags.negative = isneg!(value);
}

fn ldx(cpu : &mut CPU, _address : address, value : byte)
{
    //load memory into register X
    cpu.X = value;
    cpu.flags.zero = value==0;
    cpu.flags.negative = isneg!(value);
}

fn ldy(cpu : &mut CPU, _address : address, value: byte)
{
    //load memory into register Y
    cpu.Y = value;
    cpu.flags.zero = value==0;
    cpu.flags.negative = isneg!(value);
}

fn lsr(cpu : &mut CPU, address : address, value : byte)
{
    //logical shift right
    cpu.flags.carry = value & 0b00000001 == 1;
    let new_value = value >> 1;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
    if address > 0 { cpu.bus.put(address, new_value); }
    else { cpu.A = new_value; }
}

fn nop(_cpu : &mut CPU, _address : address, _value : byte)
{
    //no operation!
}

fn ora(cpu : &mut CPU, _address : address, value : byte)
{
    //logical OR value with Accumulator
    let new_value = cpu.A | value;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn tax(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer Accumulator into register X
    let new_value = cpu.A;
    cpu.X = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn txa(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer register X into Accumulator
    let new_value = cpu.X;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn dex(cpu : &mut CPU, _address : address, _value : byte)
{
    //decrement register X
    let new_value = cpu.X.wrapping_sub(1);
    cpu.X = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn inx(cpu : &mut CPU, _address : address, _value : byte)
{
    //increment register X
    let new_value = cpu.X.wrapping_add(1);
    cpu.X = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn tay(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer Accumulator into register Y
    let new_value = cpu.A;
    cpu.Y = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn tya(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer register Y into Accumulator
    let new_value = cpu.Y;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn dey(cpu : &mut CPU, _address : address, _value : byte)
{
    //decrement register Y
    let new_value = cpu.Y.wrapping_sub(1);
    cpu.Y = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn iny(cpu : &mut CPU, _address : address, _value : byte)
{
    //increment register Y
    let new_value = cpu.Y.wrapping_add(1);
    cpu.Y = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn rol(cpu : &mut CPU, address : address, value : byte)
{
    //rotate left
    let bit_to_push = cpu.flags.carry as u8;
    let bit_to_remove = (value & 0b10000000) >> 7;
    cpu.flags.carry = bit_to_remove == 1;
    let new_value = (value << 1) | bit_to_push;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
    if address > 0 { cpu.bus.put(address, new_value); }
    else { cpu.A = new_value; }
}

fn ror(cpu : &mut CPU, address : address, value : byte)
{
    //rotate right
    let bit_to_push = cpu.flags.carry as u8;
    let bit_to_pop = value & 0b00000001;
    cpu.flags.carry = bit_to_pop==1;
    let new_value = (value >> 1) | (bit_to_push << 7);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
    if address > 0 { cpu.bus.put(address, new_value); }
    else { cpu.A = new_value; }
}

fn rti(cpu : &mut CPU, _address : address, _value : byte)
{
    //return from interrupt
    let mut cpu_flags_to_restore = CPUFlags::from_byte(CPUStack::pop_byte(cpu));
    cpu_flags_to_restore._break = false;
    cpu_flags_to_restore.reserved = false;

    let new_address = CPUStack::pop_address(cpu);
    cpu.program_counter = new_address;
    cpu.flags = cpu_flags_to_restore;
}

fn rts(cpu : &mut CPU, _address : address, _value : byte)
{
    //return from subroutine
    let new_address = CPUStack::pop_address(cpu);
    cpu.program_counter = new_address+1;
}

fn sbc(cpu : &mut CPU, _address : address, value : byte)
{
    //substract with carry
    let (temp_value, did_first_overflow_occur) = cpu.A.overflowing_sub(value);
    let (new_value, did_second_overflow_occur) = temp_value.overflowing_sub((!cpu.flags.carry) as u8);
    let old_accumulator = cpu.A;
    cpu.A = new_value;
    cpu.flags.carry = !(did_first_overflow_occur | did_second_overflow_occur);
    cpu.flags.overflow = (isneg!(old_accumulator) ^ isneg!(value)) && (isneg!(old_accumulator) ^ isneg!(new_value));
    cpu.flags.negative = isneg!(new_value);
    cpu.flags.zero = new_value==0;
}

fn sta(cpu : &mut CPU, address : address, _value : byte)
{
    //store Accumulator into memory
    cpu.bus.put(address, cpu.A);
}

fn txs(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer register X into Stack Pointer
    CPUStack::set_pointer(cpu, cpu.X);
}

fn tsx(cpu : &mut CPU, _address : address, _value : byte)
{
    //transfer Stack Pointer into register X
    let new_value = CPUStack::get_pointer(cpu);
    cpu.X = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn pha(cpu : &mut CPU, _address : address, _value : byte)
{
    //push Accumulator on Stack
    CPUStack::push_byte(cpu, cpu.A);
}

fn pla(cpu : &mut CPU, _address : address, _value : byte)
{
    //pop Stack into Accumulator
    let new_value = CPUStack::pop_byte(cpu);
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn php(cpu : &mut CPU, _address : address, _value : byte)
{
    //push Processor Flags on Stack
    let mut flags = cpu.flags.clone();
    flags._break = true;
    flags.reserved = true;
    let byte = flags.to_byte();
    CPUStack::push_byte(cpu, byte);
}

fn plp(cpu : &mut CPU, _address : address, _value : byte)
{
    //pull Stack into Processor Flags
    let mut flags = CPUFlags::from_byte(CPUStack::pop_byte(cpu));
    flags._break = false;
    flags.reserved = false;
    cpu.flags = flags;
}

fn stx(cpu : &mut CPU, address : address, _value : byte)
{
    //store register X into memory
    cpu.bus.put(address, cpu.X);
}

fn sty(cpu : &mut CPU, address : address, _value : byte)
{
    //store register Y into memory
    cpu.bus.put(address, cpu.Y);
}

fn unofficial_aac(cpu : &mut CPU, _address : address, value : byte)
{
    //AND byte with accumulator. If result is negative then carry is set
    let new_value = cpu.A & value;
    cpu.A = new_value;
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
    cpu.flags.carry = isneg!(new_value);
}

fn unofficial_aax(cpu : &mut CPU, address : address, _value : byte)
{
    //AND X register with accumulator and store result in memory
    let new_value = cpu.X & cpu.A;
    cpu.bus.put(address, new_value);
}

fn unofficial_aar(cpu : &mut CPU, _address : address, value : byte)
{
    //AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6
    cpu.A = cpu.A & value;
    ror(cpu, 0, cpu.A);
    let bit5 = ((cpu.A & 0b00100000) >> 5)==1;
    let bit6 = ((cpu.A & 0b01000000) >> 6)==1;
    if bit5 && bit6 { cpu.flags.carry = true; cpu.flags.overflow = false; }
    else if !bit5 && !bit6 { cpu.flags.carry = false; cpu.flags.overflow = false; }
    else if bit5 { cpu.flags.carry = false; cpu.flags.overflow = true; }
    else { cpu.flags.carry = true; cpu.flags.overflow = true; }
}

fn unofficial_asr(cpu : &mut CPU, _address : address, value : byte)
{
    //AND byte with accumulator, then shift right one bit in accumulator
    cpu.A = cpu.A & value;
    lsr(cpu, 0, cpu.A);
}

fn unofficial_atx(cpu : &mut CPU, address : address, value : byte)
{
    //load accumulator then transfer accumulator to X register
    lda(cpu, address, value);
    tax(cpu, address, value);
}

fn unofficial_axa(cpu : &mut CPU, address : address, _value : byte)
{
    //AND X register with accumulator then AND result with 7 and store in memory
    cpu.bus.put(address, (cpu.X & cpu.A) & 7);
}

fn unofficial_axs(cpu : &mut CPU, _address : address, value : byte)
{
    //AND X register with accumulator and store result in X register, then subtract byte from X register (without borrow)
    let new_value_16b = ((cpu.A & cpu.X) as u32).wrapping_sub(value as u32);
    let new_value_8b = (new_value_16b & 0xFF) as u8;
    cpu.X = new_value_8b;
    cpu.flags.zero = new_value_8b==0;
    cpu.flags.negative = isneg!(new_value_8b);
    cpu.flags.carry = ((new_value_16b >> 8) & 0x01) ^ 0x01 == 0x01;
}

fn unofficial_dcp(cpu : &mut CPU, address : address, value : byte)
{
    //decrement value and save it to memory, then compare it with accumulator
    let new_value = value.wrapping_sub(1);
    cpu.bus.put(address, new_value);
    cmp(cpu, address, new_value);
}

fn unofficial_nop(_cpu : &mut CPU, _address : address, _value : byte)
{
    //no operation
}

fn unofficial_isc(cpu : &mut CPU, address : address, value : byte)
{
    //Increase memory by one, then subtract memory from accumulator (with borrow)
    let new_value = value.wrapping_add(1);
    cpu.bus.put(address, new_value);
    sbc(cpu, address, new_value);
}

fn unofficial_hlt(_cpu : &mut CPU, _address : address, opcode_key: byte)
{
    //CPU halt
    //todo should panic?
    println!("CPU was halted! Opcode {:#04X}!", opcode_key);
}

fn unofficial_lar(cpu : &mut CPU, _address : address, value : byte)
{
    //AND memory with stack pointer, transfer result to accumulator, X register and stack pointer
    let new_value = value & CPUStack::get_pointer(cpu);
    cpu.A = new_value;
    cpu.X = new_value;
    CPUStack::set_pointer(cpu, new_value);
    cpu.flags.zero = new_value==0;
    cpu.flags.negative = isneg!(new_value);
}

fn unofficial_lax(cpu : &mut CPU, _address : address, value : byte)
{
    //Load accumulator and X register with memory
    cpu.A = value;
    cpu.X = value;
    cpu.flags.zero = value==0;
    cpu.flags.negative = isneg!(value);
}

fn unofficial_rla(cpu : &mut CPU, address : address, value : byte)
{
    //Rotate one bit left in memory, then AND accumulator with memory
    rol(cpu, address, value);
    let new_value = cpu.bus.get(address);
    let new_accumulator = cpu.A & new_value;
    cpu.A = new_accumulator;
    cpu.flags.zero = new_accumulator==0;
    cpu.flags.negative = isneg!(new_accumulator);
}

fn unofficial_rra(cpu : &mut CPU, address : address, value : byte)
{
    //Rotate one bit right in memory, then add memory to accumulator (with carry)
    ror(cpu, address, value);
    let new_value = cpu.bus.get(address);
    adc(cpu, 0, new_value);
}

fn unofficial_sbc(cpu : &mut CPU, address : address, value : byte)
{
    //subtract with carry
    sbc(cpu, address, value);
}

fn unofficial_slo(cpu : &mut CPU, address : address, value : byte)
{
    //shift left one bit in memory, then OR accumulator with memory
    asl(cpu, address, value);
    let new_value = cpu.bus.get(address);
    let new_accumulator = cpu.A | new_value;
    cpu.A = new_accumulator;
    cpu.flags.zero = new_accumulator==0;
    cpu.flags.negative = isneg!(new_accumulator);
}

fn unofficial_sre(cpu : &mut CPU, address : address, value : byte)
{
    //Shift right one bit in memory, then XOR accumulator with memory
    lsr(cpu, address, value);
    let new_value = cpu.bus.get(address);
    let new_accumulator = cpu.A ^ new_value;
    cpu.A = new_accumulator;
    cpu.flags.zero = new_accumulator==0;
    cpu.flags.negative = isneg!(new_accumulator);
}

fn unofficial_sxa(cpu : &mut CPU, address : address, _value : byte)
{
    //AND X register with the high byte of the target address of the argument + 1, store the result in memory
    let new_value = (cpu.X & ((address>>8) as byte)).wrapping_add(1);
    cpu.bus.put(address, new_value);
}

fn unofficial_sya(cpu : &mut CPU, address : address, _value : byte)
{
    //AND Y register with the high byte of the target address of the argument + 1, store the result in memory
    let new_value = (cpu.Y & ((address>>8) as byte)).wrapping_add(1);
    cpu.bus.put(address, new_value);
}

fn unofficial_xaa(cpu : &mut CPU, _address : address, value : byte)
{
    //set A = (A | magic constant) & X & value
    let new_accumulator = ((cpu.A | 0xEE) & cpu.X) & value;
    cpu.A = new_accumulator;
}

fn unofficial_xas(cpu : &mut CPU, address : address, _value : byte)
{
    //AND X register with accumulator and store result in stack pointer, then
    //AND stack pointer with the high byte of the target address of the argument + 1, store result in memory
    let new_stack_pointer = cpu.X & cpu.A;
    CPUStack::set_pointer(cpu, new_stack_pointer);
    let new_value = (CPUStack::get_pointer(cpu) & ((address>>8) as byte)).wrapping_add(1);
    cpu.bus.put(address, new_value);
}
