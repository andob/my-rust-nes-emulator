use crate::system::apu::Synthesizer;
use crate::system::byte;

pub struct SquareSynth
{
    duty_cycle : byte, //todo how to use this?
    length_counter_halt_flag : bool, //todo how to use this?
    length_counter_load : byte, //todo how to use this?
    constant_volume_flag : bool, //todo how to use this?
    volume_divider_period : byte, //todo how to use this?
    is_sweep_enabled : bool, //todo how to use this?
    sweep_divider_period : byte, //todo how to use this?
    is_sweep_negated : bool, //todo how to use this?
    sweep_shift_count : byte, //todo how to use this?
    timer_low : byte, //todo how to use this?
    timer_high : byte, //todo how to use this?
}

impl SquareSynth
{
    pub fn new() -> SquareSynth
    {
        return SquareSynth
        {
            duty_cycle: 0,
            length_counter_halt_flag: false,
            constant_volume_flag: false,
            volume_divider_period: 0,
            is_sweep_enabled: false,
            sweep_divider_period: 0,
            is_sweep_negated: false,
            sweep_shift_count: 0,
            timer_low: 0,
            timer_high: 0,
            length_counter_load: 0,
        };
    }

    pub fn set_envelope(self : &mut SquareSynth, value : byte)
    {
        self.duty_cycle               = (value & 0b11000000) >> 6;
        self.length_counter_halt_flag = (value & 0b00100000) >> 5 == 1;
        self.constant_volume_flag     = (value & 0b00010000) >> 4 == 1;
        self.volume_divider_period    = (value & 0b00001111) >> 0;
    }

    pub fn set_sweep(self : &mut SquareSynth, value : byte)
    {
        self.is_sweep_enabled     = (value & 0b10000000) >> 7 == 1;
        self.sweep_divider_period = (value & 0b01110000) >> 4;
        self.is_sweep_negated     = (value & 0b00001000) >> 3 == 1;
        self.sweep_shift_count    = (value & 0b00000111) >> 0;
    }

    pub fn set_period_low(self : &mut SquareSynth, value : byte)
    {
        self.timer_low = value;
    }

    pub fn set_period_high(self : &mut SquareSynth, value : byte)
    {
        self.length_counter_load = (value & 0b11111000) >> 3;
        self.timer_high          = (value & 0b00000111) >> 0;
    }

    pub fn is_length_counter_loaded(&self) -> bool
    {
        return self.length_counter_load>0;
    }
}

impl Synthesizer for SquareSynth
{
    fn synthesize(&self, waveform_index : f64) -> f64
    {
        let two_pi = std::f64::consts::TAU;
        let frequency = 440.0;
        let sine = f64::sin(frequency * two_pi * waveform_index);
        return if sine >= 0.0 { 1.0 } else { -1.0 };
    }
}
