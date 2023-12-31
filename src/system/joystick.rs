use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use maplit2::hashmap;
use sdl2::keyboard::Keycode;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::system::byte;

pub struct Joystick
{
    is_strobe_mode_enabled : bool,
    current_index_for_reading : usize,
    order_of_keys_for_reading : Box<[JoystickKeycode]>,
    pressed_keys : HashSet<JoystickKeycode>,
    keymap : HashMap<Keycode, JoystickKeycode>,
}

#[derive(Eq, PartialEq, Hash, Clone, EnumIter)]
enum JoystickKeycode
{
    A, B, Select, Start, Up, Down, Left, Right
}

impl JoystickKeycode
{
    pub fn to_byte(&self) -> byte
    {
        return match self
        {
            JoystickKeycode::A      => 0b00000001,
            JoystickKeycode::B      => 0b00000010,
            JoystickKeycode::Select => 0b00000100,
            JoystickKeycode::Start  => 0b00001000,
            JoystickKeycode::Up     => 0b00010000,
            JoystickKeycode::Down   => 0b00100000,
            JoystickKeycode::Left   => 0b01000000,
            JoystickKeycode::Right  => 0b10000000,
        }
    }
}

impl Joystick
{
    pub fn new() -> Joystick
    {
        let sorted_keycodes : Vec<JoystickKeycode> = JoystickKeycode::iter()
            .sorted_by_key(|keycode| keycode.to_byte()).collect();

        return Joystick
        {
            is_strobe_mode_enabled: false,
            current_index_for_reading: 0,
            order_of_keys_for_reading: sorted_keycodes.into_boxed_slice(),
            pressed_keys: HashSet::new(),
            keymap: hashmap!
            {
                Keycode::A         => JoystickKeycode::A,
                Keycode::S         => JoystickKeycode::B,
                Keycode::Backslash => JoystickKeycode::Select,
                Keycode::Return    => JoystickKeycode::Start,
                Keycode::Up        => JoystickKeycode::Up,
                Keycode::Down      => JoystickKeycode::Down,
                Keycode::Left      => JoystickKeycode::Left,
                Keycode::Right     => JoystickKeycode::Right,
            },
        };
    }

    pub fn on_key_down(self : &mut Joystick, keycode : Keycode)
    {
        if let Some(joystick_keycode) = self.keymap.get(&keycode)
        {
            self.pressed_keys.insert(joystick_keycode.clone());
        }
    }

    pub fn on_key_up(self : &mut Joystick, keycode : Keycode)
    {
        if let Some(joystick_keycode) = self.keymap.get(&keycode)
        {
            self.pressed_keys.remove(joystick_keycode);
        }
    }

    pub fn set_strobe_enabled(self : &mut Joystick, is_enabled : bool)
    {
        if self.is_strobe_mode_enabled ^ is_enabled
        {
            self.is_strobe_mode_enabled = is_enabled;
            self.current_index_for_reading = 0;
        }
    }

    pub fn read_pressed_key(self : &mut Joystick) -> byte
    {
        let keycode_to_read = &self.order_of_keys_for_reading[self.current_index_for_reading];

        self.current_index_for_reading = if self.is_strobe_mode_enabled { self.current_index_for_reading }
        else { (self.current_index_for_reading+1) % self.order_of_keys_for_reading.len() };

        return self.pressed_keys.contains(keycode_to_read) as byte;
    }
}
