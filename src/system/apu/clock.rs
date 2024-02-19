
pub struct APUClock
{
    cycle_count : u64,
    cycle_count_threshold : u64,
}

impl APUClock
{
    pub fn new() -> APUClock
    {
        //todo thresholds should not be hardcoded, thresholds should be determined based on hardware capabilities!
        return APUClock
        {
            cycle_count: 0,
            cycle_count_threshold: 30000,
        }
    }

    pub fn should_notify_apu_frame_has_ended(&mut self) -> bool
    {
        self.cycle_count += 1;
        if self.cycle_count >= self.cycle_count_threshold
        {
            self.cycle_count = 0;
            return true;
        }

        return false;
    }
}
