use sdl2::keyboard::Keycode;
use crate::system::input::InputSubsystem;
use crate::system::ppu::PPURunEnvironment;

impl InputSubsystem
{
    pub fn handle_physical_keyboard_keydown(&mut self, keycode : Keycode, env : &mut PPURunEnvironment)
    {
        if let Some(joystick_keycode) = self.physical_keyboard_keymap.get(&keycode)
        {
            self.emulated_joystick.on_key_down(*joystick_keycode);
        }
        else
        {
            match keycode
            {
                Keycode::F5 => { env.debugger.should_render_background = false; }
                Keycode::F6 => { env.debugger.should_render_sprites = false; }
                Keycode::F7 => { env.debugger.should_debug_pattern_table = true; }
                _ => {}
            }
        }
    }

    pub fn handle_physical_keyboard_keyup(&mut self, keycode : Keycode, env : &mut PPURunEnvironment)
    {
        if let Some(joystick_keycode) = self.physical_keyboard_keymap.get(&keycode)
        {
            self.emulated_joystick.on_key_up(*joystick_keycode);
        }
        else
        {
            match keycode
            {
                Keycode::F5 => { env.debugger.should_render_background = true; }
                Keycode::F6 => { env.debugger.should_render_sprites = true; }
                Keycode::F7 => { env.debugger.should_debug_pattern_table = false; }
                _ => {}
            }
        }
    }
}
