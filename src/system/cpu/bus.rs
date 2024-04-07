use crate::system::{address, address_from_high_low, byte};
use crate::system::cpu::CPUChannelsToOtherSystems;
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::cpu::ram::RAM;

const RAM_START_ADDRESS : address = 0x0000;
const RAM_END_ADDRESS : address = 0x1FFF;
const PPU_REGISTERS_START_ADDRESS : address = 0x2000;
const PPU_REGISTERS_END_ADDRESS : address = 0x3FFF;
const PPU_OAM_DMA_ADDRESS : address = 0x4014;
const JOYSTICK_ADDRESS : address = 0x4016;
const APU_OPEN_BUS_ON_READ_START_ADDRESS : address = 0x4000;
const APU_OPEN_BUS_ON_READ_END_ADDRESS : address = 0x4014;
const APU_REGISTERS_START_ADDRESS : address = 0x4000;
const APU_REGISTERS_END_ADDRESS : address = 0x4017;
const PROGRAM_ROM_START_ADDRESS : address = 0x4020;
const PROGRAM_ROM_END_ADDRESS : address = 0xFFFF;

pub struct CPUBus
{
    ram : RAM,
    pub program_rom : ProgramROM,
    pub channels : CPUChannelsToOtherSystems,
    last_read_byte : byte,
}

impl CPUBus
{
    pub fn new(program_rom : ProgramROM, channels : CPUChannelsToOtherSystems) -> CPUBus
    {
        return CPUBus
        {
            ram: RAM::new(),
            program_rom: program_rom,
            channels: channels,
            last_read_byte: 0,
        };
    }

    pub fn get(self : &mut CPUBus, raw_address : address) -> byte
    {
        let read_byte = if raw_address >= RAM_START_ADDRESS && raw_address <= RAM_END_ADDRESS
        {
            self.ram.get(raw_address)
        }
        else if (raw_address >= PPU_REGISTERS_START_ADDRESS && raw_address <= PPU_REGISTERS_END_ADDRESS) || raw_address == JOYSTICK_ADDRESS
        {
            let channel_address = self.convert_raw_address_to_ppu_channel_address(raw_address);
            self.channels.ppu_channels.read(channel_address)
        }
        else if raw_address >= APU_OPEN_BUS_ON_READ_START_ADDRESS && raw_address <= APU_OPEN_BUS_ON_READ_END_ADDRESS
        {
            self.last_read_byte
        }
        else if raw_address >= APU_REGISTERS_START_ADDRESS && raw_address <= APU_REGISTERS_END_ADDRESS
        {
            self.channels.apu_channels.read(raw_address)
        }
        else if raw_address >= PROGRAM_ROM_START_ADDRESS && raw_address <= PROGRAM_ROM_END_ADDRESS
        {
            self.program_rom.get(raw_address)
        }
        else {0};

        self.last_read_byte = read_byte;
        return read_byte;
    }

    pub fn put(self : &mut CPUBus, raw_address : address, value : byte)
    {
        if raw_address >= RAM_START_ADDRESS && raw_address <= RAM_END_ADDRESS
        {
            self.ram.put(raw_address, value);
        }
        else if raw_address == PPU_OAM_DMA_ADDRESS
        {
            let start_address = address_from_high_low(value, 0x00);
            let end_address = address_from_high_low(value, 0xFF);
            let values_to_copy = (start_address..=end_address)
                .map(|address| self.get(address)).collect::<Vec<byte>>().into_boxed_slice();
            let channel_address = self.convert_raw_address_to_ppu_channel_address(raw_address);
            self.channels.ppu_channels.write(channel_address, values_to_copy);
        }
        else if (raw_address >= PPU_REGISTERS_START_ADDRESS && raw_address <= PPU_REGISTERS_END_ADDRESS) || raw_address == JOYSTICK_ADDRESS
        {
            let channel_address = self.convert_raw_address_to_ppu_channel_address(raw_address);
            self.channels.ppu_channels.write(channel_address, Box::new([value]));
        }
        else if raw_address >= APU_REGISTERS_START_ADDRESS && raw_address <= APU_REGISTERS_END_ADDRESS
        {
            self.channels.apu_channels.write(raw_address, value);
        }
        else if raw_address >= PROGRAM_ROM_START_ADDRESS && raw_address < PROGRAM_ROM_END_ADDRESS
        {
            self.program_rom.set(raw_address, value)
        }
    }

    fn convert_raw_address_to_ppu_channel_address(&self, raw_address : address) -> address
    {
        return if raw_address == PPU_OAM_DMA_ADDRESS || raw_address == JOYSTICK_ADDRESS { raw_address }
        else { ((raw_address-PPU_REGISTERS_START_ADDRESS) % 8) + PPU_REGISTERS_START_ADDRESS };
    }
}
