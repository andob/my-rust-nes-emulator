use crate::system::{address, byte};
use crate::system::rom::Mapper;

pub struct ProgramROM
{
    mapper : Mapper,
    bytes : Box<[byte]>
}

impl ProgramROM
{
    pub fn new(mapper : Mapper, bytes : &[byte]) -> ProgramROM
    {
        return ProgramROM { mapper:mapper, bytes: bytes.to_owned().into_boxed_slice() };
    }

    pub fn len(&self) -> usize { self.bytes.len() }

    pub fn get(&self, raw_address : address) -> byte
    {
        if self.mapper == Mapper::_0
        {
            let raw_offset = raw_address as usize;
            let rom_length = self.bytes.len();
            let offset = if raw_offset>=0xF && raw_offset < rom_length
                { (raw_offset+0x36AA)%rom_length } //todo check out what represents this magic value?
            else { raw_offset%rom_length };
            return self.bytes[offset];
        }

        if self.mapper == Mapper::SnakeTestGame
        {
            let address = if raw_address >= 0x0600
                { ((raw_address as usize)-0x0600)%self.bytes.len() }
            else { (raw_address as usize)%self.bytes.len() };
            return self.bytes[address];
        }

        return 0;
    }
}
