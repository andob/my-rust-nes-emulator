use crate::address_from_high_low;
use crate::system::address;
use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    pub fn software_irq(cpu : &mut CPU)
    {
        //IRQ = request interrupt
        if !cpu.flags._break
        {
            cpu.flags._break = true;
            CPUInterrupts::interrupt(cpu, 0xFFFE);
        }
    }

    pub fn hardware_irq(cpu : &mut CPU)
    {
        //IRQ = request interrupt
        CPUInterrupts::interrupt(cpu, 0xFFFE);
    }

    pub fn hardware_reset(cpu : &mut CPU)
    {
        if !cpu.flags.interrupt
        {
            cpu.flags.interrupt = true;
            CPUInterrupts::interrupt(cpu, 0xFFFC);
        }
    }

    pub fn hardware_nmi(cpu : &mut CPU)
    {
        //NMI = non-maskable interrupt
        CPUInterrupts::interrupt(cpu, 0xFFFA);
    }

    fn interrupt(cpu : &mut CPU, vector : address)
    {
        let cpu_flags_to_backup = cpu.flags.to_byte();

        CPUStack::push_address(cpu, cpu.program_counter);
        CPUStack::push_byte(cpu, cpu_flags_to_backup);

        let low = cpu.bus.get(vector);
        let high = cpu.bus.get(vector.wrapping_add(1));
        let interrupt_handler_address = address_from_high_low!(high, low);
        cpu.program_counter = interrupt_handler_address;
    }
}
