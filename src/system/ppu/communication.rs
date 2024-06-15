use crate::system::{address, address_from_high_low, byte};
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
                if ppu.is_second_scroll_write { ppu.scroll_y = 0f32; }
                // else { ppu.scroll_x = (((values[0] as i8) & 0b01111111) as f32) / 20f32; }
                else { ppu.scroll_x += if values[0]>0 { 0.01f32 } else { 0f32 }; }

                ppu.is_second_scroll_write = !ppu.is_second_scroll_write;
            }
            Ok((CPUToPPUCommTarget::BusAddress, values)) =>
            {
                if ppu.is_second_bus_pointer_write
                {
                    ppu.bus_pointer = address_from_high_low(ppu.first_bus_pointer_write, values[0]);
                    ppu.is_second_bus_pointer_write = false;
                }
                else
                {
                    ppu.first_bus_pointer_write = values[0];
                    ppu.is_second_bus_pointer_write = true;
                }
            }
            Ok((CPUToPPUCommTarget::BusData, values)) =>
            {
                ppu.bus.put(ppu.bus_pointer, values[0]);
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
