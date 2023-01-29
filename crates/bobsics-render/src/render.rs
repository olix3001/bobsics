use wgpu::{*, util::StagingBelt};
use winit::{window::Window, dpi::PhysicalSize};

use crate::{utils, QuadPipeline};

const BACKGROUND: utils::Color = utils::Color::from_rgba(0.1, 0.9, 0.1, 1.0);

pub struct BobsicsRenderer {
    instance: Instance,
    adapter: Adapter,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    staging_belt: StagingBelt,

    quad_pipeline: QuadPipeline,
}

impl BobsicsRenderer {
    pub async fn new(window: &Window) -> Self {
        let instance = Instance::new(Backends::all());

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        }, None).await.unwrap();

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

        let quad_pipeline = QuadPipeline::new(&device, surface.get_supported_formats(&adapter)[0]);

        Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            staging_belt,

            quad_pipeline
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Bobsics render encoder"),
        });

        // Color render pass
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Bobsics render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BACKGROUND.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // Quads
        self.quad_pipeline.draw(&self.device, &mut self.staging_belt, &mut encoder, &view, vec![
            crate::Quad {
                top_left: [0.1, 0.1],
                bottom_right: [0.9, 0.9],
                color: [1.0, 0.0, 0.0, 1.0],
                border_radius: 0.0
            },
            crate::Quad {
                top_left: [-0.9, -0.1],
                bottom_right: [-0.1, 0.9],
                color: [0.0, 0.0, 1.0, 1.0],
                border_radius: 0.0
            }
        ]);

        // Execute
        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.staging_belt.recall();

        Ok(())
    }
}