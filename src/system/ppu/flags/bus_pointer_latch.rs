use crate::system::{address, address_from_high_low, byte};

pub struct PPUBusPointerLatch
{
    first_bus_pointer_write : byte,
    is_second_bus_pointer_write : bool,
    bus_pointer : address,
}

impl PPUBusPointerLatch
{
    pub fn new() -> PPUBusPointerLatch
    {
        return PPUBusPointerLatch
        {
            first_bus_pointer_write: 0,
            is_second_bus_pointer_write: false,
            bus_pointer: 0,
        };
    }

    pub fn wrapping_add(&self, offset : address) -> PPUBusPointerLatch
    {
        return PPUBusPointerLatch
        {
            first_bus_pointer_write: self.first_bus_pointer_write,
            is_second_bus_pointer_write: self.is_second_bus_pointer_write,
            bus_pointer: self.bus_pointer.wrapping_add(offset),
        };
    }

    pub fn as_byte(&self) -> byte { self.bus_pointer as byte }
    pub fn as_address(&self) -> address { self.bus_pointer as address }

    pub fn write(&mut self, value : byte)
    {
        if self.is_second_bus_pointer_write
        {
            self.bus_pointer = address_from_high_low(self.first_bus_pointer_write, value);
            self.is_second_bus_pointer_write = false;
        }
        else
        {
            self.first_bus_pointer_write = value;
            self.is_second_bus_pointer_write = true;
        }
    }
}
