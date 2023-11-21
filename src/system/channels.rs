use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use crate::system::{address, byte};
use crate::system::debugger::LoggingOptions;


pub struct PPUToCPUChannels
{
    is_logging_enabled : bool,
    write_command_receiver : Receiver<(CPUToPPUCommTarget, byte)>,
    read_command_receiver : Receiver<CPUToPPUCommTarget>,
    read_command_result_sender : Sender<byte>,
}

pub struct CPUToPPUChannels
{
    is_logging_enabled : bool,
    write_command_sender : Sender<(CPUToPPUCommTarget, byte)>,
    read_command_sender : Sender<CPUToPPUCommTarget>,
    read_command_result_receiver : Receiver<byte>,
}

#[derive(Copy, Clone, Debug)]
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
    Unknown,
}

impl CPUToPPUCommTarget
{
    fn from_address(address : address) -> CPUToPPUCommTarget
    {
        return match address
        {
            0x2000 => { CPUToPPUCommTarget::ControlFlags }
            0x2001 => { CPUToPPUCommTarget::MaskFlags }
            0x2002 => { CPUToPPUCommTarget::StatusFlags }
            0x2003 => { CPUToPPUCommTarget::OAMAddress }
            0x2004 => { CPUToPPUCommTarget::OAMData }
            0x2005 => { CPUToPPUCommTarget::ScrollPosition }
            0x2006 => { CPUToPPUCommTarget::BusAddress }
            0x2007 => { CPUToPPUCommTarget::BusData }
            0x4041 => { CPUToPPUCommTarget::OAM_DMA }
            _      => { CPUToPPUCommTarget::Unknown }
        }
    }
}

impl PPUToCPUChannels
{
    pub fn get_read_command_from_cpu(&self) -> Result<CPUToPPUCommTarget, TryRecvError>
    {
        return self.read_command_receiver.try_recv();
    }

    pub fn get_write_command_from_cpu(&self) -> Result<(CPUToPPUCommTarget, byte), TryRecvError>
    {
        return self.write_command_receiver.try_recv();
    }

    pub fn respond_to_read_command_from_cpu(&self, target : CPUToPPUCommTarget, value : byte)
    {
        if self.is_logging_enabled { println!("[PPU→CPU] {:#04X} {:?}", value, target); }
        self.read_command_result_sender.send(value).unwrap_or_default();
    }
}

impl CPUToPPUChannels
{
    pub fn read(&self, address : address) -> byte
    {
        let target = CPUToPPUCommTarget::from_address(address);
        self.read_command_sender.send(target).unwrap_or_default();
        return self.read_command_result_receiver.recv().unwrap_or_default();
    }

    pub fn write(&self, address : address, value : byte)
    {
        let target = CPUToPPUCommTarget::from_address(address);
        if self.is_logging_enabled { println!("[CPU→PPU] {:#04X} {:?}", value, target); }
        self.write_command_sender.send((target, value)).unwrap_or_default();
    }
}

pub struct SystemChannels
{
    pub cpu_to_ppu_channels : CPUToPPUChannels,
    pub ppu_to_cpu_channels : PPUToCPUChannels,
}

pub fn create_system_channels(logging_options : LoggingOptions) -> SystemChannels
{
    let (write_command_sender, write_command_receiver) = channel::<(CPUToPPUCommTarget, byte)>();
    let (read_command_sender, read_command_receiver) = channel::<CPUToPPUCommTarget>();
    let (read_command_result_sender, read_command_result_receiver) = channel::<byte>();

    let cpu_to_ppu_channels = CPUToPPUChannels
    {
        is_logging_enabled: logging_options.is_channel_logging_enabled,
        write_command_sender: write_command_sender,
        read_command_sender: read_command_sender,
        read_command_result_receiver: read_command_result_receiver,
    };

    let ppu_to_cpu_channels = PPUToCPUChannels
    {
        is_logging_enabled: logging_options.is_channel_logging_enabled,
        write_command_receiver: write_command_receiver,
        read_command_receiver: read_command_receiver,
        read_command_result_sender: read_command_result_sender
    };

    return SystemChannels { cpu_to_ppu_channels, ppu_to_cpu_channels };
}
