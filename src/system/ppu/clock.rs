
const NUMBER_OF_VISIBLE_SCAN_LINES : usize = 240;
const NUMBER_OF_SCAN_LINES : usize = 262;

const VBLANK_START_SCANLINE_NUMBER : usize = 241;
const VBLANK_END_SCANLINE_NUMBER : usize = 262;

pub struct PPUClock
{
    cycle_count : usize,
    scanline_cycle_count_threshold : usize,
    current_scanline : usize,
}

pub struct PPUClockTickResult
{
    pub should_notify_scanline_reached : bool,
    pub scanline_number : usize,
}

impl PPUClockTickResult
{
    pub fn should_notify_vblank_started(&self) -> bool
    {
        return self.should_notify_scanline_reached &&
            self.scanline_number == VBLANK_START_SCANLINE_NUMBER;
    }

    pub fn should_notify_vblank_ended(&self) -> bool
    {
        return self.should_notify_scanline_reached &&
            self.scanline_number == VBLANK_END_SCANLINE_NUMBER;
    }

    pub fn should_check_sprite_zero_hit_mid_frame(&self) -> bool
    {
        return self.should_notify_scanline_reached &&
                self.scanline_number <= NUMBER_OF_VISIBLE_SCAN_LINES &&
                self.scanline_number % 100 == 0
    }
}

impl PPUClock
{
    pub fn new() -> PPUClock
    {
        //todo thresholds should not be hardcoded, thresholds should be determined based on hardware capabilities!
        //todo how to keep constant FPS?
        return PPUClock
        {
            cycle_count: 0,
            scanline_cycle_count_threshold: 40,
            current_scanline: 0,
        };
    }

    pub fn tick(&mut self) -> PPUClockTickResult
    {
        self.cycle_count += 1;

        if self.cycle_count >= self.scanline_cycle_count_threshold
        {
            self.cycle_count = 0;
            self.current_scanline += 1;

            if self.current_scanline > VBLANK_END_SCANLINE_NUMBER
            {
                self.current_scanline = 0;
            }

            return PPUClockTickResult
            {
                should_notify_scanline_reached: true,
                scanline_number: self.current_scanline,
            };
        }

        return PPUClockTickResult
        {
            should_notify_scanline_reached: false,
            scanline_number: self.current_scanline,
        };
    }
}
