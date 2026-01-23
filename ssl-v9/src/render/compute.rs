use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    position: [f32; 2],
    velocity: [f32; 2],
}

pub struct ComputeSystem {
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    particle_buffer: wgpu::Buffer,
    pub num_particles: u32,
}

impl ComputeSystem {
    pub fn new(device: &wgpu::Device) -> Self {
        let num_particles = 1000;
        
        // Initial Data
        let mut particles = Vec::with_capacity(num_particles as usize);
        for i in 0..num_particles {
            let x: f32 = (i as f32 % 100.0) / 50.0 - 1.0;
            let y: f32 = (i as f32 / 100.0) / 5.0 - 1.0;
            let vx: f32 = 0.001 * (if i % 2 == 0 { 1.0 } else { -1.0 });
            let vy: f32 = 0.001 * (if i % 3 == 0 { 1.0 } else { -1.0 });
            particles.push(Particle {
                position: [x, y],
                velocity: [vx, vy],
            });
        }

        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Buffer"),
            contents: bytemuck::cast_slice(&particles),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Compute Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: particle_buffer.as_entire_binding(),
            }],
            label: Some("Compute Bind Group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("compute.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Self {
            pipeline,
            bind_group,
            particle_buffer,
            num_particles,
        }
    }

    pub fn compute(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &self.bind_group, &[]);
        // Dispatch workgroups (64 particles per group)
        let workgroups = (self.num_particles as f32 / 64.0).ceil() as u32;
        cpass.dispatch_workgroups(workgroups, 1, 1);
    }
}
