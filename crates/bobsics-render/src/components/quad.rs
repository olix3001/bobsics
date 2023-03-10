use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, CommandEncoder, Device, TextureFormat, TextureView};

const DEFAULT_MAX_QUADS: usize = 10_000;

const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Quad {
    pub top_left: [f32; 2],
    pub bottom_right: [f32; 2],
    pub color: [f32; 4],
    pub border_radius: f32,
    pub border_color: [f32; 4],
    pub border_width: f32,
}

impl Quad {
    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x4,
        3 => Float32,
        4 => Float32x4,
        5 => Float32
    ];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug)]
pub struct QuadPipeline {
    instances: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    pipeline: wgpu::RenderPipeline,
}

impl QuadPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        // Create shader
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/quad.wgsl"));

        // Create buffers
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quad instance buffer"),
            size: (std::mem::size_of::<Quad>() * DEFAULT_MAX_QUADS) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad index buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Quad Pipeline Layout"),
                bind_group_layouts: &[globals_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Quad::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            instances: instance_buffer,
            index_buffer,

            pipeline: render_pipeline,
        }
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        instances: Vec<Quad>,
        globals_bind_group: &wgpu::BindGroup,
    ) {
        if instances.is_empty() {
            return;
        }
        // Set buffer
        let instance_bytes = bytemuck::cast_slice(&instances);
        let mut instance_buffer = staging_belt.write_buffer(
            encoder,
            &self.instances,
            0,
            wgpu::BufferSize::new(instance_bytes.len() as u64).unwrap(),
            device,
        );
        instance_buffer.copy_from_slice(instance_bytes);

        // Render pass
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Quad render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, globals_bind_group, &[]);
        rpass.set_vertex_buffer(0, self.instances.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.draw(0..INDICES.len() as u32, 0..instances.len() as u32);
    }
}

// ====< BRUSH >====
pub struct QuadBrush {
    pub pipeline: QuadPipeline,
    pub queue: Vec<Quad>,
}

impl QuadBrush {
    pub fn new(
        device: &Device,
        format: TextureFormat,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            pipeline: QuadPipeline::new(device, format, globals_bind_group_layout),
            queue: Vec::new(),
        }
    }

    pub fn queue_quad(&mut self, quad: Quad) {
        self.queue.push(quad);
    }

    pub fn draw_queued(
        &mut self,
        device: &Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        globals_bind_group: &wgpu::BindGroup,
    ) {
        self.pipeline.draw(
            device,
            staging_belt,
            encoder,
            view,
            self.queue.clone(),
            globals_bind_group,
        );
        self.queue.clear();
    }
}
