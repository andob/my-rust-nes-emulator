use std::thread;
use std::time::Duration;
use crate::system::cpu::opcodes::Opcode;

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
    cycle_count : u64,
    cycle_count_threshold : u64,
    sleep_duration : Duration,
    was_page_boundary_crossed : bool,
    was_branch_taken : bool,
}

impl CPUClock
{
    pub fn new() -> CPUClock
    {
        //todo thresholds should not be hardcoded, thresholds should be determined based on hardware capabilities!
        return CPUClock
        {
            cycle_count: 0,
            cycle_count_threshold: 3000,
            sleep_duration: Duration::from_millis(1),
            was_page_boundary_crossed: false,
            was_branch_taken: false,
        };
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
    }

    pub fn notify_cpu_cycle_stopped(&mut self, opcode : &Opcode)
    {
        let current_cycle_count : u64 = match opcode.expected_duration
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

        self.cycle_count += current_cycle_count;

        if self.cycle_count >= self.cycle_count_threshold
        {
            self.cycle_count = 0;
            thread::sleep(self.sleep_duration);
        }
    }
}
