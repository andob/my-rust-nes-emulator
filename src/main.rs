use std::{env, fs};
use crate::system::System;

mod system;

fn main()
{
    let args = env::args().collect::<Vec<String>>();
    if args.len()>=3 && args[1]=="test"
    {
        return System::test().run_test(&args[2]).unwrap();
    }
    else if args.len()>=2
    {
        let rom_bytes = fs::read(args[1].clone()).unwrap().into_boxed_slice();
        let mut nes = System::new(rom_bytes);
        nes.run();
    }
    else
    {
        println!("Syntax: <emulator> <rom_file.nes>");
    }
}
