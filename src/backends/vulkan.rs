extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use logic::{Application, ApplicationListener, ApplicationAdapter};
use input::{Key,InputEvent, Input, InputType};
use graphics;
use std::sync::Arc;
use std;
use vulkano::command_buffer;
use vulkano::command_buffer::DynamicState;
use vulkano::command_buffer::PrimaryCommandBufferBuilder;
use vulkano::command_buffer::Submission;
use vulkano::descriptor::pipeline_layout::EmptyPipeline;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineParams;
use vulkano::pipeline::blend::Blend;
use vulkano::pipeline::depth_stencil::DepthStencil;
use vulkano::pipeline::input_assembly::InputAssembly;
use vulkano::pipeline::multisample::Multisample;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::viewport::ViewportsState;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::viewport::Scissor;
use vulkano::instance::Instance;
use self::vulkano_win::VkSurfaceBuild;
use std::time::Duration;

pub struct VulkanApplication {
    pub application: Application,
    pub window: vulkano_win::Window
}

impl VulkanApplication {
    pub fn init(name: &'static str, title: &'static str, lifetime: Option<u64>, listener: Box<ApplicationListener>) -> Self {
        let width = 1024;
        let height = 768;
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
        let result = VulkanApplication {
            application: Application {
                listener: listener,
                name: name,
                platform: "vulkano",
                graphics: graphics::Graphics::new(width, height, title),
                input: Input::new(),
                lifetime: lifetime,
            },
            window: window
        };
        result.init_graphic(&physical, width,  height, title);
        result.application.listener.as_ref().init();
        result
    }

    fn init_graphic(&self, physical: &vulkano::instance::PhysicalDevice, width: u32, height: u32, title: &str) {
        println!("graphic new start");
        let queue = physical.queue_families()
            .find(|q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics() && self.window.surface().is_supported(q).unwrap_or(false)
            })
            .expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                ..vulkano::device::DeviceExtensions::none()
            };

            vulkano::device::Device::new(&physical,
                                         physical.supported_features(),
                                         &device_ext,
                                         [(queue, 0.5)].iter().cloned())
                .expect("failed to create device")
        };

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let caps = self.window.surface()
                .get_capabilities(&physical)
                .expect("failed to get surface capabilities");
            let dimensions = caps.current_extent.unwrap_or([width, height]);
            let present = caps.present_modes.iter().next().unwrap();
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            vulkano::swapchain::Swapchain::new(&device,
                                               &self.window.surface(),
                                               2,
                                               format,
                                               dimensions,
                                               1,
                                               &caps.supported_usage_flags,
                                               &queue,
                                               vulkano::swapchain::SurfaceTransform::Identity,
                                               alpha,
                                               present,
                                               true,
                                               None)
                .expect("failed to create swapchain")
        };
        let vertex_buffer =
        {
            #[derive(Debug, Clone)]
            struct Vertex {
                position: [f32; 2],
            }
            impl_vertex!(Vertex, position);

            vulkano::buffer::CpuAccessibleBuffer::from_iter(&device, &vulkano::buffer::BufferUsage::all(),
                                                            Some(queue.family()), [
                                                                Vertex { position: [-0.5, -0.25] },
                                                                Vertex { position: [0.0, 0.5] },
                                                                Vertex { position: [0.25, -0.1] }
                                                            ].iter().cloned()).expect("failed to create buffer")
        };

        mod vs {
            include! {concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_vs.glsl")}
        }
        let vs = vs::Shader::load(&device).expect("failed to create shader module");
        mod fs {
            include! {concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_fs.glsl")}
        }
        let fs = fs::Shader::load(&device).expect("failed to create shader module");

        mod render_pass {
            use vulkano::format::Format;
            single_pass_renderpass! {
        attachments: {
        color: {load: Clear, store: Store, format: Format,}
        },
        pass: {
        color: [color], depth_stencil: {}
        }
        }
        }

        let render_pass = render_pass::CustomRenderPass::new(&device,
                                                             &render_pass::Formats {
                                                                 color: (images[0].format(), 1),
                                                             })
            .unwrap();

        let pipeline =
        GraphicsPipeline::new(&device,
                              GraphicsPipelineParams {
                                  vertex_input: SingleBufferDefinition::new(),
                                  vertex_shader: vs.main_entry_point(),
                                  input_assembly: InputAssembly::triangle_list(),
                                  tessellation: None,
                                  geometry_shader: None,
                                  viewport: ViewportsState::Fixed {
                                      data: vec![(Viewport {
        origin: [0.0, 0.0],
        depth_range: 0.0..1.0,
        dimensions:
        [images[0].dimensions()[0] as f32,
        images[0].dimensions()[1] as f32],
        },
        Scissor::irrelevant())],
                                  },
                                  raster: Default::default(),
                                  multisample: Multisample::disabled(),
                                  fragment_shader: fs.main_entry_point(),
                                  depth_stencil: DepthStencil::disabled(),
                                  blend: Blend::pass_through(),
                                  layout: &EmptyPipeline::new(&device).unwrap(),
                                  render_pass: Subpass::from(&render_pass, 0).unwrap(),
                              })
            .unwrap();

        let framebuffers = images.iter()
            .map(|image| {
                let dimensions = [image.dimensions()[0], image.dimensions()[1], 1];
                Framebuffer::new(&render_pass,
                                 dimensions,
                                 render_pass::AList { color: image })
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let mut submissions: Vec<Arc<Submission>> = Vec::new();
        submissions.retain(|s| s.destroying_would_block());
        let image_num = swapchain.acquire_next_image(Duration::new(1, 0)).unwrap();
        let command_buffer = PrimaryCommandBufferBuilder::new(&device, queue.family())
            .draw_inline(&render_pass,
                         &framebuffers[image_num],
                         render_pass::ClearValues { color: [0.0, 0.0, 1.0, 1.0] })
            .draw(&pipeline, &vertex_buffer, &DynamicState::none(), (), &())
            .draw_end()
            .build();
        submissions.push(command_buffer::submit(&command_buffer, &queue).unwrap());
        swapchain.present(&queue, image_num).unwrap();
    }

    fn transform_event(&self, event: &winit::Event) -> InputEvent {
        InputEvent {
            event_type: InputType::KeyDown,
            key: Key::A,
        }
    }
}

impl ApplicationAdapter for VulkanApplication {

    fn get_application(&mut self) -> &mut Application{
        &mut self.application
    }

    fn process_input(&mut self) {
        for event in self.window.window().poll_events() {
            let transformed_event: InputEvent = self.transform_event(&event);
            &self.application.input.events_queue.push_back(transformed_event);
        }
        &self.application.input.handle();
    }

    fn update(&mut self) {
        self.application.listener.as_mut().update();
    }
}


impl InputTranslate for winit::VirtualKeyCode {
    fn translate(&self) -> Key {
        match *self {
            winit::VirtualKeyCode::Key1 => Key::Key1,
            winit::VirtualKeyCode::Key2 => Key::Key2,
            winit::VirtualKeyCode::Key3 => Key::Key3,
            winit::VirtualKeyCode::Key4 => Key::Key4,
            winit::VirtualKeyCode::Key5 => Key::Key5,
            winit::VirtualKeyCode::Key6 => Key::Key6,
            winit::VirtualKeyCode::Key7 => Key::Key7,
            winit::VirtualKeyCode::Key8 => Key::Key8,
            winit::VirtualKeyCode::Key9 => Key::Key9,
            winit::VirtualKeyCode::Key0 => Key::Key0,

            winit::VirtualKeyCode::A => Key::A,
            winit::VirtualKeyCode::B => Key::B,
            winit::VirtualKeyCode::C => Key::C,
            winit::VirtualKeyCode::D => Key::D,
            winit::VirtualKeyCode::E => Key::E,
            winit::VirtualKeyCode::F => Key::F,
            winit::VirtualKeyCode::G => Key::G,
            winit::VirtualKeyCode::H => Key::H,
            winit::VirtualKeyCode::I => Key::I,
            winit::VirtualKeyCode::J => Key::J,
            winit::VirtualKeyCode::K => Key::K,
            winit::VirtualKeyCode::L => Key::L,
            winit::VirtualKeyCode::M => Key::M,
            winit::VirtualKeyCode::N => Key::N,
            winit::VirtualKeyCode::O => Key::O,
            winit::VirtualKeyCode::P => Key::P,
            winit::VirtualKeyCode::Q => Key::Q,
            winit::VirtualKeyCode::R => Key::R,
            winit::VirtualKeyCode::S => Key::S,
            winit::VirtualKeyCode::T => Key::T,
            winit::VirtualKeyCode::U => Key::U,
            winit::VirtualKeyCode::V => Key::V,
            winit::VirtualKeyCode::W => Key::W,
            winit::VirtualKeyCode::X => Key::X,
            winit::VirtualKeyCode::Y => Key::Y,
            winit::VirtualKeyCode::Z => Key::Z,
            winit::VirtualKeyCode::Escape => Key::Escape,
            winit::VirtualKeyCode::F1 => Key::F1,
            winit::VirtualKeyCode::F2 => Key::F2,
            winit::VirtualKeyCode::F3 => Key::F3,
            winit::VirtualKeyCode::F4 => Key::F4,
            winit::VirtualKeyCode::F5 => Key::F5,
            winit::VirtualKeyCode::F6 => Key::F6,
            winit::VirtualKeyCode::F7 => Key::F7,
            winit::VirtualKeyCode::F8 => Key::F8,
            winit::VirtualKeyCode::F9 => Key::F9,
            winit::VirtualKeyCode::F10 => Key::F10,
            winit::VirtualKeyCode::F11 => Key::F11,
            winit::VirtualKeyCode::F12 => Key::F12,
            winit::VirtualKeyCode::F13 => Key::F13,
            winit::VirtualKeyCode::F14 => Key::F14,
            winit::VirtualKeyCode::F15 => Key::F15,
            winit::VirtualKeyCode::Snapshot => Key::Snapshot,
            winit::VirtualKeyCode::Scroll => Key::Scroll,
            winit::VirtualKeyCode::Pause => Key::Pause,
            winit::VirtualKeyCode::Insert => Key::Insert,
            winit::VirtualKeyCode::Home => Key::Home,
            winit::VirtualKeyCode::Delete => Key::Delete,
            winit::VirtualKeyCode::End => Key::End,
            winit::VirtualKeyCode::PageDown => Key::PageDown,
            winit::VirtualKeyCode::PageUp => Key::PageUp,
            winit::VirtualKeyCode::Left => Key::Left,
            winit::VirtualKeyCode::Up => Key::Up,
            winit::VirtualKeyCode::Right => Key::Right,
            winit::VirtualKeyCode::Down => Key::Down,
            winit::VirtualKeyCode::Back => Key::Back,
            winit::VirtualKeyCode::Return => Key::Return,
            winit::VirtualKeyCode::Space => Key::Space,
            winit::VirtualKeyCode::Numlock => Key::Numlock,
            winit::VirtualKeyCode::Numpad0 => Key::Numpad0,
            winit::VirtualKeyCode::Numpad1 => Key::Numpad1,
            winit::VirtualKeyCode::Numpad2 => Key::Numpad2,
            winit::VirtualKeyCode::Numpad3 => Key::Numpad3,
            winit::VirtualKeyCode::Numpad4 => Key::Numpad4,
            winit::VirtualKeyCode::Numpad5 => Key::Numpad5,
            winit::VirtualKeyCode::Numpad6 => Key::Numpad6,
            winit::VirtualKeyCode::Numpad7 => Key::Numpad7,
            winit::VirtualKeyCode::Numpad8 => Key::Numpad8,
            winit::VirtualKeyCode::Numpad9 => Key::Numpad9,
            winit::VirtualKeyCode::AbntC1 => Key::AbntC1,
            winit::VirtualKeyCode::AbntC2 => Key::AbntC2,
            winit::VirtualKeyCode::Add => Key::Add,
            winit::VirtualKeyCode::Apostrophe => Key::Apostrophe,
            winit::VirtualKeyCode::Apps => Key::Apps,
            winit::VirtualKeyCode::At => Key::At,
            winit::VirtualKeyCode::Ax => Key::Ax,
            winit::VirtualKeyCode::Backslash => Key::Backslash,
            winit::VirtualKeyCode::Calculator => Key::Calculator,
            winit::VirtualKeyCode::Capital => Key::Capital,
            winit::VirtualKeyCode::Colon => Key::Colon,
            winit::VirtualKeyCode::Comma => Key::Comma,
            winit::VirtualKeyCode::Convert => Key::Convert,
            winit::VirtualKeyCode::Decimal => Key::Decimal,
            winit::VirtualKeyCode::Divide => Key::Divide,
            winit::VirtualKeyCode::Equals => Key::Equals,
            winit::VirtualKeyCode::Grave => Key::Grave,
            winit::VirtualKeyCode::Kana => Key::Kana,
            winit::VirtualKeyCode::Kanji => Key::Kanji,
            winit::VirtualKeyCode::LAlt => Key::LAlt,
            winit::VirtualKeyCode::LBracket => Key::LBracket,
            winit::VirtualKeyCode::LControl => Key::LControl,
            winit::VirtualKeyCode::LMenu => Key::LMenu,
            winit::VirtualKeyCode::LShift => Key::LShift,
            winit::VirtualKeyCode::LWin => Key::LWin,
            winit::VirtualKeyCode::Mail => Key::Mail,
            winit::VirtualKeyCode::MediaSelect => Key::MediaSelect,
            winit::VirtualKeyCode::MediaStop => Key::MediaStop,
            winit::VirtualKeyCode::Minus => Key::Minus,
            winit::VirtualKeyCode::Multiply => Key::Multiply,
            winit::VirtualKeyCode::Mute => Key::Mute,
            winit::VirtualKeyCode::MyComputer => Key::MyComputer,
            winit::VirtualKeyCode::NavigateForward => Key::NavigateForward,
            winit::VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
            winit::VirtualKeyCode::NextTrack => Key::NextTrack,
            winit::VirtualKeyCode::NoConvert => Key::NoConvert,
            winit::VirtualKeyCode::NumpadComma => Key::NumpadComma,
            winit::VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            winit::VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            winit::VirtualKeyCode::OEM102 => Key::OEM102,
            winit::VirtualKeyCode::Period => Key::Period,
            winit::VirtualKeyCode::PlayPause => Key::PlayPause,
            winit::VirtualKeyCode::Power => Key::Power,
            winit::VirtualKeyCode::PrevTrack => Key::PrevTrack,
            winit::VirtualKeyCode::RAlt => Key::RAlt,
            winit::VirtualKeyCode::RBracket => Key::RBracket,
            winit::VirtualKeyCode::RControl => Key::RControl,
            winit::VirtualKeyCode::RMenu => Key::RMenu,
            winit::VirtualKeyCode::RShift => Key::RShift,
            winit::VirtualKeyCode::RWin => Key::RWin,
            winit::VirtualKeyCode::Semicolon => Key::Semicolon,
            winit::VirtualKeyCode::Slash => Key::Slash,
            winit::VirtualKeyCode::Sleep => Key::Sleep,
            winit::VirtualKeyCode::Stop => Key::Stop,
            winit::VirtualKeyCode::Subtract => Key::Subtract,
            winit::VirtualKeyCode::Sysrq => Key::Sysrq,
            winit::VirtualKeyCode::Tab => Key::Tab,
            winit::VirtualKeyCode::Underline => Key::Underline,
            winit::VirtualKeyCode::Unlabeled => Key::Unlabeled,
            winit::VirtualKeyCode::VolumeDown => Key::VolumeDown,
            winit::VirtualKeyCode::VolumeUp => Key::VolumeUp,
            winit::VirtualKeyCode::Wake => Key::Wake,
            winit::VirtualKeyCode::WebBack => Key::WebBack,
            winit::VirtualKeyCode::WebFavorites => Key::WebFavorites,
            winit::VirtualKeyCode::WebForward => Key::WebForward,
            winit::VirtualKeyCode::WebHome => Key::WebHome,
            winit::VirtualKeyCode::WebRefresh => Key::WebRefresh,
            winit::VirtualKeyCode::WebSearch => Key::WebSearch,
            winit::VirtualKeyCode::WebStop => Key::WebStop,
            winit::VirtualKeyCode::Yen => Key::Yen,
        }
    }
}

pub trait InputTranslate {
    fn translate(&self) -> Key;
}