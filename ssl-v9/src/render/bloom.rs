use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BloomUniforms {
    pub threshold: f32,
    pub intensity: f32,
    pub blur_radius: f32,
    pub _padding: f32,
}

impl Default for BloomUniforms {
    fn default() -> Self {
        Self {
            threshold: 1.0,      // Brightness threshold
            intensity: 0.8,      // Bloom intensity multiplier
            blur_radius: 1.0,    // Blur spread
            _padding: 0.0,
        }
    }
}

pub struct BloomPass {
    // Pipelines for each pass
    bright_pass_pipeline: wgpu::RenderPipeline,
    blur_h_pipeline: wgpu::RenderPipeline,
    blur_v_pipeline: wgpu::RenderPipeline,
    combine_pipeline: wgpu::RenderPipeline,
    
    // Textures
    bright_texture: wgpu::Texture,
    bright_view: wgpu::TextureView,
    blur_temp_texture: wgpu::Texture,
    blur_temp_view: wgpu::TextureView,
    blur_final_texture: wgpu::Texture,
    blur_final_view: wgpu::TextureView,
    
    // Bind groups
    bright_pass_bind_group: wgpu::BindGroup,
    blur_h_bind_group: wgpu::BindGroup,
    blur_v_bind_group: wgpu::BindGroup,
    combine_bind_group: wgpu::BindGroup,
    
    // Uniforms
    uniform_buffer: wgpu::Buffer,
    pub uniforms: BloomUniforms,
    
    // Sampler
    sampler: wgpu::Sampler,
    
    // Dimensions
    width: u32,
    height: u32,
}

impl BloomPass {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        hdr_format: wgpu::TextureFormat,
    ) -> Self {
        let width = config.width;
        let height = config.height;
        
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Bloom Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("bloom.wgsl").into()),
        });
        
        // Create textures (half resolution for bloom)
        let bloom_width = width / 2;
        let bloom_height = height / 2;
        
        let create_bloom_texture = |label: &str| -> (wgpu::Texture, wgpu::TextureView) {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: bloom_width,
                    height: bloom_height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: hdr_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            (texture, view)
        };
        
        let (bright_texture, bright_view) = create_bloom_texture("Bright Pass Texture");
        let (blur_temp_texture, blur_temp_view) = create_bloom_texture("Blur Temp Texture");
        let (blur_final_texture, blur_final_view) = create_bloom_texture("Blur Final Texture");
        
        // Sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Bloom Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        
        // Uniform buffer
        let uniforms = BloomUniforms::default();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Bloom Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bloom Bind Group Layout"),
            entries: &[
                // Input texture
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
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Uniform buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        
        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Bloom Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create pipelines
        let create_pipeline = |label: &str, entry_point: &str, format: wgpu::TextureFormat| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_fullscreen",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point,
                    targets: &[Some(wgpu::ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            })
        };
        
        let bright_pass_pipeline = create_pipeline("Bloom Bright Pass", "fs_bright_pass", hdr_format);
        let blur_h_pipeline = create_pipeline("Bloom Blur H", "fs_blur_h", hdr_format);
        let blur_v_pipeline = create_pipeline("Bloom Blur V", "fs_blur_v", hdr_format);
        let combine_pipeline = create_pipeline("Bloom Combine", "fs_bloom_combine", hdr_format);
        
        // Create bind groups (will be updated with actual textures)
        let create_bind_group = |label: &str, texture_view: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            })
        };
        
        // Placeholder bind groups (will be recreated with proper textures in render())
        let bright_pass_bind_group = create_bind_group("Bright Pass BG", &bright_view);
        let blur_h_bind_group = create_bind_group("Blur H BG", &bright_view);
        let blur_v_bind_group = create_bind_group("Blur V BG", &blur_temp_view);
        let combine_bind_group = create_bind_group("Combine BG", &blur_final_view);
        
        Self {
            bright_pass_pipeline,
            blur_h_pipeline,
            blur_v_pipeline,
            combine_pipeline,
            bright_texture,
            bright_view,
            blur_temp_texture,
            blur_temp_view,
            blur_final_texture,
            blur_final_view,
            bright_pass_bind_group,
            blur_h_bind_group,
            blur_v_bind_group,
            combine_bind_group,
            uniform_buffer,
            uniforms,
            sampler,
            width: bloom_width,
            height: bloom_height,
        }
    }
    
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        hdr_texture_view: &wgpu::TextureView,
        device: &wgpu::Device,
    ) {
        // Update uniforms
        encoder.copy_buffer_to_buffer(
            &device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[self.uniforms]),
                usage: wgpu::BufferUsages::COPY_SRC,
            }),
            0,
            &self.uniform_buffer,
            0,
            std::mem::size_of::<BloomUniforms>() as u64,
        );
        
        // Pass 1: Bright Pass (HDR -> Bright)
        {
             // Create transient bind group for input BEFORE pass
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bloom Bright Pass Input Bind Group"),
                layout: &self.bright_pass_pipeline.get_bind_group_layout(0),
                entries: &[
                     wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(hdr_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.uniform_buffer.as_entire_binding(),
                    },
                ],
            });
            
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Bloom Bright Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.bright_view,
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
            
            pass.set_pipeline(&self.bright_pass_pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.draw(0..3, 0..1);  // Fullscreen triangle
        }
        
        // Pass 2: Horizontal Blur (Bright -> Temp)
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Bloom Blur Horizontal"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blur_temp_view,
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
            
            pass.set_pipeline(&self.blur_h_pipeline);
            pass.set_bind_group(0, &self.blur_h_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        
        // Pass 3: Vertical Blur (Temp -> Final)
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Bloom Blur Vertical"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.blur_final_view,
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
            
            pass.set_pipeline(&self.blur_v_pipeline);
            pass.set_bind_group(0, &self.blur_v_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        
        // Pass 4: Combine (handled externally with bloom texture)
    }
    
    pub fn get_bloom_texture(&self) -> &wgpu::TextureView {
        &self.blur_final_view
    }

    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.width = config.width;
        self.height = config.height;

        // Re-create textures
        // Bright Texture (Full Res or Downscaled?) -> Usually same res as HDR or half? 
        // Logic in new() used config width/height? Assume yes.
        let bright_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Bloom Bright"),
            size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let bright_view = bright_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let blur_temp_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Bloom Blur Temp"),
            size: wgpu::Extent3d { width: config.width / 2, height: config.height / 2, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let blur_temp_view = blur_temp_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let blur_final_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Bloom Blur Final"),
            size: wgpu::Extent3d { width: config.width / 2, height: config.height / 2, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let blur_final_view = blur_final_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.bright_texture = bright_texture;
        self.bright_view = bright_view;
        self.blur_temp_texture = blur_temp_texture;
        self.blur_temp_view = blur_temp_view;
        self.blur_final_texture = blur_final_texture;
        self.blur_final_view = blur_final_view;
        
        // Re-create bind groups using stored pipelines
        // NOTE: bright_pass_bind_group is recreated in `render` or depends on input.
        // If `render` expects `self.bright_pass_bind_group` to be valid, we can't update it here without the input texture.
        // BUT, looking at `render` logic: `pass.set_bind_group(0, &bind_group, &[]);`
        // `render` creates a LOCAL `bind_group` using `hdr_texture_view`.
        // So we DO NOT need to update `self.bright_pass_bind_group` here.
        
        // However, Blur H and Blur V bind groups DEPEND on internal textures (bright_view, blur_temp_view).
        // These MUST be updated.
        
        self.blur_h_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bloom Blur H Bind Group"),
            layout: &self.blur_h_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.bright_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        
        self.blur_v_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bloom Blur V Bind Group"),
            layout: &self.blur_v_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.blur_temp_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        
        // Combine bind group? If used.
    }
}
