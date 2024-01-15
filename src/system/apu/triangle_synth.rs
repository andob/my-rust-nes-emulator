use crate::system::apu::Synthesizer;
use crate::system::byte;

pub struct TriangleSynth
{
    length_counter_halt_flag : bool, //todo how to use this?
    length_counter_load : byte, //todo how to use this?
    linear_counter_load : byte, //todo how to use this?
    timer_low : byte, //todo how to use this?
    timer_high : byte, //todo how to use this?
}

impl TriangleSynth
{
    pub fn new() -> TriangleSynth
    {
        return TriangleSynth
        {
            length_counter_halt_flag: false,
            length_counter_load: 0,
            linear_counter_load: 0,
            timer_low: 0,
            timer_high: 0,
        };
    }

    pub fn set_counter(self : &mut TriangleSynth, value : byte)
    {
        self.length_counter_halt_flag = (value & 0b10000000) >> 7 == 1;
        self.linear_counter_load      = (value & 0b01111111) >> 0;
    }

    pub fn set_period_low(self : &mut TriangleSynth, value : byte)
    {
        self.timer_low = value;
    }

    pub fn set_period_high(self : &mut TriangleSynth, value : byte)
    {
        self.length_counter_load = (value & 0b11111000) >> 3;
        self.timer_high          = (value & 0b00000111) >> 0;
    }

    pub fn is_length_counter_loaded(&self) -> bool
    {
        return self.length_counter_load>0;
    }
}

impl Synthesizer for TriangleSynth
{
    fn synthesize(&self, waveform_index : f64) -> f64
    {
        let two_pi = std::f64::consts::TAU;
        let pi_over_two = std::f64::consts::FRAC_PI_2;
        let frequency = 440.0;
        let sine = f64::sin(frequency * two_pi * waveform_index);
        return sine / pi_over_two - 1.0;
    }
}
