mod g2d;
mod g3d;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;


use std::sync::Arc;
use std::time::Duration;
use time;

pub trait Drawable {
    fn draw(&self);
}

pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u16,
    pub bits_per_pixel: u16,
}

pub struct Monitor {
    pub virtual_y: u32,
    pub virtual_x: u32,
    pub name: String,
    pub display_modes: Vec<DisplayMode>,
}

pub struct Graphics {
    pub graphics_type: String, // make enum
    pub display_mode: DisplayMode,
    pub frame_id: u32,
    pub delta_time: time::Duration,
    pub fps: u16,
    pub window: glutin::Window,
    pub monitors: Vec<Monitor>,
    pub should_close: bool,
    pub graphics3d: g3d::Graphics3d,
}

impl Graphics {
    pub fn new(width: u32, height: u32, title: &str) -> Self {

        let window = build_window(width, height, title);


        Graphics {
            graphics_type: "vulkano".to_string(),
            display_mode: DisplayMode {
                width: width,
                height: height,
                refresh_rate: 1,
                bits_per_pixel: 1,
            },
            frame_id: 0,
            delta_time: time::Duration::seconds(1),
            fps: 0,
            graphics3d: g3d::Graphics3d::init(width, height),
            monitors: vec![],
            window: window,
            should_close: false,
        }
    }
}

fn build_window(width: u32, height: u32, title: &str) -> glutin::Window {
    let builder = glutin::WindowBuilder::new()
            .with_title("Triangle example".to_string())
            .with_dimensions(1024, 768)
            .with_vsync();
        let (window, mut device, mut factory, main_color, _main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
        let pso = factory.create_pipeline_simple(
            include_bytes!("/glsl/triangle_150.glslv"),
            include_bytes!("/glsl/triangle_150.glslf"),
            pipe::new()
        ).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color
        };

        'main: loop {
            // loop over events
            for event in window.poll_events() {
                match event {
                    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                    glutin::Event::Closed => break 'main,
                    _ => {},
                }
            }
            // draw a frame
            encoder.clear(&data.out, CLEAR_COLOR);
            encoder.draw(&slice, &pso, &data);
            encoder.flush(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        }
    return window;
}

#[derive(Debug)]
pub struct Geometry {
    mesh: Mesh,
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
