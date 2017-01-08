pub mod g2d;
pub mod g3d;

use time;

pub trait Drawable {
    fn draw(&self);
}

pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u16,
    pub bits_per_pixel: u16
}

pub struct Monitor {
    pub virtual_y: u32,
    pub virtual_x: u32,
    pub name: String,
    pub display_modes: Vec<DisplayMode>
}

pub struct Graphics {
    pub display_mode: DisplayMode,
    pub frame_id: u32,
    pub delta_time: time::Duration,
    pub fps: u16,
    pub monitors: Vec<Monitor>,
    pub should_close: bool,
    pub is3d: bool
}

impl Graphics {
    pub fn new(width: u32, height: u32, title: &str, is3d: bool) -> Self {
        Graphics {
            display_mode: DisplayMode {width: width, height: height, refresh_rate: 1, bits_per_pixel: 1},
            frame_id: 0,
            delta_time: time::Duration::seconds(1),
            fps: 0,
            monitors: vec![],
            should_close: false,
            is3d: is3d
        }
    }
}

