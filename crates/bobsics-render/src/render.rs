use bytemuck::{Pod, Zeroable};
use wgpu::{
    util::{DeviceExt, StagingBelt},
    *,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::utils;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct GlobalsUniform {
    u_resolution: [f32; 2],
}

#[allow(dead_code)]
pub struct BobsicsRenderer {
    instance: Instance,
    adapter: Adapter,
    surface: Surface,
    pub device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    staging_belt: StagingBelt,

    globals: GlobalsUniform,
    globals_uniform: wgpu::Buffer,
    pub globals_bind_group_layout: wgpu::BindGroupLayout,
    globals_bind_group: wgpu::BindGroup,

    has_to_update_globals: bool,

    pub format: wgpu::TextureFormat,
}

impl BobsicsRenderer {
    pub async fn new(window: &Window) -> Self {
        let instance = Instance::new(Backends::all());

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let staging_belt = StagingBelt::new(10 * 1024);

        // Create globals
        let globals = GlobalsUniform {
            u_resolution: [size.width as f32, size.height as f32],
        };

        // Create globals uniform
        let globals_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Global uniform buffer"),
            contents: bytemuck::cast_slice(&[globals]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Globals bind group"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals_uniform.as_entire_binding(),
            }],
        });

        let format = surface.get_supported_formats(&adapter)[0];

        Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            staging_belt,

            globals,
            globals_uniform,
            globals_bind_group_layout,
            globals_bind_group,

            has_to_update_globals: false,

            format,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            // Update globals
            self.globals.u_resolution = [new_size.width as f32, new_size.height as f32];
            self.has_to_update_globals = true;

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, brush: &mut dyn Brush) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Bobsics render encoder"),
            });

        // Update globals if necessary
        if self.has_to_update_globals {
            self.has_to_update_globals = false;
            self.update_globals(&mut encoder);
        }

        // Color render pass
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Bobsics render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(utils::Color::from_hex(0x23242a).into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // Render method
        brush
            .draw_queued(
                &self.device,
                &mut self.staging_belt,
                &mut encoder,
                &view,
                self.config.width,
                self.config.height,
                &self.globals_bind_group,
            )
            .expect("Draw queued failed");
        // Quads

        // self.glyph_brush.draw_queued(&self.device, &mut self.staging_belt, &mut encoder, &view, self.config.width, self.config.height).expect("Draw queued failed");

        // Execute
        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.staging_belt.recall();

        Ok(())
    }

    fn update_globals(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let globals_bytes = bytemuck::bytes_of(&self.globals);
        let mut globals_buffer = self.staging_belt.write_buffer(
            encoder,
            &self.globals_uniform,
            0,
            wgpu::BufferSize::new(globals_bytes.len() as u64).unwrap(),
            &self.device,
        );
        globals_buffer.copy_from_slice(globals_bytes);
    }
}

#[allow(clippy::too_many_arguments)]
pub trait Brush {
    fn draw_queued(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        width: u32,
        height: u32,
        global_bind_group: &wgpu::BindGroup,
    ) -> Result<(), &str>;
}
