use anyhow::Result;
use std::{env, fs, panic, process};
use crate::system::System;

mod system;

fn main() -> Result<()>
{
    setup_panicking_from_all_threads();

    let args = env::args().collect::<Vec<String>>();
    if args.len()>=2 && args[1]=="test"
    {
        let test_name = args.get(2).cloned().unwrap_or_default();
        System::test().run_test(test_name)?;
    }
    else if args.len()>=2
    {
        let rom_bytes = fs::read(args[1].clone())?.into_boxed_slice();
        let mut nes = System::with_rom_bytes(rom_bytes)?;
        nes.run();
    }
    else
    {
        log_syntax!("Syntax: <emulator> <rom_file.nes>");
    }

    return Ok(());
}

pub fn setup_panicking_from_all_threads()
{
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info|
    {
        original_hook(panic_info);
        process::exit(1);
    }));
}
