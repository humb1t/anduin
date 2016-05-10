extern crate glfw;

mod g2d;
mod g3d;

use self::glfw::{Context, WindowEvent};
use std::sync::mpsc::Receiver;

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
    pub graphics_type: String,//make enum
    pub display_mode: DisplayMode,
    pub frame_id: u32,
    pub delta_time: i64,
    pub fps: u16,
    pub window: glfw::Window,
    pub glfw: glfw::Glfw,
    pub events: Receiver<(f64, WindowEvent)>,
    pub monitors: Vec<Monitor>
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(width, height, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        Graphics {
            graphics_type: "GLFW".to_string(),
            display_mode: DisplayMode {width: width, height: height, refresh_rate: 1, bits_per_pixel: 1},
            frame_id: 0,
            delta_time: 2,
            fps: 0,
            monitors: vec![],
            glfw: glfw,
            window: window,
            events: events
        }
    }
}
