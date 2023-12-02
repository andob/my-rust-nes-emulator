use crate::system::apu::APU;
use crate::system::byte;

#[derive(PartialEq, Eq)]
pub struct APUStatusFlags
{
    pub is_dmc_enabled : bool, //todo use this
    pub is_noise_enabled : bool, //todo use this
    pub is_triangle_enabled : bool, //todo use this
    pub is_square2_enabled : bool, //todo use this
    pub is_square1_enabled : bool, //todo use this
    pub dmc_interrupt_flag : bool, //todo use this
    pub frame_interrupt_flag : bool, //todo use this
}

impl APUStatusFlags
{
    pub fn new() -> APUStatusFlags
    {
        return APUStatusFlags
        {
            is_dmc_enabled: false,
            is_noise_enabled: false,
            is_triangle_enabled: false,
            is_square2_enabled: false,
            is_square1_enabled: false,
            dmc_interrupt_flag: false,
            frame_interrupt_flag: false,
        };
    }

    pub fn to_byte(self : &APUStatusFlags) -> byte
    {
        return ((self.dmc_interrupt_flag   as byte) << 7)
             | ((self.frame_interrupt_flag as byte) << 6)
             | ((self.is_dmc_enabled       as byte) << 4)
             | ((self.is_noise_enabled     as byte) << 3)
             | ((self.is_triangle_enabled  as byte) << 2)
             | ((self.is_square2_enabled   as byte) << 1)
             | ((self.is_square1_enabled   as byte) << 0);
    }

    pub fn from_byte(value : byte) -> APUStatusFlags
    {
        return APUStatusFlags
        {
            dmc_interrupt_flag:   (value & 0b10000000) >> 7 == 1,
            frame_interrupt_flag: (value & 0b01000000) >> 6 == 1,
            is_dmc_enabled:       (value & 0b00010000) >> 4 == 1,
            is_noise_enabled:     (value & 0b00001000) >> 3 == 1,
            is_triangle_enabled:  (value & 0b00000100) >> 2 == 1,
            is_square2_enabled:   (value & 0b00000010) >> 1 == 1,
            is_square1_enabled:   (value & 0b00000001) >> 0 == 1,
        };
    }

    pub fn to_byte_for_cpu_reading(self : &APUStatusFlags, apu : &APU) -> byte
    {
        //when reading flags to be sent to CPU, synthesizer statuses should also be sent
        let mut flags = self.clone();
        flags.dmc_interrupt_flag = flags.dmc_interrupt_flag; //todo to modify while implementing DMC
        flags.is_triangle_enabled = flags.is_triangle_enabled && apu.triangle_synth.length_counter_load>0; //todo is this condition correct
        flags.is_square1_enabled = flags.is_square1_enabled && apu.square1_synth.length_counter_load>0; //todo is this condition correct
        flags.is_square2_enabled = flags.is_square2_enabled && apu.square2_synth.length_counter_load>0; //todo is this condition correct
        flags.is_noise_enabled = flags.is_noise_enabled && apu.noise_synth.length_counter_load>0; //todo is this condition correct
        return flags.to_byte();
    }

    pub fn from_byte_for_cpu_writing(apu : &APU, value : byte) -> APUStatusFlags
    {
        //when writing flags that came from CPU, interrupt flags should be kept
        let old_flags = &apu.status_flags;
        let mut new_flags = APUStatusFlags::from_byte(value);
        new_flags.dmc_interrupt_flag = old_flags.dmc_interrupt_flag; //todo is this implemented correctly?
        new_flags.frame_interrupt_flag = old_flags.frame_interrupt_flag; //todo is this implemented correctly?
        return new_flags;
    }
}

impl Clone for APUStatusFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return APUStatusFlags::from_byte(byte);
    }
}
