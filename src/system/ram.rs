
pub struct RAM
{
    bytes : Vec<u8>
}

impl RAM
{
    pub fn new() -> RAM
    {
        let ram_size = 2*1024; //2KB
        return RAM { bytes: vec![0; ram_size] };
    }

    pub fn get(self : &RAM, index : usize) -> Option<&u8>
    {
        return self.bytes.get(index);
    }

    pub fn put(self : &mut RAM, address : usize, value : u8)
    {
        if address < self.bytes.len()
        {
            self.bytes[address] = value;
        }
    }
}
