use anyhow::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::system::apu_channels::{APUToCPUChannels, CPUToAPUCommTarget};
use crate::system::byte;
use crate::system::debugger::LoggingOptions;

pub struct APU
{
    pub cpu_channels : APUToCPUChannels,
}

pub struct APURunEnvironment
{
    pub logging_options : LoggingOptions,
    pub is_shutting_down : Arc<AtomicBool>,
}

impl APU
{
    pub fn new(channels : APUToCPUChannels) -> APU
    {
        return APU
        {
            cpu_channels: channels,
        };
    }

    pub fn run(self : &mut APU, env : APURunEnvironment) -> Result<()>
    {
        let apu = self;

        loop
        {
            if env.is_shutting_down.load(Ordering::Relaxed) { return Ok(()); }

            if let Ok(target) = apu.cpu_channels.get_read_command_from_cpu()
            {
                apu.cpu_channels.respond_to_read_command_from_cpu(target, match target
                {
                    CPUToAPUCommTarget::Status => { 0 as byte },
                    _ => { 0 as byte },
                });
            }

            match apu.cpu_channels.get_write_command_from_cpu()
            {
                Ok((CPUToAPUCommTarget::Square1Envelope, value)) => {}
                Ok((CPUToAPUCommTarget::Square1Sweep, value)) => {}
                Ok((CPUToAPUCommTarget::Square1PeriodLow, value)) => {}
                Ok((CPUToAPUCommTarget::Square1PeriodHigh, value)) => {}
                Ok((CPUToAPUCommTarget::Square2Envelope, value)) => {}
                Ok((CPUToAPUCommTarget::Square2Sweep, value)) => {}
                Ok((CPUToAPUCommTarget::Square2PeriodLow, value)) => {}
                Ok((CPUToAPUCommTarget::Square2PeriodHigh, value)) => {}
                Ok((CPUToAPUCommTarget::SawCounter, value)) => {}
                Ok((CPUToAPUCommTarget::SawPeriodLow, value)) => {}
                Ok((CPUToAPUCommTarget::SawPeriodHigh, value)) => {}
                Ok((CPUToAPUCommTarget::NoiseVolume, value)) => {}
                Ok((CPUToAPUCommTarget::NoisePeriodLow, value)) => {}
                Ok((CPUToAPUCommTarget::NoisePeriodHigh, value)) => {}
                Ok((CPUToAPUCommTarget::Status, value)) => {}
                Ok((CPUToAPUCommTarget::FrameCounter, value)) => {}
                _ => {}
            }

            //todo implement a proper clock
            thread::sleep(Duration::from_millis(1000));

            apu.cpu_channels.signal_frame_end();
        }
    }
}
