mod g2d;
mod g3d;

extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use self::vulkano::instance::Instance;
use self::vulkano_win::VkSurfaceBuild;
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
    pub graphics_type: String,//make enum
    pub display_mode: DisplayMode,
    pub frame_id: u32,
    pub delta_time: time::Duration,
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
        let window = winit::WindowBuilder::new().with_title(title.to_string())
            .with_dimensions(width, height).build_vk_surface(&instance).unwrap();

        let queue = physical.queue_families().find(|q| {
            // We take the first queue that supports drawing to our window.
            q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
        }).expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                .. vulkano::device::DeviceExtensions::none()
            };

            vulkano::device::Device::new(
                &physical,
                 physical.supported_features(),
                  &device_ext,
                  [(queue, 0.5)].iter().cloned()
              ).expect("failed to create device")
        };

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = window.surface().get_capabilities(&physical)
                             .expect("failed to get surface capabilities");
            let dimensions = caps.current_extent.unwrap_or([width, height]);
            let present = caps.present_modes.iter().next().unwrap();
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            vulkano::swapchain::Swapchain::new(&device, &window.surface(), 2, format, dimensions, 1,
                           &caps.supported_usage_flags, &queue,
                           vulkano::swapchain::SurfaceTransform::Identity, alpha,
                           present, true, None).expect("failed to create swapchain")
        };

        let vertex_buffer = {
            #[derive(Debug, Clone)]
            struct Vertex { position: [f32; 2] }
            impl_vertex!(Vertex, position);

            vulkano::buffer::CpuAccessibleBuffer::from_iter(&device, &vulkano::buffer::BufferUsage::all(),
             Some(queue.family()), [
                Vertex { position: [-0.5, -0.25] },
                Vertex { position: [0.0, 0.5] },
                Vertex { position: [0.25, -0.1] }
            ].iter().cloned()).expect("failed to create buffer")
        };

        mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_vs.glsl")} }
        let vs = vs::Shader::load(&device).expect("failed to create shader module");
        mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_fs.glsl")} }
        let fs = fs::Shader::load(&device).expect("failed to create shader module");

        Graphics {
            graphics_type: "vulkano".to_string(),
            display_mode: DisplayMode {width: width, height: height, refresh_rate: 1, bits_per_pixel: 1},
            frame_id: 0,
            delta_time: time::Duration::seconds(1),
            fps: 0,
            monitors: vec![],
            window: window,
            should_close: false
        }
    }
}

#[derive(Debug)]
pub struct Geometry {
    mesh: Mesh
}

#[derive(Debug)]
pub struct Mesh {
    // verticies: ,
    // edges: ,
    // polygons:
}

// enum VertexDataType {
// 		VertexArray, VertexBufferObject, VertexBufferObjectSubData, VertexBufferObjectWithVAO
// }
