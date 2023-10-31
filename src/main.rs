use anyhow::{Context, Result};
use std::{env, fs, panic, process};
use crate::system::{System, SystemStartArgs};

mod system;

fn main() -> Result<()>
{
    setup_panicking_from_all_threads();

    let args = env::args().collect::<Vec<String>>();
    if args.len()>=2 && args[1]=="test"
    {
        let test_name = args.get(2).cloned().unwrap_or_default();
        System::test().run_test(test_name).context(codeloc!())?;
    }
    else if args.len()>=2
    {
        let rom_bytes = fs::read(args[1].clone()).context(codeloc!())?.into_boxed_slice();
        let start_args = SystemStartArgs::with_rom_bytes(rom_bytes).context(codeloc!())?;
        System::start(start_args).join();
    }
    else
    {
        println!("Syntax: <emulator> <rom_file.nes>");
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
