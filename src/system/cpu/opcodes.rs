use crate::system::cpu::flags::CPUFlags;
use crate::system::cpu::program_iterator::AddressingMode;
use crate::system::cpu::stack::CPUStack;
use crate::system::{address, byte, System};
use crate::system::cpu::clock::ExpectedDuration;

pub struct Opcode
{
    pub key : byte,
    pub name : String,
    pub addressing_mode : AddressingMode,
    pub lambda : fn(&mut System, address, byte) -> (),
    pub expected_duration : ExpectedDuration,
}

macro_rules! opcode
{
    ($key : expr, $name : expr, $expected_duration : expr, $addressing_mode : expr) =>
    {{
        Opcode
        {
            key: $key, lambda: $name,
            name: stringify!($name).to_uppercase(),
            expected_duration: $expected_duration,
            addressing_mode: $addressing_mode,
        }
    }}
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

macro_rules! is_negative
{
    ($arg : expr) => {{ (($arg)>>7==1) }}
}

fn adc(nes : &mut System, _address : address, value : byte)
{
    //add with carry
    if nes.cpu.flags.decimal
    {
        //todo implement add with decimal mode
    }
    else
    {
        //ADC - Add Memory to Accumulator with Carry
        let new_value_16b = (nes.cpu.A as i16) + (value as i16) + (nes.cpu.flags.carry as i16);
        let new_value_8b = (new_value_16b & 0xFF) as u8;
        let old_accumulator = nes.cpu.A;
        nes.cpu.A = new_value_8b;
        nes.cpu.flags.carry = new_value_16b > 0xFF;
        nes.cpu.flags.overflow = is_negative!(old_accumulator) ^ is_negative!(new_value_8b);
        nes.cpu.flags.negative = is_negative!(new_value_8b);
        nes.cpu.flags.zero = new_value_8b==0;
    }
}

fn and(nes : &mut System, _address : address, value : byte)
{
    //logical AND value with Accumulator
    let new_value = nes.cpu.A & value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn asl(nes : &mut System, address : address, value : byte)
{
    //arithmetic shift left
    nes.cpu.flags.carry = (value & 0b10000000) >> 7 == 1;
    let new_value = value << 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn bit(nes : &mut System, _address : address, value : byte)
{
    //test bits
    nes.cpu.flags.negative = (value & 0b10000000) >> 7 == 1;
    nes.cpu.flags.overflow = (value & 0b01000000) >> 6 == 1;
    nes.cpu.flags.zero = (value & nes.cpu.A) == 0;
}

fn branch(nes : &mut System, signed_offset : byte)
{
    let abs_offset = (signed_offset as i8).abs() as address;
    if is_negative!(signed_offset)
    {
        nes.cpu.program_counter = nes.cpu.program_counter.wrapping_sub(abs_offset);
        nes.cpu.clock.notify_branch_taken();
    }
    else
    {
        nes.cpu.program_counter = nes.cpu.program_counter.wrapping_add(abs_offset);
        nes.cpu.clock.notify_branch_taken();
    }
}

fn bpl(nes : &mut System, _address : address, offset : byte)
{
    //branch on result plus
    if !nes.cpu.flags.negative
    {
        branch(nes, offset);
    }
}

fn bmi(nes : &mut System, _address : address, offset : byte)
{
    //branch on result minus
    if nes.cpu.flags.negative
    {
        branch(nes, offset);
    }
}

fn bvc(nes : &mut System, _address : address, offset : byte)
{
    //branch on overflow clear
    if !nes.cpu.flags.overflow
    {
        branch(nes, offset);
    }
}

fn bvs(nes : &mut System, _address : address, offset : byte)
{
    //branch on overflow set
    if nes.cpu.flags.overflow
    {
        branch(nes, offset);
    }
}

fn bcc(nes : &mut System, _address : address, offset : byte)
{
    //branch on carry clear
    if !nes.cpu.flags.carry
    {
        branch(nes, offset);
    }
}

fn bcs(nes : &mut System, _address : address, offset : byte)
{
    //branch on carry set
    if nes.cpu.flags.carry
    {
        branch(nes, offset);
    }
}

fn bne(nes : &mut System, _address : address, offset : byte)
{
    //branch on result not zero
    if !nes.cpu.flags.zero
    {
        branch(nes, offset);
    }
}

fn beq(nes : &mut System, _address : address, offset : byte)
{
    //branch on result zero
    if nes.cpu.flags.zero
    {
        branch(nes, offset);
    }
}

fn brk(_nes : &mut System, _address : address, _value : byte)
{
    //break!
    //todo implement interrupts
}

fn cmp(nes : &mut System, _address : address, value : byte)
{
    //substract Accumulator minus memory, set flags
    let (left, right) = (nes.cpu.A as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = new_value<0;
    nes.cpu.flags.carry = value<=nes.cpu.A;
}

fn cpx(nes : &mut System, _address : address, value : byte)
{
    //substract X minus memory, set flags
    let (left, right) = (nes.cpu.X as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = new_value<0;
    nes.cpu.flags.carry = value<=nes.cpu.X;
}

fn cpy(nes : &mut System, _address : address, value : byte)
{
    //substract Accumulator minus memory, set flags
    let (left, right) = (nes.cpu.Y as i8, value as i8);
    let new_value = left.wrapping_sub(right);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = new_value<0;
    nes.cpu.flags.carry = value<=nes.cpu.Y;
}

fn dec(nes : &mut System, address : address, value : byte)
{
    //decrement memory
    let new_value = value.wrapping_sub(1);
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn eor(nes : &mut System, _address : address, value : byte)
{
    //logical XOR value with Accumulator
    let new_value = nes.cpu.A ^ value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn clc(nes : &mut System, _address : address, _value : byte)
{
    //clear carry flag
    nes.cpu.flags.carry = false;
}

fn sec(nes : &mut System, _address : address, _value : byte)
{
    //set carry flag
    nes.cpu.flags.carry = true;
}

fn cli(nes : &mut System, _address : address, _value : byte)
{
    //clear interrupt disable flag
    nes.cpu.flags.interrupt = false;
}

fn sei(nes : &mut System, _address : address, _value : byte)
{
    //set interrupt disable flag
    nes.cpu.flags.interrupt = true;
}

fn clv(nes : &mut System, _address : address, _value : byte)
{
    //clear overflow flag
    nes.cpu.flags.overflow = false;
}

fn cld(nes : &mut System, _address : address, _value : byte)
{
    //clear decimal mode flag
    nes.cpu.flags.decimal = false;
}

fn sed(nes : &mut System, _address : address, _value : byte)
{
    //set decimal mode flag
    nes.cpu.flags.decimal = true;
}

fn inc(nes : &mut System, address : address, value : byte)
{
    //increment memory
    let new_value = value.wrapping_add(1);
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn jmp(nes : &mut System, address : address, _value : byte)
{
    //goto / jump
    nes.cpu.program_counter = address;
}

fn jsr(nes : &mut System, address : address, _value : byte)
{
    //jump to subroutine
    nes.cpu.program_counter -= 2;
    CPUStack::push(nes, ((nes.cpu.program_counter>>8)&0xFF) as byte);
    CPUStack::push(nes, (nes.cpu.program_counter&0xFF) as byte);
    nes.cpu.program_counter = address;
}

fn lda(nes : &mut System, _address : address, value : byte)
{
    //load memory into Accumulator
    nes.cpu.A = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldx(nes : &mut System, _address : address, value : byte)
{
    //load memory into register X
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn ldy(nes : &mut System, _address : address, value: byte)
{
    //load memory into register Y
    nes.cpu.Y = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn lsr(nes : &mut System, address : address, value : byte)
{
    //logical shift right
    nes.cpu.flags.carry = value & 0b00000001 == 1;
    let new_value = value >> 1;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    if address > 0 { nes.ram.put(address, new_value); }
    else { nes.cpu.A = new_value; }
}

fn nop(_nes : &mut System, _address : address, _value : byte)
{
    //no operation!
}

fn ora(nes : &mut System, _address : address, value : byte)
{
    //logical OR value with Accumulator
    let new_value = nes.cpu.A | value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tax(nes : &mut System, _address : address, _value : byte)
{
    //transfer Accumulator into register X
    let new_value = nes.cpu.A;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn txa(nes : &mut System, _address : address, _value : byte)
{
    //transfer register X into Accumulator
    let new_value = nes.cpu.X;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dex(nes : &mut System, _address : address, _value : byte)
{
    //decrement register X
    let new_value = nes.cpu.X.wrapping_sub(1);
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn inx(nes : &mut System, _address : address, _value : byte)
{
    //increment register X
    let new_value = nes.cpu.X.wrapping_add(1);
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tay(nes : &mut System, _address : address, _value : byte)
{
    //transfer Accumulator into register Y
    let new_value = nes.cpu.A;
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn tya(nes : &mut System, _address : address, _value : byte)
{
    //transfer register Y into Accumulator
    let new_value = nes.cpu.Y;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn dey(nes : &mut System, _address : address, _value : byte)
{
    //decrement register Y
    let new_value = nes.cpu.Y.wrapping_sub(1);
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn iny(nes : &mut System, _address : address, _value : byte)
{
    //increment register Y
    let new_value = nes.cpu.Y.wrapping_add(1);
    nes.cpu.Y = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn rol(nes : &mut System, address : address, value : byte)
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

fn ror(nes : &mut System, address : address, value : byte)
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

fn rti(_nes : &mut System, _address : address, _value : byte)
{
    //return from interrupt
    //todo implement interrupts
}

fn rts(nes : &mut System, _address : address, _value : byte)
{
    //return from subroutine
    if let Some(low) = CPUStack::pop(nes)
    {
        if let Some(high) = CPUStack::pop(nes)
        {
            let new_address = 2+(((high as address)<<8) | (low as address));
            nes.cpu.program_counter = new_address;
        }
    }
}

fn sbc(nes : &mut System, _address : address, value : byte)
{
    //substract with carry
    if nes.cpu.flags.decimal
    {
        //todo implement substract with decimal mode
    }
    else
    {
        let new_value_16b = (nes.cpu.A as i16) - (value as i16) - ((!nes.cpu.flags.carry) as i16);
        let sign_flag = if new_value_16b<0 { -1 } else { 1 };
        let new_value_8b = ((new_value_16b.abs()&0xFF) as i8) * sign_flag;
        let old_accumulator = nes.cpu.A;
        nes.cpu.A = new_value_8b as u8;
        nes.cpu.flags.carry = old_accumulator>=value;
        nes.cpu.flags.overflow = (new_value_8b as u8)>0x7F;
        nes.cpu.flags.negative = is_negative!(new_value_8b as u8);
        nes.cpu.flags.zero = new_value_8b==0;
    }
}

fn sta(nes : &mut System, address : address, _value : byte)
{
    //store Accumulator into memory
    nes.ram.put(address, nes.cpu.A);
}

fn txs(nes : &mut System, _address : address, _value : byte)
{
    //transfer register X into Stack Pointer
    nes.cpu.stack.set_pointer(nes.cpu.X);
}

fn tsx(nes : &mut System, _address : address, _value : byte)
{
    //transfer Stack Pointer into register X
    let new_value = nes.cpu.stack.get_pointer();
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn pha(nes : &mut System, _address : address, _value : byte)
{
    //push Accumulator on Stack
    CPUStack::push(nes, nes.cpu.A);
}

fn pla(nes : &mut System, _address : address, _value : byte)
{
    //pop Stack into Accumulator
    if let Some(new_value) = CPUStack::pop(nes)
    {
        nes.cpu.A = new_value;
        nes.cpu.flags.zero = new_value==0;
        nes.cpu.flags.negative = is_negative!(new_value);
    }
}

fn php(nes : &mut System, _address : address, _value : byte)
{
    //push Processor Flags on Stack
    let byte = nes.cpu.flags.to_byte();
    CPUStack::push(nes, byte);
}

fn plp(nes : &mut System, _address : address, _value : byte)
{
    //pull Stack into Processor Flags
    if let Some(byte) = CPUStack::pop(nes)
    {
        nes.cpu.flags = CPUFlags::from_byte(byte);
    }
}

fn stx(nes : &mut System, address : address, _value : byte)
{
    //store register X into memory
    nes.ram.put(address, nes.cpu.A);
}

fn sty(nes : &mut System, address : address, _value : byte)
{
    //store register Y into memory
    nes.ram.put(address, nes.cpu.A);
}

fn unofficial_aac(nes : &mut System, _address : address, value : byte)
{
    //AND byte with accumulator. If result is negative then carry is set
    let new_value = nes.cpu.A & value;
    nes.cpu.A = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    nes.cpu.flags.carry = is_negative!(new_value);
}

fn unofficial_aax(nes : &mut System, address : address, _value : byte)
{
    //AND X register with accumulator and store result in memory
    let new_value = nes.cpu.X & nes.cpu.A;
    nes.ram.put(address, new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn unofficial_aar(nes : &mut System, _address : address, value : byte)
{
    //AND byte with accumulator, then rotate one bit right in accumulator and check bit 5 and 6
    nes.cpu.A = nes.cpu.A & value;
    ror(nes, 0, nes.cpu.A);
    let bit5 = ((nes.cpu.A & 0b00100000) >> 5)==1;
    let bit6 = ((nes.cpu.A & 0b01000000) >> 6)==1;
    if bit5 && bit6 { nes.cpu.flags.carry = true; nes.cpu.flags.overflow = false; }
    else if !bit5 && !bit6 { nes.cpu.flags.carry = false; nes.cpu.flags.overflow = false; }
    else if bit5 { nes.cpu.flags.carry = false; nes.cpu.flags.overflow = true; }
    else { nes.cpu.flags.carry = true; nes.cpu.flags.overflow = true; }
}

fn unofficial_asr(nes : &mut System, _address : address, value : byte)
{
    //AND byte with accumulator, then shift right one bit in accumulator
    nes.cpu.A = nes.cpu.A & value;
    lsr(nes, 0, nes.cpu.A);
}

fn unofficial_atx(nes : &mut System, _address : address, value : byte)
{
    //AND byte with (accumulator or by magic constant), then transfer accumulator to X register
    let new_value = (nes.cpu.A | 0xEE) & value;
    nes.cpu.A = new_value;
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn unofficial_axa(nes : &mut System, address : address, _value : byte)
{
    //AND X register with accumulator then AND result with 7 and store in memory
    nes.ram.put(address, (nes.cpu.X & nes.cpu.A) & 7);
}

fn unofficial_axs(nes : &mut System, _address : address, value : byte)
{
    //AND X register with accumulator and store result in X register, then subtract byte from X register (without borrow)
    let new_value = (nes.cpu.X & nes.cpu.A).wrapping_sub(value);
    nes.cpu.X = new_value;
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
    nes.cpu.flags.carry = value<=new_value;
}

fn unofficial_dcp(nes : &mut System, address : address, value : byte)
{
    //Subtract 1 from memory (without borrow)
    let new_value = value.wrapping_sub(1);
    nes.ram.put(address, new_value);
    nes.cpu.flags.carry = value<=new_value;
}

fn unofficial_nop(_nes : &mut System, _address : address, _value : byte)
{
    //no operation
}

fn unofficial_isc(nes : &mut System, address : address, value : byte)
{
    //Increase memory by one, then subtract memory from accumulator (with borrow)
    let new_value = value.wrapping_add(1);
    nes.ram.put(address, new_value);
    sbc(nes, address, new_value);
}

fn unofficial_hlt(_nes : &mut System, _address : address, opcode_key: byte)
{
    //CPU halt
    panic!("CPU was halted! Opcode {:#04X}!", opcode_key);
}

fn unofficial_lar(nes : &mut System, _address : address, value : byte)
{
    //AND memory with stack pointer, transfer result to accumulator, X register and stack pointer
    let new_value = value & nes.cpu.stack.get_pointer();
    nes.cpu.A = new_value;
    nes.cpu.X = new_value;
    nes.cpu.stack.set_pointer(new_value);
    nes.cpu.flags.zero = new_value==0;
    nes.cpu.flags.negative = is_negative!(new_value);
}

fn unofficial_lax(nes : &mut System, _address : address, value : byte)
{
    //Load accumulator and X register with memory
    nes.cpu.A = value;
    nes.cpu.X = value;
    nes.cpu.flags.zero = value==0;
    nes.cpu.flags.negative = is_negative!(value);
}

fn unofficial_rla(nes : &mut System, address : address, value : byte)
{
    //Rotate one bit left in memory, then AND accumulator with memory
    rol(nes, address, value);
    let new_value = nes.ram.get(address);
    let new_accumulator = nes.cpu.A & new_value;
    nes.cpu.flags.zero = new_accumulator==0;
    nes.cpu.flags.negative = is_negative!(new_accumulator);
}

fn unofficial_rra(nes : &mut System, address : address, value : byte)
{
    //Rotate one bit right in memory, then add memory to accumulator (with carry)
    ror(nes, address, value);
    let new_value = nes.ram.get(address);
    adc(nes, 0, new_value);
}

fn unofficial_sbc(nes : &mut System, address : address, value : byte)
{
    //subtract with carry
    sbc(nes, address, value);
}

fn unofficial_slo(nes : &mut System, address : address, value : byte)
{
    //shift left one bit in memory, then OR accumulator with memory
    asl(nes, address, value);
    let new_value = nes.ram.get(address);
    let new_accumulator = nes.cpu.A | new_value;
    nes.cpu.A = new_accumulator;
    nes.cpu.flags.zero = new_accumulator==0;
    nes.cpu.flags.negative = is_negative!(new_accumulator);
}

fn unofficial_sre(nes : &mut System, address : address, value : byte)
{
    //Shift right one bit in memory, then XOR accumulator with memory
    lsr(nes, address, value);
    let new_value = nes.ram.get(address);
    let new_accumulator = nes.cpu.A ^ new_value;
    nes.cpu.A = new_accumulator;
    nes.cpu.flags.zero = new_accumulator==0;
    nes.cpu.flags.negative = is_negative!(new_accumulator);
}

fn unofficial_sxa(nes : &mut System, address : address, _value : byte)
{
    //AND X register with the high byte of the target address of the argument + 1, store the result in memory
    let new_value = (nes.cpu.X & ((address >> 8) as byte)).wrapping_add(1);
    nes.ram.put(address, new_value);
}

fn unofficial_sya(nes : &mut System, address : address, _value : byte)
{
    //AND Y register with the high byte of the target address of the argument + 1, store the result in memory
    let new_value = (nes.cpu.Y & ((address>>8) as byte)).wrapping_add(1);
    nes.ram.put(address, new_value);
}

fn unofficial_xaa(nes : &mut System, _address : address, value : byte)
{
    //set A = (A | magic constant) & X & value
    let new_accumulator = ((nes.cpu.A | 0xEE) & nes.cpu.X) & value;
    nes.cpu.A = new_accumulator;
}

fn unofficial_xas(nes : &mut System, address : address, _value : byte)
{
    //AND X register with accumulator and store result in stack pointer, then
    //AND stack pointer with the high byte of the target address of the argument + 1, store result in memory
    let new_stack_pointer = nes.cpu.X & nes.cpu.A;
    nes.cpu.stack.set_pointer(new_stack_pointer);
    let new_value = (nes.cpu.stack.get_pointer() & ((address>>8) as byte)).wrapping_add(1);
    nes.ram.put(address, new_value);
}
