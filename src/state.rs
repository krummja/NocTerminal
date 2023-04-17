use wgpu::{self, util::DeviceExt};
use winit::{
    window::Window,
    event::WindowEvent,
};
use bytemuck;

use std::borrow::Cow;

use crate::vertex::Vertex;
use crate::texture::Texture;


const VERTICES: &[Vertex] = &[
    // 0
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0]
    },
    // 1
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0]
    },
    // 2
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0]
    },
    // 3
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0]
    },
];


const INDICES: &[u16] = &[
    0, 2, 1,
    0, 3, 2,
];


pub struct State {
    pub window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: Texture,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // ! Adapter
        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                },
            )
            .await
            .unwrap();

        // ! Logical Device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    }
                },
                None,  // trace path
            )
            .await
            .unwrap();

        // ! Surface Configuration
        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // ! Texture Loading
        let diffuse_bytes = include_bytes!("images/yatta.png");
        let diffuse_texture = Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "yatta.png",
        ).unwrap();

        let texture_bind_group_layout = device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
            });

        let diffuse_bind_group = device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Diffuse Bind Group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &diffuse_texture.view,
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &diffuse_texture.sampler,
                        ),
                    },
                ],
            });

        // ! Render Pipeline
        let shader_resource = wgpu::ShaderSource::Wgsl(
            Cow::Borrowed(include_str!("shaders/shader.wgsl"))
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: shader_resource,
        });

        let render_pipeline_layout = device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(("Render Pipeline")),

                // The layout of bind groups for this pipeline.
                layout: Some(&render_pipeline_layout),

                // The compiled vertex stage, its entry point, and the input
                // buffers layout.
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        Vertex::desc(),
                    ],
                },

                // The compiled fragment stage, its entry point, and the color
                // targets.
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),

                // The properties of the pipeline at the primitive assembly
                // and rasterization level.
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires
                    // Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                // The effect of draw calls on the depth and stencil aspects of
                // the output target, if any.
                depth_stencil: None,


                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,  // Use all samples
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        // ! Buffers
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let num_indices = INDICES.len() as u32;

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut _render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        // This is what @location(0) in the fragment shader
                        // targets
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                // Set the key color
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color {
                                        r: 1.0,
                                        g: 0.0,
                                        b: 1.0,
                                        a: 1.0,
                                    }
                                ),
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: None,
                }
            );

            _render_pass.set_pipeline(&self.render_pipeline);
            _render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            _render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            _render_pass.set_index_buffer(
                self.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            _render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
