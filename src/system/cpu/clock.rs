use std::thread;
use std::time::Duration;
use nix::sys::time::TimeValLike;
use nix::time::{clock_gettime, ClockId};
use crate::system::cpu::opcodes::Opcode;
use crate::system::debugger::LoggingOptions;

#[allow(non_camel_case_types)]
pub enum ExpectedDuration
{
    _2,  //2 CPU clock cycles
    _3,  //3 CPU clock cycles
    _4,  //4 CPU clock cycles
    _4p, //4 CPU clock cycles + 1 if page boundary was crossed
    _5,  //5 CPU clock cycles
    _5p, //5 CPU clock cycles + 1 if page boundary was crossed
    _6,  //6 CPU clock cycles
    _7,  //7 CPU clock cycles
    _8,  //8 CPU clock cycles
    bra, //branch: 1 CPU clock cycle + 1 if branch was taken + 1 if page boundary was crossed
}

pub struct CPUClock
{
    frequency : u64,
    was_page_boundary_crossed : bool,
    was_branch_taken : bool,
    cycle_started_at : u64,
}

impl CPUClock
{
    pub fn new() -> CPUClock
    {
        return CPUClock
        {
            frequency: 1660000, //1.66Mhz
            was_page_boundary_crossed: false,
            was_branch_taken: false,
            cycle_started_at: 0,
        };
    }

    pub fn now() -> u64
    {
        //todo make it work on windows and mac os
        let result = clock_gettime(ClockId::CLOCK_MONOTONIC);
        return if let Ok(value) = result { value.num_nanoseconds() as u64 } else { 0 };
    }

    pub fn notify_page_boundary_crossed(&mut self)
    {
        self.was_page_boundary_crossed = true;
    }

    pub fn notify_branch_taken(&mut self)
    {
        self.was_branch_taken = true;
    }

    pub fn notify_cpu_cycle_started(&mut self)
    {
        self.was_page_boundary_crossed = false;
        self.was_branch_taken = false;
        self.cycle_started_at = CPUClock::now();
    }

    pub fn notify_cpu_cycle_stopped(&mut self, opcode : &Opcode, logging_options : &LoggingOptions)
    {
        let expected_duration : u64 = match opcode.expected_duration
        {
            ExpectedDuration::_2  => 2,
            ExpectedDuration::_3  => 3,
            ExpectedDuration::_4  => 4,
            ExpectedDuration::_4p => 4+(self.was_page_boundary_crossed as u64),
            ExpectedDuration::_5  => 5,
            ExpectedDuration::_5p => 5+(self.was_page_boundary_crossed as u64),
            ExpectedDuration::_6  => 6,
            ExpectedDuration::_7  => 7,
            ExpectedDuration::_8  => 8,
            ExpectedDuration::bra => 1+(self.was_page_boundary_crossed as u64)+(self.was_branch_taken as u64),
        };

        let one_second_in_ns = 1_000_000_000u64; //1second
        let target_nanoseconds = expected_duration*(one_second_in_ns/self.frequency);

        let now = CPUClock::now();
        let elapsed_nanoseconds = now-self.cycle_started_at;

        if elapsed_nanoseconds >= target_nanoseconds
        {
            if logging_options.is_cpu_too_slow_warning_logging_enabled
            {
                println!("[CPU] CPU IS TOO SLOW! {} < {}", target_nanoseconds, elapsed_nanoseconds);
            }
        }
        else
        {
            let nanoseconds_to_sleep = target_nanoseconds-elapsed_nanoseconds;
            thread::sleep(Duration::from_nanos(nanoseconds_to_sleep));
        }
    }
}
