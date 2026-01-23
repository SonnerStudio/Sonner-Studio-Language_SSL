use cgmath::{Matrix4, Point3, Vector3, InnerSpace};
use wgpu;

pub struct StereoscopicRenderer {
    pub left_eye_texture: wgpu::Texture,
    pub left_eye_view: wgpu::TextureView,
    pub right_eye_texture: wgpu::Texture,
    pub right_eye_view: wgpu::TextureView,
    pub ipd: f32, // Interpupillary distance in meters
    pub resolution_per_eye: (u32, u32),
}

impl StereoscopicRenderer {
    pub fn new(device: &wgpu::Device, resolution_per_eye: (u32, u32)) -> Self {
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("XR Eye Texture"),
            size: wgpu::Extent3d {
                width: resolution_per_eye.0,
                height: resolution_per_eye.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        
        let left_eye_texture = device.create_texture(&texture_desc);
        let left_eye_view = left_eye_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let right_eye_texture = device.create_texture(&texture_desc);
        let right_eye_view = right_eye_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        Self {
            left_eye_texture,
            left_eye_view,
            right_eye_texture,
            right_eye_view,
            ipd: 0.063, // Average IPD: 63mm
            resolution_per_eye,
        }
    }
    
    /// Generate projection matrix for an eye
    pub fn get_eye_projection(&self, eye: super::Eye, fov_degrees: f32, near: f32, far: f32) -> Matrix4<f32> {
        let aspect = self.resolution_per_eye.0 as f32 / self.resolution_per_eye.1 as f32;
        cgmath::perspective(cgmath::Deg(fov_degrees), aspect, near, far)
    }
    
    /// Generate view matrix for an eye
    pub fn get_eye_view(&self, eye: super::Eye, head_position: Point3<f32>, head_forward: Vector3<f32>, head_up: Vector3<f32>) -> Matrix4<f32> {
        // Calculate eye offset
        let right = head_forward.cross(head_up).normalize();
        let eye_offset = match eye {
            super::Eye::Left => -right * (self.ipd / 2.0),
            super::Eye::Right => right * (self.ipd / 2.0),
        };
        
        let eye_position = head_position + eye_offset;
        let target = eye_position + head_forward;
        
        Matrix4::look_at_rh(eye_position, target, head_up)
    }
    
    pub fn resize(&mut self, device: &wgpu::Device, new_resolution: (u32, u32)) {
        self.resolution_per_eye = new_resolution;
        
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("XR Eye Texture"),
            size: wgpu::Extent3d {
                width: new_resolution.0,
                height: new_resolution.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        
        self.left_eye_texture = device.create_texture(&texture_desc);
        self.left_eye_view = self.left_eye_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.right_eye_texture = device.create_texture(&texture_desc);
        self.right_eye_view = self.right_eye_texture.create_view(&wgpu::TextureViewDescriptor::default());
    }
    
    pub fn get_eye_view_ref(&self, eye: super::Eye) -> &wgpu::TextureView {
        match eye {
            super::Eye::Left => &self.left_eye_view,
            super::Eye::Right => &self.right_eye_view,
        }
    }
}
