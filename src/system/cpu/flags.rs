
pub struct CPUFlags
{
    pub negative : bool,
    pub overflow : bool,
    pub reserved : bool,
    pub _break : bool,
    pub decimal : bool,
    pub interrupt : bool,
    pub zero : bool,
    pub carry : bool,
}

impl CPUFlags
{
    pub fn to_byte(self : &CPUFlags) -> u8
    {
        return ((self.negative  as u8) << 7)
             | ((self.overflow  as u8) << 6)
             | ((self.reserved  as u8) << 5)
             | ((self._break    as u8) << 4)
             | ((self.decimal   as u8) << 3)
             | ((self.interrupt as u8) << 2)
             | ((self.zero      as u8) << 1)
             | ((self.carry     as u8) << 0);
    }

    pub fn from_byte(value : u8) -> Self
    {
        return CPUFlags
        {
            negative:  (value & 0b10000000) >> 7 == 1,
            overflow:  (value & 0b01000000) >> 6 == 1,
            reserved:  (value & 0b00100000) >> 5 == 1,
            _break:    (value & 0b00010000) >> 4 == 1,
            decimal:   (value & 0b00001000) >> 3 == 1,
            interrupt: (value & 0b00000100) >> 2 == 1,
            zero:      (value & 0b00000010) >> 1 == 1,
            carry:     (value & 0b00000001) >> 0 == 1,
        };
    }
}

impl Clone for CPUFlags
{
    fn clone(&self) -> Self
    {
        let byte = self.to_byte();
        return CPUFlags::from_byte(byte);
    }
}

impl PartialEq<CPUFlags> for CPUFlags
{
    fn eq(&self, other : &CPUFlags) -> bool
    {
        return self.to_byte()==other.to_byte();
    }
}

impl Eq for CPUFlags {}
