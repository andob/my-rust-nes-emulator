use crate::address_from_high_low;
use crate::system::address;
use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

#[derive(PartialEq, Eq)]
enum CPUInterruptType { IRQ, NMI, RESET }

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    pub fn software_irq(cpu : &mut CPU)
    {
        // IRQ = interrupt request
        CPUInterrupts::interrupt(cpu, CPUInterruptType::IRQ);
    }

    pub fn hardware_irq(cpu : &mut CPU)
    {
        // IRQ = interrupt request
        CPUInterrupts::interrupt(cpu, CPUInterruptType::IRQ);
    }

    pub fn hardware_nmi(cpu : &mut CPU)
    {
        // NMI = non-maskable interrupt
        CPUInterrupts::interrupt(cpu, CPUInterruptType::NMI);
    }

    pub fn hardware_reset(cpu : &mut CPU)
    {
        CPUInterrupts::interrupt(cpu, CPUInterruptType::RESET);
    }

    fn interrupt(cpu : &mut CPU, interrupt_type : CPUInterruptType)
    {
        if (interrupt_type == CPUInterruptType::IRQ && !cpu.flags.interrupt) ||
            interrupt_type == CPUInterruptType::NMI || interrupt_type == CPUInterruptType::RESET
        {
            let mut cpu_flags_to_backup = cpu.flags.clone();
            cpu_flags_to_backup._break = interrupt_type == CPUInterruptType::IRQ;
            cpu_flags_to_backup.reserved = true;

            CPUStack::push_address(cpu, cpu.program_counter);
            CPUStack::push_byte(cpu, cpu_flags_to_backup.to_byte());

            cpu.flags.interrupt = true;

            let vector = match interrupt_type
            {
                CPUInterruptType::IRQ => (cpu.bus.program_rom.len()-2) as address,
                CPUInterruptType::NMI => (cpu.bus.program_rom.len()-6) as address,
                CPUInterruptType::RESET => (cpu.bus.program_rom.len()-4) as address,
            };

            if !cpu.are_interrupt_vectors_disabled
            {
                let low = cpu.bus.program_rom.get(vector);
                let high = cpu.bus.program_rom.get(vector+1);
                let interrupt_handler_address = address_from_high_low!(high, low);
                cpu.program_counter = interrupt_handler_address;
            }
        }
    }
}
