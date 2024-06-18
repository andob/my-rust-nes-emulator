use crate::system::{address, byte};
use crate::system::ppu::flags::control_flags::PPUControlFlags;
use crate::system::ppu::flags::mask_flags::PPUMaskFlags;
use crate::system::ppu::PPU;
use crate::system::ppu_channels::CPUToPPUCommTarget;

impl PPU
{
    pub fn handle_read_commands_from_cpu(&mut self)
    {
        let ppu = self;
        if let Ok(target) = ppu.cpu_channels.get_read_command_from_cpu()
        {
            ppu.cpu_channels.respond_to_read_command_from_cpu(target, match target
            {
                CPUToPPUCommTarget::ControlFlags => ppu.control_flags.to_byte(),
                CPUToPPUCommTarget::MaskFlags => ppu.mask_flags.to_byte(),
                CPUToPPUCommTarget::StatusFlags => ppu.status_flags.to_byte(),
                CPUToPPUCommTarget::OAMAddress => ppu.oam_pointer as byte,
                CPUToPPUCommTarget::OAMData => ppu.oam.get(ppu.oam_pointer),
                CPUToPPUCommTarget::BusAddress => ppu.bus_pointer.as_byte(),
                CPUToPPUCommTarget::BusData => ppu.bus.get(ppu.bus_pointer.as_address()),
                CPUToPPUCommTarget::Joystick => ppu.input_subsystem.get_pressed_key(),
                _ => 0,
            });
        }
    }

    pub fn handle_write_commands_from_cpu(&mut self)
    {
        let ppu = self;
        match ppu.cpu_channels.get_write_command_from_cpu()
        {
            Ok((CPUToPPUCommTarget::ControlFlags, values)) =>
            {
                ppu.control_flags = PPUControlFlags::from_byte(values[0]);
            }
            Ok((CPUToPPUCommTarget::MaskFlags, values)) =>
            {
                ppu.mask_flags = PPUMaskFlags::from_byte(values[0]);
            }
            Ok((CPUToPPUCommTarget::OAMAddress, values)) =>
            {
                ppu.oam_pointer = values[0] as address;
            }
            Ok((CPUToPPUCommTarget::OAMData, values)) =>
            {
                ppu.oam.put(ppu.oam_pointer, values[0]);
                ppu.oam_pointer = ppu.oam_pointer.wrapping_add(1);
            }
            Ok((CPUToPPUCommTarget::ScrollPosition, values)) =>
            {
                ppu.scroll.write(values[0] as f32);
            }
            Ok((CPUToPPUCommTarget::BusAddress, values)) =>
            {
                ppu.bus_pointer.write(values[0]);
            }
            Ok((CPUToPPUCommTarget::BusData, values)) =>
            {
                ppu.bus.put(ppu.bus_pointer.as_address(), values[0]);
                ppu.bus_pointer = ppu.bus_pointer.wrapping_add(ppu.control_flags.vram_address_increment_amount as address);
            }
            Ok((CPUToPPUCommTarget::OAM_DMA, values)) =>
            {
                ppu.oam.put_many(ppu.oam_pointer, &values);
                ppu.oam_pointer = ppu.oam_pointer.wrapping_add(values.len() as address);
            }
            Ok((CPUToPPUCommTarget::Joystick, values)) =>
            {
                ppu.input_subsystem.set_strobe_enabled(values[0] & 0b00000001 == 1);
            }
            _ => {}
        }
    }
}
