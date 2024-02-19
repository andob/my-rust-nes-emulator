use crate::system::address;

pub const NES_DISPLAY_WIDTH : address = 256;
pub const NES_DISPLAY_HEIGHT : address = 240;

pub struct WindowMetrics
{
    scale_x : f32,
    scale_y : f32,
}

impl WindowMetrics
{
    pub fn new() -> WindowMetrics
    {
        return WindowMetrics { scale_x:3.0, scale_y:3.0 };
    }

    pub fn get_scale(&self) -> (f32, f32) { (self.scale_x, self.scale_y) }
    pub fn get_scale_x(&self) -> f32 { self.scale_x }
    pub fn get_scale_y(&self) -> f32 { self.scale_y }

    pub fn get_window_width(&self) -> u32 { ((NES_DISPLAY_WIDTH as f32) * self.scale_x) as u32 }
    pub fn get_window_height(&self) -> u32 { ((NES_DISPLAY_HEIGHT as f32) * self.scale_y) as u32 }

    pub fn on_window_resized(&mut self, width : i32, height : i32)
    {
        self.scale_x = (width as f32) / (NES_DISPLAY_WIDTH as f32);
        self.scale_y = (height as f32) / (NES_DISPLAY_HEIGHT as f32);
    }
}
