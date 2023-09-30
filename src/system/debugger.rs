use crate::system::System;

#[macro_export]
macro_rules! codeloc
{
    () => { format!("{}:{}", file!(), line!()) }
}

pub trait Debugger
{
    fn before_cpu_tick(&self, nes : &mut System, opcode_description : &String);
    fn after_cpu_tick(&self, nes : &mut System, opcode_description : &String);
}

pub struct DefaultDebugger {}

impl DefaultDebugger
{
    pub fn new() -> DefaultDebugger { DefaultDebugger {} }
}

impl Debugger for DefaultDebugger
{
    fn before_cpu_tick(&self, _nes : &mut System, opcode_description : &String)
    {
        println!("[CPU] {}", opcode_description);
    }

    fn after_cpu_tick(&self, _nes : &mut System, _opcode_description : &String) {}
}
