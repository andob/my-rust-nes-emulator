use anyhow::Context;
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_with_blocks_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_blocks_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

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

pub fn test_ppu_with_physics_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_physics_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_sprite_overflow_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_sprite_overflow_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_sprite_zero_hit_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_sprite_zero_hit_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_spritecans_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_spritecans_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_tutor_testrom() -> anyhow::Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_tutor_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
