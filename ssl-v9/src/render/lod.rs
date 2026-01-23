
use wgpu::util::DeviceExt;

pub struct LODMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffers: Vec<wgpu::Buffer>, // 0: High, 1: Med, 2: Low
    pub index_counts: Vec<u32>,
    pub distances: [f32; 2], // Thresholds: [High->Med, Med->Low]
}

impl LODMesh {
    pub fn new(
        device: &wgpu::Device,
        vertices: &[crate::render::scene::Vertex],
        indices_high: &[u32],
        indices_med: &[u32],
        indices_low: &[u32],
        distances: [f32; 2],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("LOD Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let create_index_buffer = |label: &str, data: &[u32]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::INDEX,
            })
        };

        let index_buffers = vec![
            create_index_buffer("LOD0 Index Buffer", indices_high),
            create_index_buffer("LOD1 Index Buffer", indices_med),
            create_index_buffer("LOD2 Index Buffer", indices_low),
        ];

        let index_counts = vec![
            indices_high.len() as u32,
            indices_med.len() as u32,
            indices_low.len() as u32,
        ];

        Self {
            vertex_buffer,
            index_buffers,
            index_counts,
            distances,
        }
    }

    pub fn get_lod_level(&self, distance: f32) -> usize {
        if distance < self.distances[0] {
            0
        } else if distance < self.distances[1] {
            1
        } else {
            2
        }
    }
}
