use std::collections::HashMap;
use joydev::Device;
use maplit2::hashmap;
use sdl2::keyboard::Keycode;
use strum_macros::{Display, EnumIter};
use crate::system::byte;
use crate::system::input::emulated_joystick::Joystick;

pub mod emulated_joystick;
pub mod physical_joystick;
pub mod physical_keyboard;

pub struct InputSubsystem
{
    emulated_joystick : Joystick,
    physical_joystick : Option<Device>,
    physical_keyboard_keymap : HashMap<Keycode, JoystickKeycode>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Display, EnumIter)]
pub enum JoystickKeycode
{
    A, B, Select, Start, Up, Down, Left, Right
}

impl InputSubsystem
{
    pub fn new() -> InputSubsystem
    {
        let physical_joystick = Device::open("/dev/input/js0").ok();

        return InputSubsystem
        {
            emulated_joystick: Joystick::new(),
            physical_joystick: physical_joystick,
            physical_keyboard_keymap: hashmap!
            {
                Keycode::K      => JoystickKeycode::A,
                Keycode::L      => JoystickKeycode::B,
                Keycode::Space  => JoystickKeycode::Select,
                Keycode::Return => JoystickKeycode::Start,
                Keycode::W      => JoystickKeycode::Up,
                Keycode::S      => JoystickKeycode::Down,
                Keycode::A      => JoystickKeycode::Left,
                Keycode::D      => JoystickKeycode::Right,
            },
        };
    }

    pub fn set_strobe_enabled(self : &mut InputSubsystem, is_enabled : bool)
    {
        self.emulated_joystick.set_strobe_enabled(is_enabled);
    }

    pub fn get_pressed_key(self : &mut InputSubsystem) -> byte
    {
        return self.emulated_joystick.read_pressed_key();
    }
}
