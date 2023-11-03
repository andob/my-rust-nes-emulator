use std::fs;
use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_show_pattern_table(args : Vec<String>) -> Result<()>
{
    let rom_file_path = args.last().cloned().unwrap_or_default();
    let rom_bytes = fs::read(rom_file_path).context(codeloc!())?.into_boxed_slice();
    let mut start_args = SystemStartArgs::with_rom_bytes(rom_bytes).context(codeloc!())?;
    start_args.ppu_debugger.should_render_pattern_tables_for_debugging = true;
    System::start(start_args).context(codeloc!())?.await_termination();
    return Ok(());
}
