use crate::address_from_high_low;
use crate::system::address;
use crate::system::cpu::CPU;
use crate::system::cpu::stack::CPUStack;

pub struct CPUInterrupts {}
impl CPUInterrupts
{
    //IRQ = interrupt request
    pub fn software_irq(cpu : &mut CPU)
    {
        CPUStack::push_address(cpu, cpu.program_counter+1);

        let mut cpu_flags_to_backup = cpu.flags.clone();
        cpu_flags_to_backup.reserved = true;
        cpu_flags_to_backup._break = true;
        CPUStack::push_byte(cpu, cpu_flags_to_backup.to_byte());

        cpu.flags.interrupt = true;

        let interrupt_vector = (cpu.bus.program_rom.len()-2) as address;
        CPUInterrupts::set_program_counter_from_vector(cpu, interrupt_vector);
    }

    pub fn hardware_irq(cpu : &mut CPU)
    {
        if !cpu.flags.interrupt
        {
            CPUStack::push_address(cpu, cpu.program_counter);

            let mut cpu_flags_to_backup = cpu.flags.clone();
            cpu_flags_to_backup.reserved = true;
            cpu_flags_to_backup._break = true;
            CPUStack::push_byte(cpu, cpu_flags_to_backup.to_byte());

            cpu.flags.interrupt = true;

            let interrupt_vector = (cpu.bus.program_rom.len()-2) as address;
            CPUInterrupts::set_program_counter_from_vector(cpu, interrupt_vector);
        }
    }

    //NMI = non-maskable interrupt
    pub fn hardware_nmi(cpu : &mut CPU)
    {
        CPUStack::push_address(cpu, cpu.program_counter);

        let mut cpu_flags_to_backup = cpu.flags.clone();
        cpu_flags_to_backup.reserved = true;
        CPUStack::push_byte(cpu, cpu_flags_to_backup.to_byte());

        cpu.flags.interrupt = true;

        let interrupt_vector = (cpu.bus.program_rom.len()-6) as address;
        CPUInterrupts::set_program_counter_from_vector(cpu, interrupt_vector);
    }

    pub fn hardware_reset(cpu : &mut CPU)
    {
        CPUStack::push_address(cpu, cpu.program_counter);

        let mut cpu_flags_to_backup = cpu.flags.clone();
        cpu_flags_to_backup.reserved = true;
        CPUStack::push_byte(cpu, cpu_flags_to_backup.to_byte());

        cpu.flags.interrupt = true;

        let interrupt_vector = (cpu.bus.program_rom.len()-4) as address;
        CPUInterrupts::set_program_counter_from_vector(cpu, interrupt_vector);
    }

    fn set_program_counter_from_vector(cpu : &mut CPU, interrupt_vector : address)
    {
        if !cpu.are_interrupt_vectors_disabled
        {
            let low = cpu.bus.program_rom.get(interrupt_vector);
            let high = cpu.bus.program_rom.get(interrupt_vector+1);
            let interrupt_handler_address = address_from_high_low!(high, low);
            cpu.program_counter = interrupt_handler_address;
        }
    }
}
