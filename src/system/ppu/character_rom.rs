use crate::system::{address, byte};
use crate::system::rom::Mapper;

pub struct CharacterROM
{
    _mapper: Mapper,
    _bytes: Box<[byte]>,
}

impl CharacterROM
{
    pub fn new(mapper : Mapper, bytes : &[byte]) -> CharacterROM
    {
        return CharacterROM { _mapper:mapper, _bytes: bytes.to_owned().into_boxed_slice() };
    }

    pub fn _len(&self) -> usize { self._bytes.len() }

    pub fn _get(&self, raw_address : address) -> byte
    {
        if self._mapper == Mapper::_0
        {
            let address = (raw_address as usize) % self._bytes.len();
            return self._bytes[address];
        }

        return 0;
    }
}
