use anyhow::{Result, Context};
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_ppu_with_blocks_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_blocks_test.nes");

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    start_args.should_disable_audio = true; //todo fix buggy audio interrupts
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_physics_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_physics_test.nes");

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    start_args.should_disable_audio = true; //todo fix buggy audio interrupts
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_sprite_overflow_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_sprite_overflow_test.nes");

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    start_args.should_disable_audio = true; //todo fix buggy audio interrupts
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_sprite_zero_hit_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_sprite_zero_hit_test.nes");

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    start_args.should_disable_audio = true; //todo fix buggy audio interrupts
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}

pub fn test_ppu_with_spritecans_testrom() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/ppu_spritecans_test.nes");

    let mut start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    start_args.should_disable_audio = true; //todo fix buggy audio interrupts
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
