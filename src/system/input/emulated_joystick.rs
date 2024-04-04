use std::collections::HashSet;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::system::byte;
use crate::system::input::JoystickKeycode;

pub struct Joystick
{
    is_strobe_mode_enabled : bool,
    current_index_for_reading : usize,
    was_at_least_one_key_read_in_strobe_mode : bool,
    order_of_keys_for_reading : Box<[JoystickKeycode]>,
    pressed_keys : HashSet<JoystickKeycode>,
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
            was_at_least_one_key_read_in_strobe_mode: false,
            order_of_keys_for_reading: sorted_keycodes.into_boxed_slice(),
            pressed_keys: HashSet::new(),
        };
    }

    pub fn on_key_down(self : &mut Joystick, keycode : JoystickKeycode)
    {
        self.pressed_keys.insert(keycode);
    }

    pub fn on_key_up(self : &mut Joystick, keycode : JoystickKeycode)
    {
        self.pressed_keys.remove(&keycode);
    }

    pub fn set_strobe_enabled(self : &mut Joystick, is_enabled : bool)
    {
        if self.is_strobe_mode_enabled ^ is_enabled
        {
            self.is_strobe_mode_enabled = is_enabled;

            if is_enabled
            {
                self.current_index_for_reading = self.order_of_keys_for_reading.len()-1;
                self.was_at_least_one_key_read_in_strobe_mode = false;
            }
            else if !self.was_at_least_one_key_read_in_strobe_mode
            {
                self.current_index_for_reading = 0;
            }
        }
    }

    pub fn read_pressed_key(self : &mut Joystick) -> byte
    {
        if self.is_strobe_mode_enabled
        {
            let is_key_pressed = self.pressed_keys.contains(&JoystickKeycode::A) as byte;

            self.current_index_for_reading = (self.current_index_for_reading+1) % self.order_of_keys_for_reading.len();
            self.was_at_least_one_key_read_in_strobe_mode = true;

            return is_key_pressed;
        }
        else
        {
            let keycode_to_read = &self.order_of_keys_for_reading[self.current_index_for_reading];
            let is_key_pressed = self.pressed_keys.contains(keycode_to_read) as byte;

            self.current_index_for_reading = (self.current_index_for_reading+1) % self.order_of_keys_for_reading.len();

            return is_key_pressed;
        }
    }
}
