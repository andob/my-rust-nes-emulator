use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use joydev::{DeviceEvent, GenericEvent};
use joydev::event_codes::{AbsoluteAxis, Key};
use maplit2::hashmap;
use sdl2::keyboard::Keycode;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use crate::system::byte;

pub struct Joystick
{
    is_strobe_mode_enabled : bool,
    current_index_for_reading : usize,
    was_at_least_one_key_read_in_strobe_mode : bool,
    order_of_keys_for_reading : Box<[JoystickKeycode]>,
    pressed_keys : HashSet<JoystickKeycode>,
    keyboard_keymap : HashMap<Keycode, JoystickKeycode>,
}

#[derive(Eq, PartialEq, Hash, Clone, Display, EnumIter)]
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
            was_at_least_one_key_read_in_strobe_mode: false,
            order_of_keys_for_reading: sorted_keycodes.into_boxed_slice(),
            pressed_keys: HashSet::new(),
            keyboard_keymap: hashmap!
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

    pub fn on_keyboard_key_down(self : &mut Joystick, keycode : Keycode)
    {
        if let Some(joystick_keycode) = self.keyboard_keymap.get(&keycode)
        {
            self.pressed_keys.insert(joystick_keycode.clone());
        }
    }

    pub fn on_keyboard_key_up(self : &mut Joystick, keycode : Keycode)
    {
        if let Some(joystick_keycode) = self.keyboard_keymap.get(&keycode)
        {
            self.pressed_keys.remove(joystick_keycode);
        }
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

    pub fn on_physical_joystick_event(self : &mut Joystick, event : &DeviceEvent)
    {
        match event
        {
            DeviceEvent::Axis(axis_event) =>
            {
                if axis_event.axis() == AbsoluteAxis::LeftY
                {
                    if axis_event.value() < 0
                    {
                        self.pressed_keys.insert(JoystickKeycode::Up);
                        self.pressed_keys.remove(&JoystickKeycode::Down);
                    }
                    else if axis_event.value() > 0
                    {
                        self.pressed_keys.remove(&JoystickKeycode::Up);
                        self.pressed_keys.insert(JoystickKeycode::Down);
                    }
                    else
                    {
                        self.pressed_keys.remove(&JoystickKeycode::Up);
                        self.pressed_keys.remove(&JoystickKeycode::Down);
                    }
                }
                else if axis_event.axis() == AbsoluteAxis::LeftX
                {
                    if axis_event.value() < 0
                    {
                        self.pressed_keys.insert(JoystickKeycode::Left);
                        self.pressed_keys.remove(&JoystickKeycode::Right);
                    }
                    else if axis_event.value() > 0
                    {
                        self.pressed_keys.remove(&JoystickKeycode::Left);
                        self.pressed_keys.insert(JoystickKeycode::Right);
                    }
                    else
                    {
                        self.pressed_keys.remove(&JoystickKeycode::Left);
                        self.pressed_keys.remove(&JoystickKeycode::Right);
                    }
                }
            }

            DeviceEvent::Button(button_event) =>
            {
                let button = match button_event.button()
                {
                    Key::ButtonThumb => { JoystickKeycode::A }
                    Key::ButtonTrigger => { JoystickKeycode::B }
                    Key::ButtonBase3 => { JoystickKeycode::Select }
                    Key::ButtonBase4 => { JoystickKeycode::Start }
                    _ => { return }
                };

                if button_event.value()==0
                {
                    self.pressed_keys.remove(&button);
                }
                else
                {
                    self.pressed_keys.insert(button);
                }
            }
        }
    }
}
