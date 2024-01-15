use rand::random;
use crate::system::apu::Synthesizer;
use crate::system::byte;

pub struct NoiseSynth
{
    period : byte, //todo how to use this?
    should_loop : bool, //todo how to use this?
    volume : byte, //todo how to use this?
    constant_volume_flag : bool, //todo how to use this?
    length_counter_halt_flag : bool, //todo how to use this?
    length_counter_load : byte, //todo how to use this?
}

impl NoiseSynth
{
    pub fn new() -> NoiseSynth
    {
        return NoiseSynth
        {
            period: 0,
            should_loop: false,
            volume: 0,
            constant_volume_flag: false,
            length_counter_halt_flag: false,
            length_counter_load: 0,
        };
    }

    pub fn set_volume(self : &mut NoiseSynth, value : byte)
    {
        self.length_counter_halt_flag = (value & 0b00100000) >> 5 == 1;
        self.constant_volume_flag     = (value & 0b00010000) >> 4 == 1;
        self.volume                   = (value & 0b00001111) >> 0;
    }

    pub fn set_period_low(self : &mut NoiseSynth, value : byte)
    {
        self.should_loop = (value & 0b10000000) >> 7 == 1;
        self.period      = (value & 0b00001111) >> 0;
    }

    pub fn set_period_high(self : &mut NoiseSynth, value : byte)
    {
        self.length_counter_load = (value & 0b11111000) >> 3;
    }

    pub fn is_length_counter_loaded(&self) -> bool
    {
        return self.length_counter_load>0;
    }
}

impl Synthesizer for NoiseSynth
{
    fn synthesize(&self, _waveform_index : f64) -> f64
    {
        return random::<f64>()*2.0-1.0;
    }
}
