use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_with_sprite_zero_hit_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_sprite_zero_hit_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
