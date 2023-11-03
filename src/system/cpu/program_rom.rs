use crate::system::{address, byte, mapper};

pub struct ProgramROM
{
    mapper : mapper,
    bytes : Box<[byte]>
}

impl ProgramROM
{
    pub fn new(mapper : mapper, bytes : &[byte]) -> ProgramROM
    {
        return ProgramROM { mapper:mapper, bytes: bytes.to_owned().into_boxed_slice() };
    }

    pub fn len(&self) -> usize { self.bytes.len() }

    pub fn get(&self, raw_address : address) -> byte
    {
        if self.mapper==0
        {
            let raw_offset = raw_address as usize;
            let rom_length = self.bytes.len();
            let offset = if raw_offset>=0xF && raw_offset < rom_length
                { (raw_offset+0x36AA)%rom_length } //todo check out what represents this magic value?
            else { raw_offset%rom_length };
            return self.bytes[offset];
        }

        return 0;
    }
}
