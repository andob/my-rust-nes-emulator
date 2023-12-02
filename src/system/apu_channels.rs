use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::system::{address, byte, System};
use crate::system::debugger::LoggingOptions;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CPUToAPUCommTarget
{
    Square1Envelope,
    Square1Sweep,
    Square1PeriodLow,
    Square1PeriodHigh,
    Square2Envelope,
    Square2Sweep,
    Square2PeriodLow,
    Square2PeriodHigh,
    TriangleCounter,
    TrianglePeriodLow,
    TrianglePeriodHigh,
    NoiseVolume,
    NoisePeriodLow,
    NoisePeriodHigh,
    StatusFlags,
    FrameCounterFlags,
    Unknown,
}

impl CPUToAPUCommTarget
{
    pub fn from_address(address : address) -> CPUToAPUCommTarget
    {
        return match address
        {
            0x4000 => CPUToAPUCommTarget::Square1Envelope,
            0x4001 => CPUToAPUCommTarget::Square1Sweep,
            0x4002 => CPUToAPUCommTarget::Square1PeriodLow,
            0x4003 => CPUToAPUCommTarget::Square1PeriodHigh,
            0x4004 => CPUToAPUCommTarget::Square2Envelope,
            0x4005 => CPUToAPUCommTarget::Square2Sweep,
            0x4006 => CPUToAPUCommTarget::Square2PeriodLow,
            0x4007 => CPUToAPUCommTarget::Square2PeriodHigh,
            0x4008 => CPUToAPUCommTarget::TriangleCounter,
            0x400A => CPUToAPUCommTarget::TrianglePeriodLow,
            0x400B => CPUToAPUCommTarget::TrianglePeriodHigh,
            0x400C => CPUToAPUCommTarget::NoiseVolume,
            0x400E => CPUToAPUCommTarget::NoisePeriodLow,
            0x400F => CPUToAPUCommTarget::NoisePeriodHigh,
            0x4015 => CPUToAPUCommTarget::StatusFlags,
            0x4017 => CPUToAPUCommTarget::FrameCounterFlags,
            _      => CPUToAPUCommTarget::Unknown,
        };
    }
}

pub struct APUToCPUChannels
{
    logging_options : LoggingOptions,
    write_command_receiver : Receiver<(CPUToAPUCommTarget, byte)>,
    read_command_receiver : Receiver<CPUToAPUCommTarget>,
    read_command_result_sender : Sender<byte>,
    frame_end_signal_sender : Sender<()>,
}

pub struct CPUToAPUChannels
{
    logging_options : LoggingOptions,
    write_command_sender : Sender<(CPUToAPUCommTarget, byte)>,
    read_command_sender : Sender<CPUToAPUCommTarget>,
    read_command_result_receiver : Receiver<byte>,
    frame_end_signal_receiver : Receiver<()>,
}

impl APUToCPUChannels
{
    pub fn get_read_command_from_cpu(&self) -> Result<CPUToAPUCommTarget, TryRecvError>
    {
        return self.read_command_receiver.try_recv();
    }

    pub fn get_write_command_from_cpu(&self) -> Result<(CPUToAPUCommTarget, byte), TryRecvError>
    {
        return self.write_command_receiver.try_recv();
    }

    pub fn respond_to_read_command_from_cpu(&self, target : CPUToAPUCommTarget, value : byte)
    {
        self.read_command_result_sender.send(value).unwrap_or_default();
        if self.logging_options.is_cpu_to_apu_channel_logging_enabled
        {
            println!("[APU→CPU] {:#04X} {:?}", value, target);
        }
    }

    pub fn signal_frame_end(&self)
    {
        self.frame_end_signal_sender.send(()).unwrap_or_default();
    }
}

impl CPUToAPUChannels
{
    pub fn read(&self, address : address) -> byte
    {
        let target = CPUToAPUCommTarget::from_address(address);
        self.read_command_sender.send(target).unwrap_or_default();
        return self.read_command_result_receiver.recv().unwrap_or_default();
    }

    pub fn write(&self, address : address, value : byte)
    {
        let target = CPUToAPUCommTarget::from_address(address);
        self.write_command_sender.send((target, value)).unwrap_or_default();
        if self.logging_options.is_cpu_to_apu_channel_logging_enabled
        {
            println!("[CPU→APU] {:#04X} {:?}", value, target);
        }
    }

    pub fn is_apu_signaling_that_frame_has_ended(&self) -> bool
    {
        return self.frame_end_signal_receiver.try_recv().is_ok();
    }
}

impl System
{
    pub fn create_apu_system_channels(logging_options : LoggingOptions) -> (CPUToAPUChannels, APUToCPUChannels)
    {
        let (write_command_sender, write_command_receiver) = channel::<(CPUToAPUCommTarget, byte)>();
        let (read_command_sender, read_command_receiver) = channel::<CPUToAPUCommTarget>();
        let (read_command_result_sender, read_command_result_receiver) = channel::<byte>();
        let (frame_end_signal_sender, frame_end_signal_receiver) = channel::<()>();

        let cpu_to_apu_channels = CPUToAPUChannels
        {
            logging_options: logging_options.clone(),
            write_command_sender: write_command_sender,
            read_command_sender: read_command_sender,
            read_command_result_receiver: read_command_result_receiver,
            frame_end_signal_receiver: frame_end_signal_receiver,
        };

        let apu_to_cpu_channels = APUToCPUChannels
        {
            logging_options: logging_options.clone(),
            write_command_receiver: write_command_receiver,
            read_command_receiver: read_command_receiver,
            read_command_result_sender: read_command_result_sender,
            frame_end_signal_sender: frame_end_signal_sender,
        };

        return (cpu_to_apu_channels, apu_to_cpu_channels);
    }
}
