use std::io::{Cursor, Read};
use anyhow::{Result, Context};
use zip::ZipArchive;
use crate::codeloc;
use crate::system::{byte, System, SystemStartArgs};

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
    //todo make it pass
    let zip_archive_bytes = *include_bytes!("roms/ppu_sprite_zero_hit_test.zip");
    let mut zip_archive = ZipArchive::new(Cursor::new(zip_archive_bytes)).context(codeloc!())?;
    for i in 0..zip_archive.len()
    {
        if i!=2 { continue; }
        let mut rom_bytes: Vec<byte> = Vec::new();
        let mut zipped_file = zip_archive.by_index(i).context(codeloc!())?;
        zipped_file.read_to_end(&mut rom_bytes).context(codeloc!())?;

        let mut start_args = SystemStartArgs::with_rom_bytes(rom_bytes.into_boxed_slice()).context(codeloc!())?;
        start_args.should_disable_audio = true; //todo fix buggy audio interrupts
        start_args.window_title = format!("[{}] {}", i, zipped_file.name());

        let running_system = System::start(start_args).context(codeloc!())?;
        running_system.await_termination();
    }

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
