use artnet_protocol::*;
use std::net::{ToSocketAddrs, UdpSocket};

struct WGPUState {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl WGPUState {
    async fn new() -> Self {
        // The handle to our GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        println!("instance: {:?}", instance);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .unwrap();
        println!("adapter: {:?}", adapter);

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

        println!("device: {:?}", device);
        println!("queue: {:?}", queue);

        Self {
            device: device,
            queue: queue,
        }
    }

    async fn render(&mut self, texture_view: &wgpu::TextureView, texture: &wgpu::Texture) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let storage_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: 1024,
            usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.8,
                            g: 0.8,
                            b: 0.8,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }
        encoder.copy_texture_to_buffer(
            wgpu::TextureCopyView {
                texture: texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::BufferCopyView {
                buffer: &storage_buffer,
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 10,
                    rows_per_image: 10,
                },
            },
            wgpu::Extent3d {
                width: 0,
                height: 1,
                depth: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = storage_buffer.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);

        self.device.poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        if let Ok(()) = buffer_future.await {
            println!("ey");
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            println!("{:?}", data);
        }
        println!("buffer: {:?}", storage_buffer);
    }
}

fn main() {
    use futures::executor::block_on;
    let mut wgpu_state = block_on(WGPUState::new());
    let texture = wgpu_state.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("test texture"),
        size: wgpu::Extent3d {
            width: 20,
            height: 20,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsage::all(),
    });

    println!("texture: {:?}", texture);

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
        ..Default::default() //...wgpu::TextureViewDescriptor::Default,
                             // label: Some("Test view"),
                             // format: Some(wgpu::TextureFormat::Rgba16Float),
                             // dimension: Some(wgpu::TextureDimension::D2),
                             // aspect: wgpu::TextureAspect::All,
                             // base_mip_level: 0,
                             // level_count: 1,
                             // base_array_layer: 0,
                             // array_layer_count: None
    });
    block_on(wgpu_state.render(&texture_view, &texture));

    println!("texture: {:?}", texture);
}
