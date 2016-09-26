extern crate vulkano;
extern crate vulkano_win;

use std::sync::Arc;
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
use std::time::Duration;

pub struct Graphics3d {

}

impl Graphics3d {
    pub fn init(physical: &vulkano::instance::PhysicalDevice,
                window: &vulkano_win::Window,
                width: u32,
                height: u32)
                -> Self {

        let queue = physical.queue_families()
            .find(|q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
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
            let caps = window.surface()
                .get_capabilities(&physical)
                .expect("failed to get surface capabilities");
            let dimensions = caps.current_extent.unwrap_or([width, height]);
            let present = caps.present_modes.iter().next().unwrap();
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            vulkano::swapchain::Swapchain::new(&device,
                                               &window.surface(),
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
            include!{concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_vs.glsl")}
        }
        let vs = vs::Shader::load(&device).expect("failed to create shader module");
        mod fs {
            include!{concat!(env!("OUT_DIR"), "/shaders/glsl/triangle_fs.glsl")}
        }
        let fs = fs::Shader::load(&device).expect("failed to create shader module");

        mod render_pass {
            use vulkano::format::Format;
            single_pass_renderpass!{
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

        Graphics3d {}
    }
}
