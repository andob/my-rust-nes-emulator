use crate::system::{address, byte};
use crate::system::channels::CPUToPPUChannels;
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::ram::RAM;

const RAM_START_ADDRESS : address = 0x0000;
const RAM_END_ADDRESS : address = 0x1FFF;
const PPU_REGISTERS_START_ADDRESS : address = 0x2000;
const PPU_REGISTERS_END_ADDRESS : address = 0x3FFF;
const PPU_OAM_DMA_ADDRESS : address = 0x4014;
const IO_REGISTERS_START_ADDRESS : address = 0x4000;
const IO_REGISTERS_END_ADDRESS : address = 0x4017;
const PROGRAM_ROM_START_ADDRESS : address = 0x4020;
const PROGRAM_ROM_END_ADDRESS : address = 0xFFFF;

pub struct CPUBus
{
    pub ram : RAM,
    pub program_rom : ProgramROM,
    pub ppu_channels : CPUToPPUChannels,
}

impl CPUBus
{
    pub fn new(program_rom : ProgramROM, channels : CPUToPPUChannels) -> CPUBus
    {
        return CPUBus
        {
            ram: RAM::new(),
            program_rom: program_rom,
            ppu_channels: channels,
        };
    }

    pub fn get(self : &CPUBus, raw_address : address) -> byte
    {
        if raw_address >= RAM_START_ADDRESS && raw_address <= RAM_END_ADDRESS
        {
            return self.ram.get(raw_address);
        }

        if (raw_address >= PPU_REGISTERS_START_ADDRESS && raw_address <= PPU_REGISTERS_END_ADDRESS) || raw_address == PPU_OAM_DMA_ADDRESS
        {
            let channel_address = self.convert_raw_address_to_ppu_channel_address(raw_address);
            return self.ppu_channels.read(channel_address);
        }

        if raw_address >= IO_REGISTERS_START_ADDRESS && raw_address <= IO_REGISTERS_END_ADDRESS
        {
            //todo NES APU and IO registers
            return 0;
        }

        if raw_address >= PROGRAM_ROM_START_ADDRESS && raw_address < PROGRAM_ROM_END_ADDRESS
        {
            return self.program_rom.get(raw_address-PROGRAM_ROM_START_ADDRESS);
        }

        return 0;
    }

    pub fn put(self : &mut CPUBus, raw_address : address, value : byte)
    {
        if raw_address >= RAM_START_ADDRESS && raw_address <= RAM_END_ADDRESS
        {
            self.ram.put(raw_address, value);
        }

        if (raw_address >= PPU_REGISTERS_START_ADDRESS && raw_address <= PPU_REGISTERS_END_ADDRESS) || raw_address == PPU_OAM_DMA_ADDRESS
        {
            let channel_address = self.convert_raw_address_to_ppu_channel_address(raw_address);
            self.ppu_channels.write(channel_address, value);
        }

        if raw_address >= IO_REGISTERS_START_ADDRESS && raw_address <= IO_REGISTERS_END_ADDRESS
        {
            //todo NES APU and IO registers
        }
    }

    fn convert_raw_address_to_ppu_channel_address(&self, raw_address : address) -> address
    {
        return if raw_address == PPU_OAM_DMA_ADDRESS { PPU_OAM_DMA_ADDRESS }
        else { ((raw_address-PPU_REGISTERS_START_ADDRESS) % 8) + PPU_REGISTERS_START_ADDRESS };
    }
}
