// Forced rebuild to pick up WGSL changes
use wgpu::util::DeviceExt;

pub struct GBuffer {
    pub position_texture: wgpu::Texture,
    pub position_view: wgpu::TextureView,
    pub normal_texture: wgpu::Texture,
    pub normal_view: wgpu::TextureView,
    pub albedo_texture: wgpu::Texture,
    pub albedo_view: wgpu::TextureView,
    // Depth is shared from State
}

impl GBuffer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        // 1. Position Texture (RGB16F)
        let position_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GBuffer Position"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        // 2. Normal Texture (RGB16F) - High precision for accurate lighting/SSR
        let normal_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GBuffer Normal"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        // 3. Albedo Texture (BGRA8)
        let albedo_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GBuffer Albedo"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        Self {
            position_view: position_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            position_texture,
            normal_view: normal_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            normal_texture,
            albedo_view: albedo_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            albedo_texture,
        }
    }
}

pub struct DeferredPass {
    pub gbuffer: GBuffer,
    pub geometry_pipeline: wgpu::RenderPipeline,
    pub lighting_pipeline: wgpu::RenderPipeline,
    pub lighting_bind_group_layout: wgpu::BindGroupLayout,
    pub lighting_bind_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
}

impl DeferredPass {
    pub fn new(
        device: &wgpu::Device, 
        config: &wgpu::SurfaceConfiguration, 
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout, // Used for Lighting Pass
        material_bind_group_layout: &wgpu::BindGroupLayout, // Used for Geometry Pass
        settings_bind_group_layout: &wgpu::BindGroupLayout,
        bone_bind_group_layout: &wgpu::BindGroupLayout, // NEW: Bone matrices
    ) -> Self {
        // 1. Create G-Buffer
        let gbuffer = GBuffer::new(device, config);
        
        // 2. Load Shaders
        let geometry_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Deferred Geometry Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("deferred_geometry.wgsl").into()),
        });
        
        let lighting_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Deferred Lighting Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("deferred_lighting.wgsl").into()),
        });
        
        // 3. Geometry Pipeline
        let geometry_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Geometry Pipeline Layout"),
            bind_group_layouts: &[
                camera_bind_group_layout,
                material_bind_group_layout, // Material (Texture, Normal Map)
                bone_bind_group_layout,     // Bone Matrices (Group 2 in shader is wrong, should be 3)
            ],
            push_constant_ranges: &[],
        });
        
        let geometry_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Geometry Pipeline"),
            layout: Some(&geometry_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &geometry_shader,
                entry_point: "vs_geometry",
                buffers: &[crate::render::scene::Vertex::desc(), crate::render::scene::InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &geometry_shader,
                entry_point: "fs_geometry",
                targets: &[
                    // Target 0: Position
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    // Target 1: Normal
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    // Target 2: Albedo
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // 4. Lighting Pipeline
        // Needs to read G-Buffer
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("GBuffer Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let lighting_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Deferred Lighting Bind Group Layout"),
            entries: &[
                // Position Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Position Sampler (Shared)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Normal Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Normal Sampler (Shared)
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Albedo Texture
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Albedo Sampler (Shared)
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Deferred Lighting Bind Group"),
            layout: &lighting_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&gbuffer.position_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&gbuffer.normal_view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&sampler) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&gbuffer.albedo_view) },
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        let lighting_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Lighting Pipeline Layout"),
            bind_group_layouts: &[
                camera_bind_group_layout, // Need Camera pos
                light_bind_group_layout,  // Need Lights
                &lighting_bind_group_layout, // G-Buffer
                settings_bind_group_layout, // Settings
            ],
            push_constant_ranges: &[],
        });
        
        let lighting_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Lighting Pipeline"),
            layout: Some(&lighting_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &lighting_shader,
                entry_point: "vs_lighting",
                buffers: &[], // Fullscreen quad generated
            },
            fragment: Some(wgpu::FragmentState {
                module: &lighting_shader,
                entry_point: "fs_lighting",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float, // HDR Output
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None, // No depth testing for lighting pass (fullscreen quad)
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            gbuffer,
            geometry_pipeline,
            lighting_pipeline,
            lighting_bind_group_layout,
            lighting_bind_group,
            sampler,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.gbuffer = GBuffer::new(device, config);
        
        // Recreate Bind Group
        self.lighting_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Deferred Lighting Bind Group"),
            layout: &self.lighting_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.gbuffer.position_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&self.gbuffer.normal_view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.sampler) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&self.gbuffer.albedo_view) },
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
    }
}
