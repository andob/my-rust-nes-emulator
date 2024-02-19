use crate::system::apu::APU;
use crate::system::apu::flags::frame_counter_flags::APUFrameCounterFlags;
use crate::system::apu::flags::status_flags::APUStatusFlags;
use crate::system::apu_channels::CPUToAPUCommTarget;
use crate::system::byte;

impl APU
{
    pub fn handle_read_commands_from_cpu(&mut self)
    {
        let apu = self;
        if let Ok(target) = apu.cpu_channels.get_read_command_from_cpu()
        {
            apu.cpu_channels.respond_to_read_command_from_cpu(target, match target
            {
                CPUToAPUCommTarget::StatusFlags => apu.status_flags.to_byte_for_cpu_reading(&apu),
                CPUToAPUCommTarget::FrameCounterFlags => apu.frame_counter_flags.to_byte(),
                _ => 0 as byte,
            });
        }
    }

    pub fn handle_write_commands_from_cpu(&mut self)
    {
        let apu = self;
        match apu.cpu_channels.get_write_command_from_cpu()
        {
            Ok((CPUToAPUCommTarget::Square1Envelope, value)) =>
                { apu.square1_synth.set_envelope(value); }
            Ok((CPUToAPUCommTarget::Square1Sweep, value)) =>
                { apu.square1_synth.set_sweep(value); }
            Ok((CPUToAPUCommTarget::Square1PeriodLow, value)) =>
                { apu.square1_synth.set_period_low(value); }
            Ok((CPUToAPUCommTarget::Square1PeriodHigh, value)) =>
                { apu.square1_synth.set_period_high(value); }

            Ok((CPUToAPUCommTarget::Square2Envelope, value)) =>
                { apu.square2_synth.set_envelope(value); }
            Ok((CPUToAPUCommTarget::Square2Sweep, value)) =>
                { apu.square2_synth.set_sweep(value); }
            Ok((CPUToAPUCommTarget::Square2PeriodLow, value)) =>
                { apu.square2_synth.set_period_low(value); }
            Ok((CPUToAPUCommTarget::Square2PeriodHigh, value)) =>
                { apu.square2_synth.set_period_high(value); }

            Ok((CPUToAPUCommTarget::TriangleCounter, value)) =>
                { apu.triangle_synth.set_counter(value); }
            Ok((CPUToAPUCommTarget::TrianglePeriodLow, value)) =>
                { apu.triangle_synth.set_period_low(value); }
            Ok((CPUToAPUCommTarget::TrianglePeriodHigh, value)) =>
                { apu.triangle_synth.set_period_high(value); }

            Ok((CPUToAPUCommTarget::NoiseVolume, value)) =>
                { apu.noise_synth.set_volume(value); }
            Ok((CPUToAPUCommTarget::NoisePeriodLow, value)) =>
                { apu.noise_synth.set_period_low(value); }
            Ok((CPUToAPUCommTarget::NoisePeriodHigh, value)) =>
                { apu.noise_synth.set_period_high(value); }

            Ok((CPUToAPUCommTarget::StatusFlags, value)) =>
                { apu.status_flags = APUStatusFlags::from_byte_for_cpu_writing(&apu, value); }
            Ok((CPUToAPUCommTarget::FrameCounterFlags, value)) =>
                { apu.frame_counter_flags = APUFrameCounterFlags::from_byte(value); }
            _ => {}
        }
    }
}
