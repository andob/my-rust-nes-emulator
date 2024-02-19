mod square_synth;
mod triangle_synth;
mod mixer;
mod noise_synth;
mod speaker;
mod flags;
mod communication;
mod clock;

use anyhow::{Context, Result};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::codeloc;
use crate::system::apu::clock::APUClock;
use crate::system::apu::flags::frame_counter_flags::APUFrameCounterFlags;
use crate::system::apu::flags::status_flags::APUStatusFlags;
use crate::system::apu::mixer::Mixer;
use crate::system::apu::noise_synth::NoiseSynth;
use crate::system::apu::speaker::Speaker;
use crate::system::apu::square_synth::SquareSynth;
use crate::system::apu::triangle_synth::TriangleSynth;
use crate::system::apu_channels::APUToCPUChannels;
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
    pub clock : APUClock,
}

pub struct APURunEnvironment
{
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
    pub should_disable_audio : bool,
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
            clock: APUClock::new(),
        };
    }

    pub fn run(self : &mut APU, env : APURunEnvironment) -> Result<()>
    {
        if env.should_disable_audio { return Ok(()) }
        let apu = self;

        let mut speaker = Speaker::new().context(codeloc!())?;
        speaker.play();

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { speaker.pause(); return Ok(()) }

            let waveform_index = speaker.advance_to_next_waveform_index();
            let waveform_value = Mixer::mix(&apu, waveform_index);
            speaker.accept_waveform_value(waveform_value);

            apu.handle_read_commands_from_cpu();
            apu.handle_write_commands_from_cpu();

            if apu.clock.should_notify_apu_frame_has_ended()
            {
                apu.cpu_channels.signal_frame_end();
            }
        }
    }
}
