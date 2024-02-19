
pub struct PPUClock
{
    cycle_count : u64,
    cycle_count_vblank_start_threshold : u64,
    cycle_count_vblank_end_threshold : u64,
    was_vblank_start_notified : bool,
    was_vblank_end_notified : bool,
}

pub struct PPUClockTickResult
{
    pub should_notify_vblank_status_started : bool,
    pub should_notify_vblank_status_ended : bool,
}

impl PPUClock
{
    pub fn new() -> PPUClock
    {
        //todo thresholds should not be hardcoded, thresholds should be determined based on hardware capabilities!
        return PPUClock
        {
            cycle_count: 0,
            cycle_count_vblank_start_threshold: 5000,
            cycle_count_vblank_end_threshold: 10000,
            was_vblank_start_notified: false,
            was_vblank_end_notified: false,
        }
    }

    pub fn tick(&mut self) -> PPUClockTickResult
    {
        self.cycle_count += 1;

        return if self.cycle_count >= self.cycle_count_vblank_end_threshold
        {
            if !self.was_vblank_end_notified
            {
                self.was_vblank_end_notified = true;

                PPUClockTickResult
                {
                    should_notify_vblank_status_started: false,
                    should_notify_vblank_status_ended: true,
                }
            }
            else
            {
                self.cycle_count = 0;
                self.was_vblank_start_notified = false;
                self.was_vblank_end_notified = false;

                PPUClockTickResult
                {
                    should_notify_vblank_status_started: false,
                    should_notify_vblank_status_ended: false,
                }
            }
        }
        else if self.cycle_count >= self.cycle_count_vblank_start_threshold
        {
            if !self.was_vblank_start_notified
            {
                self.was_vblank_start_notified = true;

                PPUClockTickResult
                {
                    should_notify_vblank_status_started: true,
                    should_notify_vblank_status_ended: false,
                }
            }
            else
            {
                PPUClockTickResult
                {
                    should_notify_vblank_status_started: false,
                    should_notify_vblank_status_ended: false,
                }
            }
        }
        else
        {
            PPUClockTickResult
            {
                should_notify_vblank_status_started: false,
                should_notify_vblank_status_ended: false,
            }
        }
    }
}
