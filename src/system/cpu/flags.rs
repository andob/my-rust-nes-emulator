use std::fmt::{Display, Formatter};
use crate::system::byte;

pub struct CPUFlags
{
    pub negative : bool,
    pub overflow : bool,
    pub reserved : bool,
    pub _break : bool, //todo how is this used?
    pub decimal : bool,
    pub interrupt : bool, //todo how is this used?
    pub zero : bool,
    pub carry : bool,
}

impl CPUFlags
{
    pub fn to_byte(self : &CPUFlags) -> byte
    {
        return ((self.negative  as byte) << 7)
             | ((self.overflow  as byte) << 6)
             | ((self.reserved  as byte) << 5)
             | ((self._break    as byte) << 4)
             | ((self.decimal   as byte) << 3)
             | ((self.interrupt as byte) << 2)
             | ((self.zero      as byte) << 1)
             | ((self.carry     as byte) << 0);
    }

    pub fn from_byte(value : byte) -> CPUFlags
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

    pub fn clone_from_byte(&self, value : byte) -> CPUFlags
    {
        return CPUFlags::from_byte(value);
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
        return self.negative == other.negative &&
            self.overflow == other.overflow &&
            self._break == other._break &&
            self.interrupt == other.interrupt &&
            self.zero == other.zero &&
            self.carry == other.carry;
    }
}

impl Eq for CPUFlags {}

impl Display for CPUFlags
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "negative:{} overflow:{} break:{} interrupt:{} zero:{} carry:{}\n",
            self.negative, self.overflow, self._break, self.interrupt, self.zero, self.carry)
    }
}
