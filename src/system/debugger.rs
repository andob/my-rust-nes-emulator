use crate::system::System;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

#[macro_export]
macro_rules! debug_log
{
    ($($arg:tt)*) =>
    {{
        println!($($arg)*);
    }};
}

pub trait Debugger
{
    fn before_cpu_opcode(&mut self, nes : &mut System);
    fn after_cpu_opcode(&mut self, nes : &mut System);
}

pub struct EmptyDebugger {}

impl EmptyDebugger
{
    pub fn new() -> EmptyDebugger { EmptyDebugger {} }
}

impl Debugger for EmptyDebugger
{
    fn before_cpu_opcode(&mut self, _nes : &mut System) {}

    fn after_cpu_opcode(&mut self, _nes : &mut System) {}
}
