use crate::address_from_high_low;
use crate::system::address;
use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    pub fn irq(cpu : &mut CPU)
    {
        //IRQ = request interrupt
        if !cpu.flags._break && !cpu.flags.interrupt
        {
            cpu.flags._break = true;
            cpu.flags.interrupt = true;
            CPUInterrupts::interrupt(cpu, 0xFFFE);
        }
    }

    pub fn reset(cpu : &mut CPU)
    {
        if !cpu.flags.interrupt
        {
            cpu.flags.interrupt = true;
            CPUInterrupts::interrupt(cpu, 0xFFFC);
        }
    }

    pub fn nmi(cpu : &mut CPU)
    {
        //NMI = non-maskable interrupt
        CPUInterrupts::interrupt(cpu, 0xFFFA);
    }

    fn interrupt(cpu : &mut CPU, vector : address)
    {
        let cpu_flags_to_backup = cpu.flags.to_byte();

        CPUStack::push_address(cpu, cpu.program_counter);
        CPUStack::push_byte(cpu, cpu_flags_to_backup);

        if (vector as usize) < cpu.bus.program_rom.len()
        {
            let low = cpu.bus.program_rom.get(vector);
            let high = cpu.bus.program_rom.get(vector.wrapping_add(1));
            let interrupt_handler_address = address_from_high_low!(high, low);
            cpu.program_counter = interrupt_handler_address;
        }
    }
}
