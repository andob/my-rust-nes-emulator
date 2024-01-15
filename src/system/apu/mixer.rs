use crate::system::apu::{APU, Synthesizer};

pub struct Mixer {}

impl Mixer
{
    pub fn mix(apu : &APU, waveform_index : f64) -> f64
    {
        //todo implement square 1
        // let square1 = 0.0;
        let square1 = if apu.status_flags.is_square1_enabled
            { apu.square1_synth.synthesize(waveform_index) }
        else { 0.0 };

        //todo implement square 2
        // let square2 = 0.0;
        let square2 = if apu.status_flags.is_square2_enabled
            { apu.square2_synth.synthesize(waveform_index) }
        else { 0.0 };

        //todo implement triangle
        // let triangle = 0.0;
        let triangle = if apu.status_flags.is_triangle_enabled
            { apu.triangle_synth.synthesize(waveform_index) }
        else { 0.0 };

        //todo implement noise
        // let noise = 0.0;
        let noise = if apu.status_flags.is_noise_enabled
            { apu.noise_synth.synthesize(waveform_index) }
        else { 0.0 };

        return (0.2632 * (square1 + square2)) + (0.29785 * triangle + 0.1729 * noise);
    }
}
