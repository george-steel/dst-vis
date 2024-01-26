use crate::model::*;
use crate::util::*;
use std::ops::Range;
use wgpu::{BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Buffer, BufferUsages};
use wgpu::util::DeviceExt;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub pos: [f32;2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct BlockData {
    pub color: RGBA32float,
}


pub struct EmbDisplay {
    pipeline: wgpu::RenderPipeline,
    camera_buf: wgpu::Buffer,
    block_buf: wgpu::Buffer,
    block_ranges: Vec<Range<u32>>,
    bind_group: wgpu::BindGroup,
    design_bounds: Rectangle2,
}


impl EmbDisplay {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat, design: &[EmbOp]) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        /*let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });*/

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(texture_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let initial_ransform: [[f32; 2]; 2] = [[1.0,0.0], [0.0,1.0]];

        let camera_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("transform"),
            contents: bytemuck::cast_slice(&[Camera2D::id()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let mut stitch_data = Vec::new();
        let mut block_data = Vec::new();
        let mut block_ranges = Vec::new();
        let mut design_bounds = Rectangle2::single(cgmath::point2(0.0, 0.0));

        for op in design {
            for block in &op.blocks {
                let i = stitch_data.len();
                stitch_data.extend(block.iter().map(|p| {Vertex {pos: (*p).into()}}));
                design_bounds = block.iter().copied().fold(design_bounds, Rectangle2::add);

                let j = stitch_data.len();
                block_data.push(BlockData{
                    color: op.color,
                });
                block_ranges.push((i as u32)..(j as u32));
            }
        }
        let block_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("blocks"),
            contents: bytemuck::cast_slice(&block_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        let stitch_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("stitches"),
            contents: bytemuck::cast_slice(&stitch_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("stitch_vars"),
            layout: &render_pipeline.get_bind_group_layout(0),
            entries: &[
                BindGroupEntry{binding: 0, resource: camera_buf.as_entire_binding()},
                BindGroupEntry{binding: 1, resource: block_buf.as_entire_binding()},
                BindGroupEntry{binding: 2, resource: stitch_buf.as_entire_binding()},
            ],
        });
        EmbDisplay {
            pipeline: render_pipeline,
            camera_buf,
            block_buf,
            block_ranges,
            bind_group,
            design_bounds,
        }
    }
    pub fn render(&self, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::Texture) {
        let view = frame.create_view(&Default::default());
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(srgba(0x111111ff).into()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let size = frame.size();
        let aspect = size.width as f32 / size.height as f32;
        queue.write_buffer(&self.camera_buf, 0, bytemuck::cast_slice(&[Camera2D::fit_rect(&self.design_bounds, aspect)]));
        
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_pipeline(&self.pipeline);
        for (i, blk) in (0..).zip(self.block_ranges.iter()){
            rpass.draw(blk.clone(), i..i+1);
        }
    }
}

