use flume::{Receiver, Sender, TryRecvError};
use crate::system::{address, byte, System};
use crate::system::debugger::LoggingOptions;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum CPUToPPUCommTarget
{
    ControlFlags,
    MaskFlags,
    StatusFlags,
    OAMAddress,
    OAMData,
    ScrollPosition,
    BusAddress,
    BusData,
    OAM_DMA,
    Joystick,
    Unknown,
}

impl CPUToPPUCommTarget
{
    fn from_address(address : address) -> CPUToPPUCommTarget
    {
        return match address
        {
            0x2000 => CPUToPPUCommTarget::ControlFlags,
            0x2001 => CPUToPPUCommTarget::MaskFlags,
            0x2002 => CPUToPPUCommTarget::StatusFlags,
            0x2003 => CPUToPPUCommTarget::OAMAddress,
            0x2004 => CPUToPPUCommTarget::OAMData,
            0x2005 => CPUToPPUCommTarget::ScrollPosition,
            0x2006 => CPUToPPUCommTarget::BusAddress,
            0x2007 => CPUToPPUCommTarget::BusData,
            0x4014 => CPUToPPUCommTarget::OAM_DMA,
            0x4016 => CPUToPPUCommTarget::Joystick,
            _      => CPUToPPUCommTarget::Unknown,
        }
    }
}

pub struct PPUToCPUChannels
{
    logging_options : LoggingOptions,
    write_command_receiver : Receiver<(CPUToPPUCommTarget, Box<[byte]>)>,
    read_command_receiver : Receiver<CPUToPPUCommTarget>,
    read_command_result_sender : Sender<byte>,
    vblank_signal_sender : Sender<()>,
}

pub struct CPUToPPUChannels
{
    logging_options : LoggingOptions,
    write_command_sender : Sender<(CPUToPPUCommTarget, Box<[byte]>)>,
    read_command_sender : Sender<CPUToPPUCommTarget>,
    read_command_result_receiver : Receiver<byte>,
    vblank_signal_receiver : Receiver<()>,
}

impl PPUToCPUChannels
{
    pub fn get_read_command_from_cpu(&mut self) -> Result<CPUToPPUCommTarget, TryRecvError>
    {
        return self.read_command_receiver.try_recv();
    }

    pub fn get_write_command_from_cpu(&mut self) -> Result<(CPUToPPUCommTarget, Box<[byte]>), TryRecvError>
    {
        return self.write_command_receiver.try_recv();
    }

    pub fn respond_to_read_command_from_cpu(&self, target : CPUToPPUCommTarget, value : byte)
    {
        self.read_command_result_sender.send(value).unwrap_or_default();
        if self.logging_options.is_cpu_to_ppu_channel_logging_enabled
        {
            println!("[PPU→CPU] {:#04X} {:?}", value, target);
        }
    }

    pub fn signal_vblank(&self)
    {
        self.vblank_signal_sender.send(()).unwrap_or_default();
    }
}

impl CPUToPPUChannels
{
    pub fn read(&mut self, address : address) -> byte
    {
        let target = CPUToPPUCommTarget::from_address(address);
        self.read_command_sender.send(target).unwrap_or_default();
        return self.read_command_result_receiver.recv().unwrap_or_default();
    }

    pub fn write(&self, address : address, values : Box<[byte]>)
    {
        let target = CPUToPPUCommTarget::from_address(address);
        if self.logging_options.is_cpu_to_ppu_channel_logging_enabled
        {
            let first_value = values.first().cloned().unwrap_or_default();
            println!("[CPU→PPU] {:#04X} {:?}", first_value, target);
        }

        self.write_command_sender.send((target, values)).unwrap_or_default();
    }

    pub fn ppu_is_signaling_that_vblank_has_started(&mut self) -> bool
    {
        return self.vblank_signal_receiver.try_recv().is_ok();
    }
}

impl System
{
    pub fn create_ppu_system_channels(logging_options : LoggingOptions) -> (CPUToPPUChannels, PPUToCPUChannels)
    {
        let (write_command_sender, write_command_receiver) = flume::unbounded::<(CPUToPPUCommTarget, Box<[byte]>)>();
        let (read_command_sender, read_command_receiver) = flume::unbounded::<CPUToPPUCommTarget>();
        let (read_command_result_sender, read_command_result_receiver) = flume::unbounded::<byte>();
        let (vblank_signal_sender, vblank_signal_receiver) = flume::unbounded::<()>();

        let cpu_to_ppu_channels = CPUToPPUChannels
        {
            logging_options: logging_options.clone(),
            write_command_sender: write_command_sender,
            read_command_sender: read_command_sender,
            read_command_result_receiver: read_command_result_receiver,
            vblank_signal_receiver: vblank_signal_receiver,
        };

        let ppu_to_cpu_channels = PPUToCPUChannels
        {
            logging_options: logging_options.clone(),
            write_command_receiver: write_command_receiver,
            read_command_receiver: read_command_receiver,
            read_command_result_sender: read_command_result_sender,
            vblank_signal_sender: vblank_signal_sender,
        };

        return (cpu_to_ppu_channels, ppu_to_cpu_channels);
    }
}
