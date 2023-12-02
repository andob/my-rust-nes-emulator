mod square_synth;
mod triangle_synth;
mod mixer;
mod noise_synth;
mod speaker;
mod flags;

use anyhow::{Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::codeloc;
use crate::system::apu::flags::frame_counter_flags::APUFrameCounterFlags;
use crate::system::apu::flags::status_flags::APUStatusFlags;
use crate::system::apu::mixer::Mixer;
use crate::system::apu::noise_synth::NoiseSynth;
use crate::system::apu::speaker::Speaker;
use crate::system::apu::square_synth::SquareSynth;
use crate::system::apu::triangle_synth::TriangleSynth;
use crate::system::apu_channels::{APUToCPUChannels, CPUToAPUCommTarget};
use crate::system::byte;
use crate::system::debugger::LoggingOptions;

pub struct APU
{
    pub square1_synth : SquareSynth,
    pub square2_synth : SquareSynth,
    pub triangle_synth : TriangleSynth,
    pub noise_synth : NoiseSynth,
    pub status_flags : APUStatusFlags,
    pub frame_counter_flags : APUFrameCounterFlags,
    pub cpu_channels : APUToCPUChannels,
}

pub struct APURunEnvironment
{
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
}

pub trait Synthesizer
{
    fn synthesize(&self, waveform_index : f64) -> f64;
}

impl APU
{
    pub fn new(channels : APUToCPUChannels) -> APU
    {
        return APU
        {
            square1_synth: SquareSynth::new(),
            square2_synth: SquareSynth::new(),
            triangle_synth: TriangleSynth::new(),
            noise_synth: NoiseSynth::new(),
            status_flags: APUStatusFlags::new(),
            frame_counter_flags: APUFrameCounterFlags::new(),
            cpu_channels: channels,
        };
    }

    pub fn run(self : &mut APU, env : APURunEnvironment) -> Result<()>
    {
        let apu = self;

        let mut speaker = Speaker::new().context(codeloc!())?;
        speaker.play();

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { speaker.pause(); return Ok(()); }

            let waveform_index = speaker.advance_to_next_waveform_index();
            let waveform_value = Mixer::mix(&apu, waveform_index);
            speaker.accept_waveform_value(waveform_value);

            if let Ok(target) = apu.cpu_channels.get_read_command_from_cpu()
            {
                apu.cpu_channels.respond_to_read_command_from_cpu(target, match target
                {
                    CPUToAPUCommTarget::StatusFlags => apu.status_flags.to_byte_for_cpu_reading(&apu),
                    CPUToAPUCommTarget::FrameCounterFlags => apu.frame_counter_flags.to_byte(),
                    _ => 0 as byte,
                });
            }

            match apu.cpu_channels.get_write_command_from_cpu()
            {
                Ok((CPUToAPUCommTarget::Square1Envelope, value)) => { apu.square1_synth.set_envelope(value); }
                Ok((CPUToAPUCommTarget::Square1Sweep, value)) => { apu.square1_synth.set_sweep(value); }
                Ok((CPUToAPUCommTarget::Square1PeriodLow, value)) => { apu.square1_synth.set_period_low(value); }
                Ok((CPUToAPUCommTarget::Square1PeriodHigh, value)) => { apu.square1_synth.set_period_high(value); }
                Ok((CPUToAPUCommTarget::Square2Envelope, value)) => { apu.square2_synth.set_envelope(value); }
                Ok((CPUToAPUCommTarget::Square2Sweep, value)) => { apu.square2_synth.set_sweep(value); }
                Ok((CPUToAPUCommTarget::Square2PeriodLow, value)) => { apu.square2_synth.set_period_low(value); }
                Ok((CPUToAPUCommTarget::Square2PeriodHigh, value)) => { apu.square2_synth.set_period_high(value); }
                Ok((CPUToAPUCommTarget::TriangleCounter, value)) => { apu.triangle_synth.set_counter(value); }
                Ok((CPUToAPUCommTarget::TrianglePeriodLow, value)) => { apu.triangle_synth.set_period_low(value); }
                Ok((CPUToAPUCommTarget::TrianglePeriodHigh, value)) => { apu.triangle_synth.set_period_high(value); }
                Ok((CPUToAPUCommTarget::NoiseVolume, value)) => { apu.noise_synth.set_volume(value); }
                Ok((CPUToAPUCommTarget::NoisePeriodLow, value)) => { apu.noise_synth.set_period_low(value); }
                Ok((CPUToAPUCommTarget::NoisePeriodHigh, value)) => { apu.noise_synth.set_period_high(value); }
                Ok((CPUToAPUCommTarget::StatusFlags, value)) => { apu.status_flags = APUStatusFlags::from_byte_for_cpu_writing(&apu, value); }
                Ok((CPUToAPUCommTarget::FrameCounterFlags, value)) => { apu.frame_counter_flags = APUFrameCounterFlags::from_byte(value); }
                _ => {}
            }

            //todo implement a proper clock
            // thread::sleep(Duration::from_millis(17));

            //todo apu.cpu_channels.signal_frame_end();
        }
    }
}
