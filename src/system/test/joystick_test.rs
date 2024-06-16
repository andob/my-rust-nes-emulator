use anyhow::{Context, Result};
use crate::codeloc;
use crate::system::{System, SystemStartArgs};

pub fn test_joystick() -> Result<()>
{
    let rom_bytes = *include_bytes!("roms/joystick_test.nes");

    let start_args = SystemStartArgs::with_rom_bytes(Box::new(rom_bytes)).context(codeloc!())?;
    let running_system = System::start(start_args).context(codeloc!())?;
    running_system.await_termination();

    return Ok(());
}
