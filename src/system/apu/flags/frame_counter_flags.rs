use crate::system::byte;

#[derive(PartialEq, Eq)]
pub enum SequencerMode
{
    FourStep,
    FiveStep,
}

#[derive(PartialEq, Eq)]
pub struct APUFrameCounterFlags
{
    pub mode : SequencerMode, //todo how to use this?
    pub irq_inhibit_flag : bool, //todo how to use this?
}

impl APUFrameCounterFlags
{
    pub fn new() -> APUFrameCounterFlags
    {
        return APUFrameCounterFlags
        {
            mode: SequencerMode::FourStep,
            irq_inhibit_flag: false,
        }
    }

    pub fn to_byte(self : &APUFrameCounterFlags) -> byte
    {
        return ((if self.mode==SequencerMode::FiveStep {1} else {0} as byte) << 7)
             | ((self.irq_inhibit_flag                              as byte) << 6);
    }

    pub fn from_byte(value : byte) -> APUFrameCounterFlags
    {
        return APUFrameCounterFlags
        {
            mode:          if (value & 0b10000000) >> 7 == 1 { SequencerMode::FiveStep } else { SequencerMode::FourStep },
            irq_inhibit_flag: (value & 0b01000000) >> 6 == 1,
        };
    }
}

impl Clone for APUFrameCounterFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return APUFrameCounterFlags::from_byte(byte);
    }
}
