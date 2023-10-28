use crate::address_from_high_low;
use crate::system::{address, System};
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}

impl CPUInterrupts
{
    pub fn irq(nes : &mut System)
    {
        //IRQ = request interrupt
        if !nes.cpu.flags._break
        {
            nes.cpu.flags._break = true;
            CPUInterrupts::interrupt(nes, 0xFFFE);
        }
    }

    pub fn reset(nes : &mut System)
    {
        CPUInterrupts::interrupt(nes, 0xFFFC);
    }

    pub fn _nmi(nes : &mut System)
    {
        //todo call this on PPU v-blank
        //NMI = non-maskable interrupt
        CPUInterrupts::interrupt(nes, 0xFFFA);
    }

    fn interrupt(nes : &mut System, vector : address)
    {
        if !nes.cpu.flags.interrupt
        {
            let cpu_flags_to_backup = nes.cpu.flags.to_byte();
            nes.cpu.flags.interrupt = true;

            CPUStack::push_address(nes, nes.cpu.program_counter);
            CPUStack::push_byte(nes, cpu_flags_to_backup);

            if (vector as usize) < nes.cpu_bus.program_rom.len()
            {
                let low = nes.cpu_bus.program_rom.get(vector);
                let high = nes.cpu_bus.program_rom.get(vector.wrapping_add(1));
                let interrupt_handler_address = address_from_high_low!(high, low);
                nes.cpu.program_counter = interrupt_handler_address;
            }
        }
    }
}
