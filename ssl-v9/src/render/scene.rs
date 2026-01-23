use wgpu::util::DeviceExt;
use cgmath::prelude::*;
use crate::render::animation::{Skeleton, AnimationClip};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub joints: [u32; 4], // Bone indices
    pub weights: [f32; 4], // Bone weights
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 2 + mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 3 + mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 4 + mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Uint32x4,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() * 4 + mem::size_of::<[f32; 2]>() + mem::size_of::<[u32; 4]>()) as wgpu::BufferAddress,
                    shader_location: 13,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // Front face
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0], tangent: [-1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0], tangent: [-1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0], tangent: [-1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0], tangent: [-1.0, 0.0, 0.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    // Top face
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, -1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 0.0], normal: [0.0, 1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, -1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, -1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, -1.0], joints: [0; 4], weights: [0.0; 4] },
    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, -1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, 1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, 1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], normal: [0.0, -1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, 1.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0], tangent: [1.0, 0.0, 0.0], bitangent: [0.0, 0.0, 1.0], joints: [0; 4], weights: [0.0; 4] },
    // Right face
    Vertex { position: [0.5, -0.5, -0.5], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0], tangent: [0.0, 0.0, -1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, -0.5], tex_coords: [1.0, 1.0], normal: [1.0, 0.0, 0.0], tangent: [0.0, 0.0, -1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0], tangent: [0.0, 0.0, -1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [0.5, -0.5, 0.5], tex_coords: [0.0, 0.0], normal: [1.0, 0.0, 0.0], tangent: [0.0, 0.0, -1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0], normal: [-1.0, 0.0, 0.0], tangent: [0.0, 0.0, 1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, -0.5, 0.5], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0], tangent: [0.0, 0.0, 1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], normal: [-1.0, 0.0, 0.0], tangent: [0.0, 0.0, 1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
    Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0], tangent: [0.0, 0.0, 1.0], bitangent: [0.0, 1.0, 0.0], joints: [0; 4], weights: [0.0; 4] },
];

pub const INDICES: &[u32] = &[
    0, 1, 2, 2, 3, 0, // Front
    4, 5, 6, 6, 7, 4, // Back
    8, 9, 10, 10, 11, 8, // Top
    12, 13, 14, 14, 15, 12, // Bottom
    16, 17, 18, 18, 19, 16, // Right
    20, 21, 22, 22, 23, 20, // Left
];

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        // WGPU coordinate system difference? usually clip space -1 to 1 or 0 to 1
        // OPENGL_TO_WGPU_MATRIX needed if cgmath assumes OpenGL
        let opengl_to_wgpu_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );
        opengl_to_wgpu_matrix * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub view_pos: [f32; 4], // Padding for WGSL alignment? vec4 is 16 bytes.
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderSettingsUniform {
    pub cel_shading_factor: f32,
    pub padding: [f32; 7],
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub skeleton: Option<Skeleton>,
    pub animations: Vec<AnimationClip>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: crate::render::texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material_id: usize,
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model = cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        let normal = cgmath::Matrix3::from(self.rotation);
        
        InstanceRaw {
            model: model.into(),
            normal: normal.into(),
        }
    }
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
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
    }
}
