use cgmath::*;
use std::path::Path;
use wgpu::util::DeviceExt;

#[derive(Debug, Clone)]
pub struct Bone {
    pub name: String,
    pub index: usize,
    pub parent_index: Option<usize>,
    pub inverse_bind_matrix: Matrix4<f32>,
    pub local_transform: Matrix4<f32>, // Bind pose
    pub children: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub bones: Vec<Bone>,
    pub global_inverse_transform: Matrix4<f32>,
}

#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
    pub channels: Vec<AnimationChannel>,
}

#[derive(Debug, Clone)]
pub struct AnimationChannel {
    pub target_bone_index: usize,
    pub inputs: Vec<f32>, // Time inputs
    pub outputs: ChannelOutputs,
}

#[derive(Debug, Clone)]
pub enum ChannelOutputs {
    Translations(Vec<Vector3<f32>>),
    Rotations(Vec<Quaternion<f32>>), // xyz w
    Scales(Vec<Vector3<f32>>),
}

impl Skeleton {
    pub fn new(bones: Vec<Bone>) -> Self {
        Self {
            bones,
            global_inverse_transform: Matrix4::identity(),
        }
    }

    pub fn compute_matrices(&self) -> Vec<Matrix4<f32>> {
        let mut matrices = vec![Matrix4::identity(); self.bones.len()];
        
        // Compute global transforms recursively (or iteratively if sorted)
        // Since we stored children, we can use a stack or recursion.
        // Assuming roots have parent_index = None.
        
        // Optimization: If bones are sorted by hierarchy (parents before children), simple loop works.
        // We guarantee this? No.
        // So let's do recursive.
        
        let mut global_transforms = vec![Matrix4::identity(); self.bones.len()];
        
        for i in 0..self.bones.len() {
            if self.bones[i].parent_index.is_none() {
                self.compute_global_transform(i, Matrix4::identity(), &mut global_transforms);
            }
        }
        
        for i in 0..self.bones.len() {
             matrices[i] = global_transforms[i] * self.bones[i].inverse_bind_matrix;
        }
        matrices
    }

    fn compute_global_transform(&self, bone_idx: usize, parent_transform: Matrix4<f32>, globals: &mut Vec<Matrix4<f32>>) {
        let bone = &self.bones[bone_idx];
        let global = parent_transform * bone.local_transform;
        globals[bone_idx] = global;
        
        for &child_idx in &bone.children {
            self.compute_global_transform(child_idx, global, globals);
        }
    }
}

impl AnimationClip {
    pub fn interpolate(&self, time: f32) -> Vec<(usize, Matrix4<f32>)> {
        // Return (bone_idx, local_transform) for affected bones
        // Simple mock for now: no interpolation, just identity
        // Logic: Find keyframes before/after time, lerp.
        vec![] 
    }
}

pub fn load_gltf(
    path: &Path,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout, // Material layout
) -> anyhow::Result<crate::render::scene::Model> {
    let (document, buffers, images) = gltf::import(path)?;

    // 1. Load Textures & Materials (Placeholder for now)
    let materials = vec![]; 

    // 2. Load Skins (Skeleton)
    let mut skeleton = None;
    if let Some(skin) = document.skins().next() {
        let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
        let ibms: Vec<Matrix4<f32>> = reader
            .read_inverse_bind_matrices()
            .map(|iter| iter.map(|m| Matrix4::from(m)).collect())
            .unwrap_or_else(|| vec![]);

        let mut bones = vec![];
        let mut node_to_bone_idx = std::collections::HashMap::new();

        // First pass: Create Bones
        for (i, joint) in skin.joints().enumerate() {
            let name = joint.name().unwrap_or("Bone").to_string();
            
            // Extract local transform
            let (t, r, s) = joint.transform().decomposed();
             let local_transform = Matrix4::from_translation(Vector3::from(t))
                * Matrix4::from(Quaternion::new(r[3], r[0], r[1], r[2]))
                * Matrix4::from_nonuniform_scale(s[0], s[1], s[2]);

            bones.push(Bone {
                name,
                index: i,
                parent_index: None, 
                inverse_bind_matrix: if i < ibms.len() { ibms[i] } else { Matrix4::identity() },
                local_transform,
                children: vec![], 
            });
            node_to_bone_idx.insert(joint.index(), i);
        }

        // Second pass: Connect Constraints
        for joint in skin.joints() {
            let parent_node_idx = joint.index();
            if let Some(&parent_bone_idx) = node_to_bone_idx.get(&parent_node_idx) {
                for child in joint.children() {
                    if let Some(&child_bone_idx) = node_to_bone_idx.get(&child.index()) {
                        // Connect
                        bones[parent_bone_idx].children.push(child_bone_idx);
                        bones[child_bone_idx].parent_index = Some(parent_bone_idx);
                    }
                }
            }
        }
        
        skeleton = Some(Skeleton::new(bones));
    }

    // 3. Load Meshes
    let mut meshes = vec![];
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut vertices = vec![];
            
            // Positions
            let positions: Vec<[f32; 3]> = reader.read_positions().map(|iter| iter.collect()).unwrap_or_default();
            // Normals
            let normals: Vec<[f32; 3]> = reader.read_normals().map(|iter| iter.collect()).unwrap_or_default();
             // UVs
            let tex_coords: Vec<[f32; 2]> = reader.read_tex_coords(0).map(|iter| iter.into_f32().collect()).unwrap_or_default();
            
            // Joints & Weights
            let joints_vec: Vec<[u16; 4]> = reader.read_joints(0).map(|iter| iter.into_u16().collect()).unwrap_or_default();
            let weights_vec: Vec<[f32; 4]> = reader.read_weights(0).map(|iter| iter.into_f32().collect()).unwrap_or_default();

            for i in 0..positions.len() {
                let p = positions[i];
                let n = if i < normals.len() { normals[i] } else { [0.0, 1.0, 0.0] };
                let t = if i < tex_coords.len() { tex_coords[i] } else { [0.0, 0.0] };
                
                let j_u16 = if i < joints_vec.len() { joints_vec[i] } else { [0; 4] };
                let j = [j_u16[0] as u32, j_u16[1] as u32, j_u16[2] as u32, j_u16[3] as u32];
                let w = if i < weights_vec.len() { weights_vec[i] } else { [0.0; 4] };

                vertices.push(crate::render::scene::Vertex {
                    position: p,
                    normal: n,
                    tex_coords: t,
                    tangent: [0.0; 3], // Should compute tangents
                    bitangent: [0.0; 3],
                    joints: j,
                    weights: w,
                });
            }

            // Indices
            let indices: Vec<u32> = reader.read_indices().map(|iter| iter.into_u32().collect()).unwrap_or_default();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", mesh.name())),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", mesh.name())),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            meshes.push(crate::render::scene::Mesh {
                name: mesh.name().unwrap_or("Mesh").to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32,
                material_id: 0,
            });
        }
    }

    Ok(crate::render::scene::Model {
        meshes,
        materials,
        skeleton,
        animations: vec![], // TODO: Load animations
    })
}
