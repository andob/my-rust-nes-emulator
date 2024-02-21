use anyhow::Context;
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_with_blocks_test() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_blocks_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
