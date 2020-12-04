use std::mem::size_of;

struct BufferDimensions {
    width: usize,
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let bytes_per_pixel = size_of::<u32>();
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;
        Self {
            width,
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
        }
    }
}

pub struct WGPUState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    buffer_dimensions: BufferDimensions,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    texture_extent: wgpu::Extent3d,
    storage_buffer: wgpu::Buffer,
}

impl WGPUState {
    fn create_shader_modules(device: &wgpu::Device) -> (wgpu::ShaderModule, wgpu::ShaderModule) {
        let vs_src = include_str!("shader.vert");
        let fs_src = include_str!("shader.frag");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler
            .compile_into_spirv(
                vs_src,
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
                fs_src,
                shaderc::ShaderKind::Fragment,
                "shader.frag",
                "main",
                None,
            )
            .unwrap();
        let vs_module =
            device.create_shader_module(wgpu::util::make_spirv(&vs_spirv.as_binary_u8()));
        let fs_module =
            device.create_shader_module(wgpu::util::make_spirv(&fs_spirv.as_binary_u8()));

        return (vs_module, fs_module);
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        render_pipeline
    }

    fn create_texture_and_buffer(
        device: &wgpu::Device,
        buffer_dimensions: &BufferDimensions,
    ) -> (
        wgpu::Texture,
        wgpu::Extent3d,
        wgpu::TextureView,
        wgpu::Buffer,
    ) {
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: (buffer_dimensions.padded_bytes_per_row * buffer_dimensions.height) as u64,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_extent = wgpu::Extent3d {
            width: buffer_dimensions.width as u32,
            height: buffer_dimensions.height as u32,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("test texture"),
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::all(),
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        return (texture, texture_extent, texture_view, storage_buffer);
    }

    pub async fn new(width: usize, height: usize) -> Self {
        // The handle to our GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let (vs_module, fs_module) = Self::create_shader_modules(&device);

        let render_pipeline = Self::create_render_pipeline(&device, &vs_module, &fs_module);

        let buffer_dimensions = BufferDimensions::new(width, height);
        let (texture, texture_extent, texture_view, storage_buffer) =
            Self::create_texture_and_buffer(&device, &buffer_dimensions);

        Self {
            device: device,
            queue: queue,
            render_pipeline: render_pipeline,
            buffer_dimensions: buffer_dimensions,
            texture: texture,
            texture_view: texture_view,
            texture_extent: texture_extent,
            storage_buffer: storage_buffer,
        }
    }

    pub async fn render(&mut self) -> Vec<u8> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // Start render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            // Set pipeline and draw
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // Copy result from draw into CPU buffer
        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &self.storage_buffer,
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: self.buffer_dimensions.padded_bytes_per_row as u32,
                    rows_per_image: 0,
                },
            },
            self.texture_extent,
        );

        // Submit encoder to GPU queue
        self.queue.submit(std::iter::once(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.storage_buffer.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

        self.device.poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        let mut result: Vec<u8> = vec![];
        if let Ok(()) = buffer_future.await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            data.chunks(self.buffer_dimensions.padded_bytes_per_row)
                .flat_map(|padded_buffer_row| {
                    &padded_buffer_row[..self.buffer_dimensions.unpadded_bytes_per_row]
                })
                .for_each(|c| result.push(c.clone()));

            // Free storage_buffer memory
            drop(data);
            self.storage_buffer.unmap();
        }

        result
    }
}
