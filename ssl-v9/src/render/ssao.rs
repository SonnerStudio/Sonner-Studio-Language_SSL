use wgpu::util::DeviceExt;
use rand::Rng;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SSAOUniforms {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub inverse_projection: [[f32; 4]; 4],
    pub kernel_size: i32,
    pub radius: f32,
    pub bias: f32,
    pub power: f32,
    pub noise_scale: [f32; 2],
    pub _padding: [f32; 2], // Alignment padding
}

pub struct SSAOPass {
    pub pipeline: wgpu::RenderPipeline,
    pub blur_pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub blur_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_buffer: wgpu::Buffer,
    pub kernel_buffer: wgpu::Buffer,
    pub noise_texture: wgpu::Texture,
    pub noise_view: wgpu::TextureView,
    pub noise_sampler: wgpu::Sampler,
    pub ssao_texture: wgpu::Texture,
    pub ssao_view: wgpu::TextureView,
    pub blur_texture: wgpu::Texture,
    pub blur_view: wgpu::TextureView,
    pub output_format: wgpu::TextureFormat,
    
    kernel_size: i32,
    width: u32,
    height: u32,
}

impl SSAOPass {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let kernel_size = 64;
        let output_format = wgpu::TextureFormat::R8Unorm; // Single channel occlusion map
        let width = config.width;
        let height = config.height;

        // 1. Generate Kernel
        let mut rng = rand::thread_rng();
        let mut kernel: Vec<[f32; 4]> = Vec::with_capacity(kernel_size as usize);
        for i in 0..kernel_size {
            let scale = i as f32 / kernel_size as f32;
            let scale = 0.1 + scale * scale * (1.0 - 0.1); // Lerp
            
            let mut sample = [
                rng.gen_range(-1.0f32..1.0f32),
                rng.gen_range(-1.0f32..1.0f32),
                rng.gen_range(0.0f32..1.0f32), // Hemisphere
                0.0,
            ];
            // Normalize and scale
            let len: f32 = (sample[0] * sample[0] + sample[1] * sample[1] + sample[2] * sample[2]);
            let len = len.sqrt();
            if len > 0.0 {
                sample[0] /= len;
                sample[1] /= len;
                sample[2] /= len;
                sample[0] *= scale; // Scale
                sample[1] *= scale;
                sample[2] *= scale;
            }
            kernel.push(sample);
        }
        
        let kernel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("SSAO Kernel Buffer"),
            contents: bytemuck::cast_slice(&kernel),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 2. Generate Noise Texture (4x4)
        let noise_size = 4;
        let mut noise_data = Vec::with_capacity((noise_size * noise_size * 4) as usize);
        for _ in 0..(noise_size * noise_size) {
            noise_data.push(rng.gen_range(0.0..1.0) as f32); // R
            noise_data.push(rng.gen_range(0.0..1.0) as f32); // G
            noise_data.push(0.0); // B
            noise_data.push(1.0); // A
        }

        let noise_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("SSAO Noise Texture"),
            size: wgpu::Extent3d {
                width: noise_size,
                height: noise_size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &noise_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&noise_data),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(noise_size * 16), // 4 floats * 4 bytes
                rows_per_image: Some(noise_size),
            },
            wgpu::Extent3d {
                width: noise_size,
                height: noise_size,
                depth_or_array_layers: 1,
            },
        );

        let noise_view = noise_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let noise_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // 3. Output Textures (SSAO Raw + Blur)
        let (ssao_texture, ssao_view) = create_attachment_texture(device, width, height, output_format, "SSAO Raw");
        let (blur_texture, blur_view) = create_attachment_texture(device, width, height, output_format, "SSAO Blur");

        // 4. Uniform Buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SSAO Uniforms"),
            size: std::mem::size_of::<SSAOUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // 5. Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("SSAO Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("ssao.wgsl").into()),
        });

        // 6. Pipelines
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("SSAO Bind Group Layout"),
            entries: &[
                // Uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Depth Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Depth Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                // Noise Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Noise Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                // Kernel Array
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Normal Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                }
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("SSAO Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SSAO Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_ssao",
                targets: &[Some(wgpu::ColorTargetState {
                    format: output_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Blur Pipeline
        let blur_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("SSAO Blur Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let blur_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("SSAO Blur Pipeline Layout"),
            bind_group_layouts: &[&blur_bind_group_layout],
            push_constant_ranges: &[],
        });

        let blur_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SSAO Blur Pipeline"),
            layout: Some(&blur_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_blur",
                targets: &[Some(wgpu::ColorTargetState {
                    format: output_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Initialize noise data here actually (requires Queue, so we'll do it via temporary buffer usually or pass Queue in Init)
        // For simplicity, we assume caller will upload noise or we need a staging buffer method.
        // Let's rely on write_texture in constructor not being easy without Queue.
        // We'll update noise in first render frame or refactor new() to take Queue.

        Self {
            pipeline,
            blur_pipeline,
            bind_group_layout,
            blur_bind_group_layout,
            uniform_buffer,
            kernel_buffer,
            noise_texture,
            noise_view,
            noise_sampler,
            ssao_texture,
            ssao_view,
            blur_texture,
            blur_view,
            output_format,
            kernel_size,
            width,
            height,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let (ssao_texture, ssao_view) = create_attachment_texture(device, width, height, self.output_format, "SSAO Raw");
        let (blur_texture, blur_view) = create_attachment_texture(device, width, height, self.output_format, "SSAO Blur");
        self.ssao_texture = ssao_texture;
        self.ssao_view = ssao_view;
        self.blur_texture = blur_texture;
        self.blur_view = blur_view;
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        queue: &wgpu::Queue,
        depth_view: &wgpu::TextureView,
        view_matrix: [[f32; 4]; 4],
        proj_matrix: [[f32; 4]; 4],
        normal_view: &wgpu::TextureView,
    ) {
        // 1. Update Uniforms
        // Calculate Inverse Proj
        use cgmath::Matrix4;
        use cgmath::SquareMatrix;
        
        let proj: Matrix4<f32> = proj_matrix.into();
        let inv_proj = proj.invert().unwrap_or(Matrix4::identity());
        
        let uniforms = SSAOUniforms {
            view_matrix,
            projection_matrix: proj_matrix,
            inverse_projection: inv_proj.into(),
            kernel_size: self.kernel_size,
            radius: 0.5,
            bias: 0.025,
            power: 2.0,
            noise_scale: [self.width as f32 / 4.0, self.height as f32 / 4.0],
            _padding: [0.0, 0.0],
        };
        
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // 2. Wrap Noise init if needed (one time hack)
        // ...

        // 3. SSAO Pass
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SSAO Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: self.uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(depth_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&self.noise_sampler) }, // Depth sampler (reuse noise sampler)
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&self.noise_view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&self.noise_sampler) },
                wgpu::BindGroupEntry { binding: 5, resource: self.kernel_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 6, resource: wgpu::BindingResource::TextureView(normal_view) },
            ],
        });
        
        // 4. SSAO Render Pass
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("SSAO Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.ssao_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        
        // 5. Blur Pass
        let blur_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SSAO Blur Bind Group"),
            layout: &self.blur_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.ssao_view) }, // Input: Raw SSAO
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.noise_sampler) }, // Re-use sampler (filtering)
            ],
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("SSAO Blur Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blur_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            pass.set_pipeline(&self.blur_pipeline);
            pass.set_bind_group(0, &blur_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
    }
}

fn create_attachment_texture(device: &wgpu::Device, width: u32, height: u32, format: wgpu::TextureFormat, label: &str) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}


