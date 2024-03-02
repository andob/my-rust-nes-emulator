use anyhow::Context;
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_with_litewall_testrom() -> anyhow::Result<()>
{
    let rom3_bytes = *include_bytes!("roms/ppu_litewall3_test.nes");
    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom3_bytes)).context(codeloc!())?;
    let mut running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    let rom2_bytes = *include_bytes!("roms/ppu_litewall2_test.nes");
    start_args = SystemStartArgs::with_rom_bytes(Box::new(rom2_bytes)).context(codeloc!())?;
    running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    let rom1_bytes = *include_bytes!("roms/ppu_litewall1_test.nes");
    start_args = SystemStartArgs::with_rom_bytes(Box::new(rom1_bytes)).context(codeloc!())?;
    running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
