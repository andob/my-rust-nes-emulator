use std::collections::HashSet;
use crate::system::{address, byte, mapper};

pub struct ProgramROM
{
    mapper : mapper,
    mappings : HashSet<SimpleMapping>,
    bytes : Box<[byte]>,
    pub program_start_address : address,
    pub program_end_address : address,
}

type large_address = usize;

#[derive(Eq, PartialEq, Hash)]
struct SimpleMapping
{
    source_start_address : address,
    destination_start_address : large_address,
    length : address,
}

impl ProgramROM
{
    pub fn new(mapper : mapper, bytes : &[byte]) -> ProgramROM
    {
        let mut rom = ProgramROM
        {
            mapper: mapper, mappings: HashSet::new(),
            bytes: bytes.to_owned().into_boxed_slice(),
            program_start_address: 0x8000,
            program_end_address: 0xFFFF,
        };

        if mapper==31
        {
            rom.set(0x5FFF, 0xFF);
        }

        return rom;
    }

    pub fn len(&self) -> usize { self.bytes.len() }

    pub fn get(&self, raw_address : address) -> byte
    {
        if raw_address>=self.program_start_address && raw_address<=self.program_end_address
        {
            for mapping in &self.mappings
            {
                if mapping.source_start_address<=raw_address && raw_address<=mapping.source_start_address-1+mapping.length
                {
                    let mapped_address = ((raw_address - mapping.source_start_address) as large_address) + mapping.destination_start_address;
                    return self.bytes[mapped_address % self.bytes.len()];
                }
            }

            let mapped_address = raw_address - self.program_start_address;
            return self.bytes[(mapped_address as usize) % self.bytes.len()];
        }

        return 0;
    }

    pub fn set(&mut self, raw_address : address, value : byte)
    {
        if self.mapper==31 && raw_address>=0x5FF8 && raw_address<=0x5FFF
        {
            let bank_size = 4*1024 as address;
            let number_of_banks_in_source = 8 as address;
            let number_of_banks_in_destination = self.bytes.len() / (bank_size as large_address);
            let bank_index = (value as large_address) % number_of_banks_in_destination;
            let destination_start_address = (bank_size as large_address) * bank_index;
            let source_start_address = self.program_start_address + ((raw_address & 0xF) % number_of_banks_in_source) * bank_size;
            let mapping = SimpleMapping { source_start_address, destination_start_address, length:bank_size };
            self.mappings.insert(mapping);
        }
    }
}
