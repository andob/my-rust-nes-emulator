use std::collections::HashMap;
use crate::system::{address, byte};
use crate::system::cpu::flags::CPUFlags;
use std::sync::mpsc::{Receiver, Sender};
use crate::system::cpu::CPU;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

#[derive(Clone)]
pub struct LoggingOptions
{
    pub is_cpu_opcode_logging_enabled : bool,
    pub is_cpu_too_slow_warning_logging_enabled : bool,
}

impl LoggingOptions
{
    pub fn defaults() -> LoggingOptions
    {
        return LoggingOptions
        {
            is_cpu_opcode_logging_enabled: true,
            is_cpu_too_slow_warning_logging_enabled: false,
        };
    }
}

#[derive(Clone, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct CPUState
{
    pub A : byte, //Accumulator register
    pub X : byte, //X index register
    pub Y : byte, //Y index register
    pub stack_pointer : byte,
    pub program_counter : address,
    pub flags : CPUFlags,
}

impl From<&CPU> for CPUState
{
    fn from(cpu : &CPU) -> Self
    {
        return CPUState
        {
            A: cpu.A,
            X: cpu.X,
            Y: cpu.Y,
            stack_pointer: cpu.stack.get_pointer(),
            program_counter: cpu.program_counter,
            flags: cpu.flags.clone(),
        }
    }
}

pub struct RAMSnapshot
{
    pub variables : HashMap<address, byte>
}

impl RAMSnapshot
{
    pub fn new(variables : HashMap<address, byte>) -> RAMSnapshot { RAMSnapshot { variables } }
}

pub struct CPUDebugger
{
    pub cpu_state_watcher : Option<Sender<CPUState>>,
    pub cpu_bus_watcher_targets : Vec<address>,
    pub cpu_bus_watcher : Option<Sender<RAMSnapshot>>,
    pub cpu_bus_setter : Option<Receiver<RAMSnapshot>>,
}

impl CPUDebugger
{
    pub fn new() -> CPUDebugger
    {
        return CPUDebugger
        {
            cpu_state_watcher: None,
            cpu_bus_watcher_targets: Vec::new(),
            cpu_bus_watcher: None,
            cpu_bus_setter: None,
        };
    }

    pub fn notify_cpu_state_to_watchers(&self, cpu : &mut CPU)
    {
        if let Some(sender) = &self.cpu_state_watcher
        {
            sender.send(CPUState::from(&*cpu)).unwrap_or_default();
        }

        if let Some(sender) = &self.cpu_bus_watcher
        {
            sender.send(RAMSnapshot::new(self.cpu_bus_watcher_targets.iter()
                .map(|address| (*address, cpu.bus.get(*address)))
                .collect())).unwrap_or_default();
        }

        if let Some(receiver) = &self.cpu_bus_setter
        {
            if let Ok(ram_snapshot) = receiver.recv()
            {
                for (address, value) in ram_snapshot.variables
                {
                    cpu.bus.put(address, value);
                }
            }
        }
    }
}
