use crate::address_from_high_low;
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
            //todo check implementation https://www.nesdev.org/wiki/PPU_registers
            ppu.cpu_channels.respond_to_read_command_from_cpu(target, match target
            {
                CPUToPPUCommTarget::ControlFlags => ppu.control_flags.to_byte(),
                CPUToPPUCommTarget::MaskFlags => ppu.mask_flags.to_byte(),
                CPUToPPUCommTarget::StatusFlags => ppu.status_flags.to_byte(),
                CPUToPPUCommTarget::OAMAddress => ppu.oam_pointer as byte,
                CPUToPPUCommTarget::OAMData => ppu.oam.get(ppu.oam_pointer),
                CPUToPPUCommTarget::BusAddress => ppu.bus_pointer as byte,
                CPUToPPUCommTarget::BusData => ppu.bus.get(ppu.bus_pointer),
                CPUToPPUCommTarget::Joystick => ppu.joystick.read_pressed_key(),
                _ => 0,
            });
        }
    }

    pub fn handle_write_commands_from_cpu(&mut self)
    {
        let ppu = self;
        match ppu.cpu_channels.get_write_command_from_cpu()
        {
            //todo check implementation https://www.nesdev.org/wiki/PPU_registers
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
                if ppu.is_second_scroll_write { ppu.scroll_y = values[0]; }
                else { ppu.scroll_x = values[0]; }
                ppu.is_second_scroll_write = !ppu.is_second_scroll_write;
            }
            Ok((CPUToPPUCommTarget::BusAddress, values)) =>
            {
                let low = values[0] as address;
                let high = if ppu.is_second_bus_pointer_write { ppu.bus_pointer } else { 0 };
                ppu.bus_pointer = address_from_high_low!(high, low);
                ppu.is_second_bus_pointer_write = !ppu.is_second_bus_pointer_write;
            }
            Ok((CPUToPPUCommTarget::BusData, values)) =>
            {
                ppu.bus.put(ppu.bus_pointer, values[0]);
                ppu.bus_pointer = ppu.bus_pointer.wrapping_add(1);
            }
            Ok((CPUToPPUCommTarget::OAM_DMA, values)) =>
            {
                ppu.oam.put_many(ppu.oam_pointer, &values);
                ppu.oam_pointer = ppu.oam_pointer.wrapping_add(values.len() as address);
            }
            Ok((CPUToPPUCommTarget::Joystick, values)) =>
            {
                ppu.joystick.set_strobe_enabled(values[0]&1==1);
            }
            _ => {}
        }
    }
}
