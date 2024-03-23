use std::io::{Cursor, Read};
use anyhow::{Context, Result};
use zip::ZipArchive;
use crate::codeloc;
use crate::system::{byte, System, SystemStartArgs};

pub fn test_cpu_with_blargg_testrom() -> Result<()>
{
    let zip_archive_bytes = *include_bytes!("roms/cpu_blargg_test.zip");
    let mut zip_archive = ZipArchive::new(Cursor::new(zip_archive_bytes)).context(codeloc!())?;
    for i in 0..zip_archive.len()
    {
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
