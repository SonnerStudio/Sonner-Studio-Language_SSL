// Forced rebuild to pick up WGSL changes
use wgpu::util::DeviceExt;
use cgmath::prelude::*; // Import SquareMatrix and others

#[cfg(not(target_arch = "wasm32"))]
use tao as window_lib;
#[cfg(target_arch = "wasm32")]
use winit as window_lib;

use window_lib::window::Window;
use crate::render::texture::Texture;
use crate::render::scene::Vertex;
use cgmath::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    view_pos: [f32; 4],
    resolution: [f32; 2],
    flags: u32, // Bit 0: Retro Mode, Bit 1: Fog Enable
    _padding: u32,
    fog_color: [f32; 4], // RGB + Fog Mode (0=linear, 1=exp, 2=exp2)
    fog_params: [f32; 4], // start, end, density, _padding
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 4],
    color: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: window_lib::dpi::PhysicalSize<u32>,
    pub window: std::sync::Arc<Window>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub depth_texture: Texture,
    
    // Textures & Materials
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub env_texture: Texture,
    
    // Layouts
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
    pub material_bind_group_layout: wgpu::BindGroupLayout,
    pub light_bind_group_layout: wgpu::BindGroupLayout,
    
    // Instance Data
    pub instances: Vec<crate::render::scene::Instance>,
    pub instance_buffer: wgpu::Buffer,
    
    // Mesh Data (Legacy/LOD0)
    // We will replace these with LODMesh for the main scene
    // pub vertex_buffer: wgpu::Buffer,
    // pub index_buffer: wgpu::Buffer,
    // pub num_indices: u32,
    
    // Phase 8: LOD System
    pub lod_mesh: crate::render::lod::LODMesh,
    // Buffers for sorted instances per LOD
    pub instance_buffer_lod0: wgpu::Buffer,
    pub instance_buffer_lod1: wgpu::Buffer,
    pub instance_buffer_lod2: wgpu::Buffer,
    
    // TODO Phase 8: Re-enable these after implementing modules
    // pub compute_system: crate::render::compute::ComputeSystem,
    // pub post_process: crate::render::post::PostProcess,
    
    // Shadows & Uniforms
    // Storing matrices for SSAO/PostFX access
    pub camera_view_matrix: [[f32; 4]; 4],
    pub camera_proj_matrix: [[f32; 4]; 4],
    
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_buffer: wgpu::Buffer,
    pub material_bind_group: wgpu::BindGroup,
    pub light_buffer: wgpu::Buffer,
    pub light_bind_group: wgpu::BindGroup,
    pub shadow_pass_bind_group: wgpu::BindGroup,
    pub shadow_texture: Texture,
    pub shadow_pipeline: wgpu::RenderPipeline,

    // Raytracer
    pub raytracer: crate::render::raytracer::Raytracer,
    
    // Phase 8: Post-Processing
    pub bloom_pass: Option<crate::render::bloom::BloomPass>,
    pub bloom_enabled: bool,
    pub post_effects_pass: Option<crate::render::post_effects::PostEffectsPass>,
    pub post_effects_enabled: bool,
    
    // HDR Pipeline
    pub hdr_texture: wgpu::Texture,
    pub hdr_view: wgpu::TextureView,
    pub hdr_format: wgpu::TextureFormat,
    
    // Sprint 3: Deferred Shading
    pub deferred_pass: crate::render::deferred::DeferredPass,
    pub deferred_enabled: bool,
    
    // Sprint 2: SSAO
    pub ssao_pass: Option<crate::render::ssao::SSAOPass>,
    pub ssao_enabled: bool,
    
    // Depth of Field
    pub dof_pass: Option<crate::render::dof::DOFPass>,
    pub dof_enabled: bool,
    
    // Feature Flags
    pub ssr_pass: Option<crate::render::ssr::SSRPass>,
    pub ssr_enabled: bool,
    pub retro_mode: bool,
    pub fog_enabled: bool,
    pub fog_mode: u32, // 0=linear, 1=exp, 2=exp2
    pub fog_color: [f32; 3],
    pub fog_start: f32,
    pub fog_end: f32,
    pub fog_density: f32,
    pub motion_blur_enabled: bool,
    
    // Render Settings (Cel Shading etc)
    pub settings_buffer: wgpu::Buffer,
    pub settings_bind_group: wgpu::BindGroup,
    pub cel_shading_enabled: bool,
    
    // Outlines
    pub outline_pass: Option<crate::render::outlines::OutlinePass>,
    pub outline_enabled: bool,
    
    // Physics
    pub physics_engine: crate::render::physics::PhysicsEngine,
    pub physics_handles: Vec<rapier3d::dynamics::RigidBodyHandle>,

    // Audio
    pub audio_engine: Option<crate::render::audio::AudioEngine>,
    
    // Particles
    pub particle_system: Option<crate::render::particles::ParticleSystem>,
    pub particles_enabled: bool,
    
    // Animation (Phase 10)
    pub bone_bind_group_layout: wgpu::BindGroupLayout,
    pub bone_buffer: wgpu::Buffer,
    pub bone_bind_group: wgpu::BindGroup,
    pub skeleton: Option<crate::render::animation::Skeleton>,
    pub animation_time: f32,
    pub is_operational: bool,
}

impl State {
    pub async fn new(window: std::sync::Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let surface = match unsafe { 
            match wgpu::SurfaceTargetUnsafe::from_window(&*window) {
                Ok(target) => instance.create_surface_unsafe(target),
                Err(e) => {
                    eprintln!("❌ Failed to create surface target: {:?}", e);
                    eprintln!("💡 Your graphics hardware may not be compatible with this application.");
                    panic!("Cannot create graphics surface target - incompatible hardware");
                }
            }
        } {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Failed to create surface: {:?}", e);
                eprintln!("💡 Your graphics hardware may not be compatible with this application.");
                panic!("Cannot create graphics surface - incompatible hardware");
            }
        };

        let adapter = match instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await {
            Some(adapter) => adapter,
            None => {
                eprintln!("❌ Failed to find compatible graphics adapter with high performance preference");
                eprintln!("💡 Trying fallback adapter with lower power preference...");
                
                // Try with fallback adapter
                match instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::LowPower,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: true,
                }).await {
                    Some(adapter) => {
                        eprintln!("✅ Using fallback graphics adapter");
                        adapter
                    },
                    None => {
                        eprintln!("❌ No compatible graphics adapter found");
                        eprintln!("💡 Please update your graphics drivers or ensure your system meets minimum requirements.");
                        panic!("No compatible graphics adapter found - please update graphics drivers");
                    }
                }
            }
        };

        let (device, queue) = match adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await {
            Ok((device, queue)) => (device, queue),
            Err(e) => {
                eprintln!("❌ Failed to create graphics device: {:?}", e);
                eprintln!("💡 Your graphics drivers may need to be updated.");
                eprintln!("💡 Supported backends: {:?}", wgpu::Backends::all());
                panic!("Cannot create graphics device - please update graphics drivers");
            }
        };

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
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
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        let diffuse_texture = Texture::create_plain_texture(&device, "diffuse_texture");
        let normal_texture = Texture::create_plain_texture(&device, "normal_texture");
        let env_texture = Texture::create_cube_texture(&device, "env_texture");

        // --- Bind Group Layouts ---
        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let material_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
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
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2, // Normal Map
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3, // Normal Sampler
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("material_bind_group_layout"),
        });

        let light_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { // Uniform
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Shadow Map
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Shadow Sampler
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Env Map
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::Cube,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry { // Env Sampler
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("light_bind_group_layout"),
        });

        let shadow_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("shadow_bind_group_layout"),
        });


        // --- PIPELINES ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &material_bind_group_layout,
                &light_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                    wgpu::VertexBufferLayout {
                         array_stride: 100 as wgpu::BufferAddress, 
                         step_mode: wgpu::VertexStepMode::Instance,
                         attributes: &[
                             wgpu::VertexAttribute { offset: 0, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 16, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 32, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 48, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 64, shader_location: 9, format: wgpu::VertexFormat::Float32x3 },
                             wgpu::VertexAttribute { offset: 76, shader_location: 10, format: wgpu::VertexFormat::Float32x3 },
                             wgpu::VertexAttribute { offset: 88, shader_location: 11, format: wgpu::VertexFormat::Float32x3 },
                         ],
                    }
                ], 
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float, 
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // --- Shadow Pipeline ---
        let shadow_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shadow.wgsl").into()),
        });
        
        let shadow_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
             label: Some("Shadow Pipeline Layout"),
             bind_group_layouts: &[&camera_bind_group_layout, &shadow_bind_group_layout], 
             push_constant_ranges: &[],
        });
        
        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: Some(&shadow_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shadow_shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                    wgpu::VertexBufferLayout {
                         array_stride: 100 as wgpu::BufferAddress,
                         step_mode: wgpu::VertexStepMode::Instance,
                         attributes: &[
                             wgpu::VertexAttribute { offset: 0, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 16, shader_location: 6, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 32, shader_location: 7, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 48, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                             wgpu::VertexAttribute { offset: 64, shader_location: 9, format: wgpu::VertexFormat::Float32x3 },
                             wgpu::VertexAttribute { offset: 76, shader_location: 10, format: wgpu::VertexFormat::Float32x3 },
                             wgpu::VertexAttribute { offset: 88, shader_location: 11, format: wgpu::VertexFormat::Float32x3 },
                         ],
                    }
                ], 
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Front),
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // --- Instance Data ---
        let instances = vec![crate::render::scene::Instance {
            position: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
        }];
        
        let instance_data = instances.iter().map(crate::render::scene::Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        // --- Mesh Data (Quad) ---
        const VERTICES: &[Vertex] = &[
            Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
            Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
            Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
            Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
        ];
        const INDICES: &[u16] = &[
            0, 1, 2,
            0, 2, 3, 
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // Initialize LOD Mesh
        // Simulate LODs by reducing index count (just for visual test of system)
        // LOD0: Full indices
        // LOD1: 50% indices (stride 2) - messy geometry but proves the switch
        // LOD2: 25% indices (stride 4)
        
        let indices_lod0 = crate::render::scene::INDICES.to_vec();
        let indices_lod1: Vec<u32> = indices_lod0.iter().step_by(2).cloned().collect(); // Very crude simplification!
        let indices_lod2: Vec<u32> = indices_lod0.iter().step_by(4).cloned().collect();
        
        let lod_mesh = crate::render::lod::LODMesh::new(
            &device,
            crate::render::scene::VERTICES, // Full vertex data
            &indices_lod0,
            &indices_lod1,
            &indices_lod2,
            [15.0, 30.0], // Distances: <15=LOD0, <30=LOD1, >30=LOD2
        );
        
        // Instance Buffers for each LOD bucket
        // Max capacity = total instances (safe upper bound)
        // Since we filter, we re-create or write to these buffers every frame if dynamic.
        // It's better to create them large enough once.
        let instance_data = instances.iter().map(crate::render::scene::Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer_size = (instance_data.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64;
        
        let create_inst_buf = |label: &str| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: instance_buffer_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        
        let instance_buffer_lod0 = create_inst_buf("Instance Buffer LOD0");
        let instance_buffer_lod1 = create_inst_buf("Instance Buffer LOD1");
        let instance_buffer_lod2 = create_inst_buf("Instance Buffer LOD2");
        
        // Fill initial for safety (though overwritten in render)
        // queue.write_buffer(&instance_buffer_lod0, 0, bytemuck::cast_slice(&instance_data));
        
        let index_count_align = 0; // Padding if needed

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        // --- Resources (Buffers & BindGroups) ---
        let camera_uniform = CameraUniform {
            view_proj: cgmath::Matrix4::identity().into(),
            view_pos: [0.0; 4],
            resolution: [config.width as f32, config.height as f32],
            flags: 1, // Enable Retro Mode by default for demo
            _padding: 0,
            fog_color: [0.5, 0.5, 0.6, 0.0], // Light gray-blue
            fog_params: [10.0, 50.0, 0.05, 0.0], // start, end, density
        };
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() }],
            label: Some("camera_bind_group"),
        });

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&diffuse_texture.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&normal_texture.view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&normal_texture.sampler) },
            ],
            label: Some("material_bind_group"),
        });

        let light_uniform = LightUniform {
             position: [2.0, 4.0, 2.0, 1.0],
             color: [1.0, 1.0, 1.0, 1.0],
             view_proj: cgmath::Matrix4::identity().into(), // Will update in render
        };
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        
        let shadow_texture = Texture::create_shadow_texture(&device, "shadow_texture");
        
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: light_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&shadow_texture.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&shadow_texture.sampler) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&env_texture.view) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::Sampler(&env_texture.sampler) },
            ],
            label: Some("light_bind_group"),
        });

        // TODO Phase 8: Re-enable after implementing modules
        // let compute_system = crate::render::compute::ComputeSystem::new(&device);
        // let post_process = crate::render::post::PostProcess::new(&device, &config);
        let raytracer = crate::render::raytracer::Raytracer::new(&device, &config);
        
        // Phase 8: Create HDR Render Target
        let hdr_format = wgpu::TextureFormat::Rgba16Float;
        let hdr_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("HDR Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: hdr_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let hdr_view = hdr_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let identity_matrix: [[f32; 4]; 4] = cgmath::Matrix4::<f32>::identity().into();

        // Sprint 2: Initialize SSAO
        let ssao_pass = Some(crate::render::ssao::SSAOPass::new(
            &device,
            &queue,
            &config,
        ));

        // Sprint 2: Initialize DOF
        // Uses surface format for output usually (unless post-processing chain continues)
        let dof_pass = Some(crate::render::dof::DOFPass::new(
            &device,
            &config,
            config.format, // Input format likely same as output for simplicity? 
            // Wait, input is HDR/Bloom? Or directly Surface?
            // If we run DOF at end of chain, it takes input color and outputs to surface/swapchain
        ));
        
        // Phase 8: Initialize Bloom (using HDR format)
        let bloom_pass = Some(crate::render::bloom::BloomPass::new(
            &device,
            &config,
            hdr_format, // Now using proper HDR format!
        ));
        
        // Phase 8: Initialize Post Effects (outputs to surface format)
        let post_effects_pass = Some(crate::render::post_effects::PostEffectsPass::new(
            &device,
            config.format, // Outputs to surface
        ));

        // Initialize SSR
        let ssr_pass = Some(crate::render::ssr::SSRPass::new(&device, &config));

        // --- Render Settings ---
        let settings_uniform = crate::render::scene::RenderSettingsUniform {
            cel_shading_factor: 1.0, // Enable by default for demo
            padding: [0.0; 7],
        };
        let settings_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Settings Buffer"),
            contents: bytemuck::cast_slice(&[settings_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let settings_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Settings Bind Group Layout"),
            entries: &[
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
            ],
        });

        let settings_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Settings Bind Group"),
            layout: &settings_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: settings_buffer.as_entire_binding() },
            ],
        });

        // Initialize Outlines
        let outline_pass = Some(crate::render::outlines::OutlinePass::new(
             &device,
             &config,
             config.format, // Output directly to surface? Or blend?
             // Since we use resolve loops, let's output to surface format.
        ));
        
        // Initialize Physics
        let mut physics_engine = crate::render::physics::PhysicsEngine::new();
        let mut physics_handles = Vec::new();
        
        // Create Floor (Fixed)
        let _floor_handle = physics_engine.add_box([0.0, -1.0, 0.0], [50.0, 1.0, 50.0], false);
        
        // Create Falling Box (Dynamic)
        // Matches instance[0] which was at 0,0,0. Let's move it up.
        let box_handle = physics_engine.add_box([0.0, 10.0, 0.0], [1.0, 1.0, 1.0], true);
        physics_handles.push(box_handle);

        // Initialize Audio (Delayed - will be done in init_audio)
        let audio_engine = None;

        // Initialize Particles
        // Need to pass Camera Buffer (which is inside camera_bind_group or camera_buffer?)
        // State has `camera_buffer`.
        let particle_system = Some(crate::render::particles::ParticleSystem::new(
            &device,
            &config,
            &camera_buffer
        ));

        // Initialize Bone Buffer for Skeletal Animation
        let bone_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bone Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create initial bone buffer (256 bones max, identity matrices)
        let max_bones = 256;
        let identity_matrices = vec![cgmath::Matrix4::<f32>::identity(); max_bones];
        let bone_data: Vec<[[f32; 4]; 4]> = identity_matrices.iter().map(|m| (*m).into()).collect();
        
        let bone_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Bone Buffer"),
            contents: bytemuck::cast_slice(&bone_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bone_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bone Bind Group"),
            layout: &bone_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: bone_buffer.as_entire_binding(),
                },
            ],
        });

        // Initialize Deferred Shading
        let deferred_pass = crate::render::deferred::DeferredPass::new(
            &device,
            &config,
            &camera_bind_group_layout,
            &light_bind_group_layout,
            &material_bind_group_layout,
            &settings_bind_group_layout,
            &bone_bind_group_layout,
        );

        // Initialize Shadow Pass Bind Group (Safe for Shadow Pass - uses Depth Texture instead of Shadow Texture)
        let shadow_pass_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &shadow_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: light_buffer.as_entire_binding() },
            ],
            label: Some("shadow_pass_bind_group"),
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            diffuse_texture,
            normal_texture,
            env_texture,
            camera_bind_group_layout,
            light_bind_group_layout,
            camera_buffer,
            camera_bind_group,
            camera_view_matrix: cgmath::Matrix4::identity().into(),
            camera_proj_matrix: cgmath::Matrix4::identity().into(),
            light_buffer,
            light_bind_group,
            shadow_pass_bind_group,
            material_bind_group,
            material_bind_group_layout,
            instances,
            instance_buffer,
            instance_buffer_lod0,
            instance_buffer_lod1,
            instance_buffer_lod2,
            lod_mesh,
            depth_texture,
            // compute_system,
            raytracer,
            shadow_texture,
            shadow_pipeline,
            hdr_texture,
            hdr_view,
            hdr_format,
            bloom_pass,
            bloom_enabled: true,
            post_effects_pass,
            post_effects_enabled: true,
            ssao_pass,
            ssao_enabled: true,
            dof_pass,
            dof_enabled: true,
            ssr_pass,
            ssr_enabled: true,
            
            audio_engine,
            
            particle_system,
            particles_enabled: true,
            
            deferred_pass,
            deferred_enabled: true,

            retro_mode: false,
            fog_enabled: false,
            fog_mode: 1,
            fog_color: [0.5, 0.6, 0.7],
            fog_start: 10.0,
            fog_end: 50.0,
            fog_density: 0.05,
            motion_blur_enabled: false,
            
            settings_buffer,
            settings_bind_group,
            cel_shading_enabled: true, // Default On
            
            outline_pass,
            outline_enabled: true, // Default On
            
            physics_engine,
            physics_handles,
            
            bone_bind_group_layout,
            bone_buffer,
            bone_bind_group,
            skeleton: None,
            animation_time: 0.0,
            is_operational: true,
            
            
            





            

        }
    }

    pub fn resize(&mut self, new_size: window_lib::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            // Re-create Depth Texture
            self.depth_texture = crate::render::texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            
            // Resize HDR Texture
             let hdr_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("HDR Texture"),
                size: wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.hdr_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            self.hdr_view = hdr_texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.hdr_texture = hdr_texture; // Update struct field
            
            // Resize SSAO
            if let Some(ref mut ssao) = self.ssao_pass {
                ssao.resize(&self.device, self.config.width, self.config.height);
            }
            
            // Resize Bloom
            if let Some(ref mut bloom) = self.bloom_pass {
                bloom.resize(&self.device, &self.config);
            }
            
            // Resize SSR
            if let Some(ref mut ssr) = self.ssr_pass {
                ssr.resize(&self.device, &self.config);
            }
            
            // Resize Deferred
            self.deferred_pass.resize(&self.device, &self.config);
            
            // Resize Outlines
            if let Some(ref mut outline) = self.outline_pass {
               outline.resize(&self.device, self.config.width, self.config.height);
            }
             
            // self.post_process.resize(&self.device, new_size.width, new_size.height);
            // Raytracer resize needs care, but for now we skip complex resize logic to avoid bugs
            self.raytracer.resize(&self.device, new_size.width, new_size.height);
        }
    }

    pub fn input(&mut self, event: &window_lib::event::WindowEvent) -> bool {
        match event {
            window_lib::event::WindowEvent::KeyboardInput {
                event:
                    window_lib::event::KeyEvent {
                        physical_key,
                        state: window_lib::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                if *physical_key == window_lib::keyboard::KeyCode::KeyC {
                     self.cel_shading_enabled = !self.cel_shading_enabled;
                     println!("Cel Shading: {}", self.cel_shading_enabled);
                     true
                    } else if *physical_key == window_lib::keyboard::KeyCode::KeyO {
                     self.outline_enabled = !self.outline_enabled;
                     println!("Outlines: {}", self.outline_enabled);
                     true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    pub fn update(&mut self) {
        // Update Render Settings Buffer
        let settings_uniform = crate::render::scene::RenderSettingsUniform {
            cel_shading_factor: if self.cel_shading_enabled { 1.0 } else { 0.0 },
            padding: [0.0; 7],
        };
        self.queue.write_buffer(&self.settings_buffer, 0, bytemuck::cast_slice(&[settings_uniform]));
        
        if let Some(ref mut outline) = self.outline_pass {
            outline.update_uniforms(&self.queue, self.config.width, self.config.height);
        }
        
        // Update Physics
        self.physics_engine.step();
        
        // Sync Physics to Visuals (Instances)
        // Assuming physics_handles[0] maps to instances[0]
        if let Some(handle) = self.physics_handles.get(0) {
            if let Some(body) = self.physics_engine.rigid_body_set.get(*handle) {
                let translation = body.translation();
                let rotation = body.rotation();
                
                // Update Instance 0
                // We need to access self.instances (Wait, instances are not in State struct directly?)
                // Ah, instances were local in new(). We need to move them to State struct to update them!
                 if let Some(instance) = self.instances.get_mut(0) {
                     instance.position = cgmath::Vector3::new(translation.x, translation.y, translation.z);
                     instance.rotation = cgmath::Quaternion::new(rotation.w, rotation.i, rotation.j, rotation.k);
                 }
            }
        }
        
        // Write to Instance Buffer (Dynamic Update)
        let instance_data = self.instances.iter().map(crate::render::scene::Instance::to_raw).collect::<Vec<_>>();
        self.queue.write_buffer(&self.instance_buffer_lod0, 0, bytemuck::cast_slice(&instance_data));
        
        // Update Particles (Compute)
        if self.particles_enabled {
            if let Some(ref particle_system) = self.particle_system {
                // Fixed dt for now
                particle_system.update(&self.device, &self.queue, 0.016);
            }
        }
        
        // Update Animation
        self.animation_time += 0.016; // 60 FPS assumption
        if let Some(ref skeleton) = self.skeleton {
            let bone_matrices = skeleton.compute_matrices();
            let bone_data: Vec<[[f32; 4]; 4]> = bone_matrices.iter().map(|m| (*m).into()).collect();
            self.queue.write_buffer(&self.bone_buffer, 0, bytemuck::cast_slice(&bone_data));
        }
    }
    
    pub fn rotate_object(&mut self, angle: f32) {
        use cgmath::Rotation3;
        for instance in &mut self.instances {
             let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_y(), cgmath::Deg(angle));
             instance.rotation = instance.rotation * rotation;
        }
        
        let instance_data = self.instances.iter().map(crate::render::scene::Instance::to_raw).collect::<Vec<_>>();
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));
    }

    pub fn load_material(&mut self, diffuse_path: &str, normal_path: Option<&str>) {
        // Load Diffuse
        let diffuse_bytes = std::fs::read(diffuse_path).unwrap_or_else(|_| vec![255, 0, 255, 255]); // Magenta fallback
        let diffuse_texture = Texture::from_bytes(&self.device, &self.queue, &diffuse_bytes, "diffuse_texture").unwrap();
        
        // Load Normal (or default)
        let normal_texture = if let Some(path) = normal_path {
             let bytes = std::fs::read(path).unwrap_or_default();
             if bytes.is_empty() {
                 Texture::create_plain_texture(&self.device, "normal_default")
             } else {
                 Texture::from_bytes(&self.device, &self.queue, &bytes, "normal_texture").unwrap()
             }
        } else {
            Texture::create_plain_texture(&self.device, "normal_default")
        };
        
        // Update Bind Group
        self.diffuse_texture = diffuse_texture;
        self.normal_texture = normal_texture;
        
        self.material_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.material_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.diffuse_texture.view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.diffuse_texture.sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&self.normal_texture.view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&self.normal_texture.sampler) },
            ],
            label: Some("material_bind_group"),
        });
    }

    pub fn set_retro_mode(&mut self, enabled: bool) {
        self.retro_mode = enabled;
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if !self.is_operational {
            return Ok(());
        }
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });
        
        let cam_x = 4.0;
        let cam_z = 4.0;

        // 1. Update Uniforms (Camera & Light)
        {
             // Simple Orbit Camera logic (could be moved to update)
             let time = 0.0; // dummy time if needed
             // let cam_x = ... (removed)
             let view = cgmath::Matrix4::look_at_rh(
                cgmath::Point3::new(cam_x, 2.0, cam_z),
                cgmath::Point3::new(0.0, 0.0, 0.0),
                cgmath::Vector3::unit_y(),
             );
             let proj = cgmath::perspective(cgmath::Deg(45.0), self.config.width as f32 / self.config.height as f32, 0.1, 100.0);
             
             // Store for SSAO/PostFX
             self.camera_view_matrix = view.into();
             self.camera_proj_matrix = proj.into(); // Standard projection
             
             // Invert Y for WGPU if using direct proj?
             // cgmath::perspective assumes -1 to 1 Z usually? WGPU is 0 to 1.
             // But existing code seemed to rely on it. Let's assume correct for now.
             
             let raw_cam = CameraUniform {
                 view_proj: (proj * view).into(),
                 view_pos: [cam_x, 2.0, cam_z, 1.0],
                 resolution: [self.config.width as f32, self.config.height as f32],
                 flags: if self.retro_mode { 1 } else { 0 } | if self.fog_enabled { 2 } else { 0 },
                 _padding: 0,
                 fog_color: [0.5, 0.6, 0.7, 0.0], // Linear fog
                 fog_params: [self.fog_start, self.fog_end, self.fog_density, 0.0],
             };
             self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[raw_cam]));
             
             // Light
             let light_pos = cgmath::Point3::new(2.0, 4.0, 2.0);
             let light_view = cgmath::Matrix4::look_at_rh(
                 light_pos,
                 cgmath::Point3::new(0.0, 0.0, 0.0),
                 cgmath::Vector3::unit_y(),
             );
             let light_proj = cgmath::ortho(-10.0, 10.0, -10.0, 10.0, 1.0, 20.0);
             let raw_light = LightUniform {
                 position: [light_pos.x, light_pos.y, light_pos.z, 1.0],
                 color: [1.0, 1.0, 1.0, 1.0],
                 view_proj: (light_proj * light_view).into(),
             };
             self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[raw_light]));
        }
        
        // 2. Compute Pass
        // self.compute_system.compute(&mut encoder);
        
        // 2b. Raytracing Pass
        self.raytracer.compute(&mut encoder, self.config.width, self.config.height);
        
        // 3. Shadow Pass (Refactored)
        // LOD Sorting & Culling first

        // --- PREPARE INSTANCE BUFFERS (LOD + CULLING) ---
        
         // Re-calculate matrices (needed for Frustum)
        let view_mat: cgmath::Matrix4<f32> = self.camera_view_matrix.into();
        let proj_mat: cgmath::Matrix4<f32> = self.camera_proj_matrix.into();
        let view_proj = proj_mat * view_mat;
        
        let frustum = crate::render::frustum::Frustum::from_matrix(view_proj);
        
        let mut batch_lod0 = Vec::new();
        let mut batch_lod1 = Vec::new();
        let mut batch_lod2 = Vec::new();
        
        // We also need a "Shadow Batch" - typically all visible shadow casters from LIGHT pov.
        // For simplicity, we use the CAMERA visible set for now (imperfection: shadows from off-screen objects missing).
        // Correct way: Cull against Light Frustum + Camera Frustum.
        // For now: Just use the camera visible ones to keep it simple.
        
        let cam_pos = cgmath::Vector3::new(cam_x, 2.0, cam_z);
        
        for instance in &self.instances {
            let pos = cgmath::Vector3::new(
                instance.position[0],
                instance.position[1],
                instance.position[2]
            );
            
            if frustum.contains_sphere(pos, 1.5) {
                let dist = (pos - cam_pos).magnitude();
                let lod = self.lod_mesh.get_lod_level(dist);
                
                let raw = instance.to_raw();
                match lod {
                    0 => batch_lod0.push(raw),
                    1 => batch_lod1.push(raw),
                    _ => batch_lod2.push(raw),
                }
            }
        }
        
        // Upload buffers
        if !batch_lod0.is_empty() {
            self.queue.write_buffer(&self.instance_buffer_lod0, 0, bytemuck::cast_slice(&batch_lod0));
        }
        if !batch_lod1.is_empty() {
            self.queue.write_buffer(&self.instance_buffer_lod1, 0, bytemuck::cast_slice(&batch_lod1));
        }
        if !batch_lod2.is_empty() {
            self.queue.write_buffer(&self.instance_buffer_lod2, 0, bytemuck::cast_slice(&batch_lod2));
        }

        // 3. Shadow Pass
        {
            let mut shadow_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Shadow Pass"),
                color_attachments: &[],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.shadow_texture.view,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            shadow_pass.set_pipeline(&self.shadow_pipeline);
            shadow_pass.set_bind_group(0, &self.camera_bind_group, &[]); 
            shadow_pass.set_bind_group(1, &self.shadow_pass_bind_group, &[]); 
            
            // Use LOD2 (Low Poly) for shadows for performance
            shadow_pass.set_vertex_buffer(0, self.lod_mesh.vertex_buffer.slice(..)); 
            
            // Draw all batches using LOD2 geometry?
            // Actually, if we want correct shadows, we should draw them with their respective geometry or a unified low-poly proxy.
            // If we use LOD2 indices for everything, it might look wrong for close objects if LOD2 is very decimated.
            // Let's iterate and draw all 3 batches using LOD2 indices (shadows don't need high detail).
            
            shadow_pass.set_index_buffer(self.lod_mesh.index_buffers[2].slice(..), wgpu::IndexFormat::Uint32);
            
            if !batch_lod0.is_empty() {
                shadow_pass.set_vertex_buffer(1, self.instance_buffer_lod0.slice(0..(batch_lod0.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                shadow_pass.draw_indexed(0..self.lod_mesh.index_counts[2], 0, 0..batch_lod0.len() as u32);
            }
             if !batch_lod1.is_empty() {
                shadow_pass.set_vertex_buffer(1, self.instance_buffer_lod1.slice(0..(batch_lod1.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                shadow_pass.draw_indexed(0..self.lod_mesh.index_counts[2], 0, 0..batch_lod1.len() as u32);
            }
            if !batch_lod2.is_empty() {
                shadow_pass.set_vertex_buffer(1, self.instance_buffer_lod2.slice(0..(batch_lod2.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                shadow_pass.draw_indexed(0..self.lod_mesh.index_counts[2], 0, 0..batch_lod2.len() as u32);
            }
            drop(shadow_pass);
        } // End Shadow Pass

        // 4a. Deferred Geometry Pass (Scene -> G-Buffer)
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Deferred Geometry Pass"),
                color_attachments: &[
                    // Target 0: Position
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.deferred_pass.gbuffer.position_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    // Target 1: Normal
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.deferred_pass.gbuffer.normal_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    // Target 2: Albedo
                    Some(wgpu::RenderPassColorAttachment {
                        view: &self.deferred_pass.gbuffer.albedo_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.deferred_pass.geometry_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.material_bind_group, &[]);
            render_pass.set_bind_group(2, &self.bone_bind_group, &[]);
            
            // Set Common Vertex Buffer (LODMesh)
            render_pass.set_vertex_buffer(0, self.lod_mesh.vertex_buffer.slice(..));
            
            // Draw LOD0
            if !batch_lod0.is_empty() {
                 render_pass.set_vertex_buffer(1, self.instance_buffer_lod0.slice(0..(batch_lod0.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                 render_pass.set_index_buffer(self.lod_mesh.index_buffers[0].slice(..), wgpu::IndexFormat::Uint32);
                 render_pass.draw_indexed(0..self.lod_mesh.index_counts[0], 0, 0..batch_lod0.len() as u32);
            }
            
            // Draw LOD1
            if !batch_lod1.is_empty() {
                 render_pass.set_vertex_buffer(1, self.instance_buffer_lod1.slice(0..(batch_lod1.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                 render_pass.set_index_buffer(self.lod_mesh.index_buffers[1].slice(..), wgpu::IndexFormat::Uint32);
                 render_pass.draw_indexed(0..self.lod_mesh.index_counts[1], 0, 0..batch_lod1.len() as u32);
            }
            
            // Draw LOD2
            if !batch_lod2.is_empty() {
                 render_pass.set_vertex_buffer(1, self.instance_buffer_lod2.slice(0..(batch_lod2.len() * std::mem::size_of::<crate::render::scene::InstanceRaw>()) as u64));
                 render_pass.set_index_buffer(self.lod_mesh.index_buffers[2].slice(..), wgpu::IndexFormat::Uint32);
                 render_pass.draw_indexed(0..self.lod_mesh.index_counts[2], 0, 0..batch_lod2.len() as u32);
            }
        } // End Geometry Pass

        // 4b. Deferred Lighting Pass (G-Buffer -> HDR Texture)
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Deferred Lighting Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                     view: &self.hdr_view, // Write to HDR
                     resolve_target: None,
                     ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 }), // Sky/Clear color
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None, // Lighting pass is fullscreen, no depth needed
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.deferred_pass.lighting_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.light_bind_group, &[]);
            render_pass.set_bind_group(2, &self.deferred_pass.lighting_bind_group, &[]);
            render_pass.set_bind_group(3, &self.settings_bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Fullscreen Quad
        }
        
        // Phase 8: Multi-Pass Post-Processing Pipeline
        
        // Pass 0: SSAO
        if self.ssao_enabled {
            if let Some(ref ssao_pass) = self.ssao_pass {
                ssao_pass.render(
                    &self.device, 
                    &mut encoder, 
                    &self.queue, 
                    &self.depth_texture.view, 
                    self.camera_view_matrix,
                    self.camera_proj_matrix,
                    &self.deferred_pass.gbuffer.normal_view // Use G-Buffer Normal
                );
            }
        }

        // Pass 1: Bloom
        if self.bloom_enabled {
            if let Some(ref bloom_pass) = self.bloom_pass {
                bloom_pass.render(&mut encoder, &self.hdr_view, &self.device);
            }
        }
        
        // Pass 1.5: SSR
        // We render SSR to its own texture. Later we might combine it.
        // For now, let's just run it to verify no crashes.
        let mut ssr_output_view = &self.hdr_view; // Fallback to HDR if SSR fails or disabled? No.
        
        if self.ssr_enabled {
            if let Some(ref ssr_pass) = self.ssr_pass {
                 ssr_pass.render(
                    &self.device,
                    &mut encoder,
                    &self.queue,
                    self.camera_view_matrix.into(),
                    self.camera_proj_matrix.into(),
                    &self.depth_texture.view,
                     &self.deferred_pass.gbuffer.normal_view,
                    &self.hdr_view, // Scene Color
                );
            }
        }

        // Pass 2: Post Effects or DOF
        // For now, handle output to screen
        
        let mut final_input_view = &self.hdr_view;
         if self.bloom_enabled {
             if let Some(ref bloom_pass) = self.bloom_pass {
                 final_input_view = bloom_pass.get_bloom_texture();
             }
         }
        
        // 9. Outlines (Post Process)
        // If Outline enabled, we render it. If Post Effects enabled, we render that?
        // Let's assume Outline is the "Final Stylistic Pass" overtaking realistic post processing.
        // OR we combine them.
        // For now, if Outline enabled, we bypass post effects or run it last. 
        // Let's Run Post Effects first (Tonemapping) then Outline on top?
        // But Outline Shader takes SCENE COLOR input. If we Tonemap first, colors change.
        // Let's run Outline on HDR View, then Tonemap?
        // Outline Pass outputs to specific view.
        
        // Let's try this flow:
        // 1. Deferred Lighting -> HDR
        // 2. Bloom -> Bloom Tex
        // 3. Post Effects (Tonemap) -> Surface
        // 4. Outline (if enabled) -> Reads HDR/Surface -> Writes Surface (blending black lines)
        
        // But our Outline Shader writes opaque.
        // Let's make Outline Shader write to Surface, reading HDR. It replaces PostEffects if enabled?
        // Yes, for now simpler.
        
        let mut rendered_to_surface = false;
        
        if self.outline_enabled {
            if let Some(ref outline) = self.outline_pass {
                outline.render(
                    &self.device,
                    &mut encoder,
                    &view, // Output to Surface
                    &self.depth_texture.view,
                     &self.deferred_pass.gbuffer.normal_view,
                    &self.hdr_view, // Input 
                );
                rendered_to_surface = true;
            }
        }
        
        // Render Particles (Transparent/Additive - rendered to same target as Outlines?)
        // Render to view (Surface). We should probably render to HDR if we want Bloom on particles.
        // But ParticleSystem is configured for Surface Format.
        // For now Render to Surface.
        
        if self.particles_enabled {
            if let Some(ref particle_system) = self.particle_system {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Particle Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load, 
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                
                particle_system.render(&mut pass);
            }
        }
        
        if !rendered_to_surface && self.post_effects_enabled {
            if let Some(ref post_effects_pass) = self.post_effects_pass {
                 // Ignore bloom input for now to fix compile, use HDR
                post_effects_pass.render(
                    &self.device,
                    &mut encoder,
                    &self.hdr_view, 
                    &view, // Output to Screen
                    &self.queue
                );
                rendered_to_surface = true;
            }
        } 
        
        if !rendered_to_surface {
             // Fallback: Copy HDR to Screen (Linear copy, no tonemap)
             // Or better: Use a simple copy pass.
             // For now just clear black if nothing enabled, or crash less?
             // Actually we can just run PostEffects if Outline disabled.
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }

    pub fn init_audio(&mut self) {
        if self.audio_engine.is_none() {
            println!("🔊 Initializing audio engine...");
            self.audio_engine = Some(crate::render::audio::AudioEngine::new());
        }
    }
}
