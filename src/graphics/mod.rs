mod g2d;
mod g3d;

extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use self::vulkano::instance::Instance;
use self::vulkano_win::VkSurfaceBuild;

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
    pub window: vulkano_win::Window,
    pub monitors: Vec<Monitor>,
    pub should_close: bool
}

impl Graphics {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        println!("graphic new start");
        let instance = {
            let extensions = vulkano_win::required_extensions();
            println!("extensions: {:?}", extensions);
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };
        println!("instance created");
        let physical = vulkano::instance::PhysicalDevice::enumerate(&instance)
                            .next().expect("no device available");
        println!("Using device: {} (type: {:?})", physical.name(), physical.ty());
        let window = winit::WindowBuilder::new().build_vk_surface(&instance).unwrap();

        Graphics {
            graphics_type: "vulkano".to_string(),
            display_mode: DisplayMode {width: width, height: height, refresh_rate: 1, bits_per_pixel: 1},
            frame_id: 0,
            delta_time: 2,
            fps: 0,
            monitors: vec![],
            window: window,
            should_close: false
        }
    }
}
