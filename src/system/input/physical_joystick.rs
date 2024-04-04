use joydev::{DeviceEvent, GenericEvent};
use joydev::event_codes::{AbsoluteAxis, Key};
use crate::system::input::{InputSubsystem, JoystickKeycode};

impl InputSubsystem
{
    pub fn handle_physical_joystick_events(&mut self)
    {
        let emulated_joystick = &mut self.emulated_joystick;
        if let Some(physical_joystick) = &self.physical_joystick
        {
            while let Ok(event) = physical_joystick.get_event()
            {
                match event
                {
                    DeviceEvent::Axis(axis_event) =>
                    {
                        if axis_event.axis() == AbsoluteAxis::LeftY
                        {
                            if axis_event.value() < 0
                            {
                                emulated_joystick.on_key_down(JoystickKeycode::Up);
                                emulated_joystick.on_key_up(JoystickKeycode::Down);
                            }
                            else if axis_event.value() > 0
                            {
                                emulated_joystick.on_key_up(JoystickKeycode::Up);
                                emulated_joystick.on_key_down(JoystickKeycode::Down);
                            }
                            else
                            {
                                emulated_joystick.on_key_up(JoystickKeycode::Up);
                                emulated_joystick.on_key_up(JoystickKeycode::Down);
                            }
                        }
                        else if axis_event.axis() == AbsoluteAxis::LeftX
                        {
                            if axis_event.value() < 0
                            {
                                emulated_joystick.on_key_down(JoystickKeycode::Left);
                                emulated_joystick.on_key_up(JoystickKeycode::Right);
                            }
                            else if axis_event.value() > 0
                            {
                                emulated_joystick.on_key_up(JoystickKeycode::Left);
                                emulated_joystick.on_key_down(JoystickKeycode::Right);
                            }
                            else
                            {
                                emulated_joystick.on_key_up(JoystickKeycode::Left);
                                emulated_joystick.on_key_up(JoystickKeycode::Right);
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
                            emulated_joystick.on_key_up(button);
                        }
                        else
                        {
                            emulated_joystick.on_key_down(button);
                        }
                    }
                }
            }
        }
    }
}
