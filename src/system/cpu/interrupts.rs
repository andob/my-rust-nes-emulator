use crate::address_from_high_low;
use crate::system::address;
use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    pub fn software_irq(cpu : &mut CPU)
    {
        if !cpu.flags._break
        {
            cpu.flags._break = true;

            let vector = cpu.bus.program_rom.len()-2;
            CPUInterrupts::interrupt(cpu, vector as address);
        }
    }

    pub fn hardware_irq(cpu : &mut CPU)
    {
        //IRQ = interrupt request
        let vector = cpu.bus.program_rom.len()-2;
        CPUInterrupts::interrupt(cpu, vector as address);
    }

    pub fn hardware_reset(cpu : &mut CPU)
    {
        cpu.flags.interrupt = true;

        let vector = cpu.bus.program_rom.len()-4;
        CPUInterrupts::interrupt(cpu, vector as address);
    }

    pub fn hardware_nmi(cpu : &mut CPU)
    {
        //NMI = non-maskable interrupt
        let vector = cpu.bus.program_rom.len()-6;
        CPUInterrupts::interrupt(cpu, vector as address);
    }

    fn interrupt(cpu : &mut CPU, vector : address)
    {
        let cpu_flags_to_backup = cpu.flags.to_byte();

        CPUStack::push_address(cpu, cpu.program_counter);
        CPUStack::push_byte(cpu, cpu_flags_to_backup);

        if !cpu.are_interrupt_vectors_disabled
        {
            let low = cpu.bus.program_rom.get(vector);
            let high = cpu.bus.program_rom.get(vector+1);
            let interrupt_handler_address = address_from_high_low!(high, low);
            cpu.program_counter = interrupt_handler_address;
        }
    }
}
