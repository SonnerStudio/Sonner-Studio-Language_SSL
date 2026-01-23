// Deferred Shading Shader

// --- GEOMETRY PASS ---

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(12) joints: vec4<u32>,
    @location(13) weights: vec4<f32>,
    // Instance Matrix (Mat4) - Locations 5-8
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    // Normal Matrix (Mat3) - Locations 9-11
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<f32>,
    flags: u32,
    _padding: u32,
    fog_color: vec4<f32>,
    fog_params: vec4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

// Bone Matrices Storage Buffer
struct BoneMatrices {
    matrices: array<mat4x4<f32>>,
}
@group(2) @binding(0) var<storage, read> bone_matrices: BoneMatrices;

fn get_skin_matrix(joints: vec4<u32>, weights: vec4<f32>) -> mat4x4<f32> {
    let m0 = bone_matrices.matrices[joints.x];
    let m1 = bone_matrices.matrices[joints.y];
    let m2 = bone_matrices.matrices[joints.z];
    let m3 = bone_matrices.matrices[joints.w];
    
    return m0 * weights.x + m1 * weights.y + m2 * weights.z + m3 * weights.w;
}

@vertex
fn vs_geometry(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Reconstruct Model Matrix
    let model_matrix = mat4x4<f32>(
        in.model_matrix_0,
        in.model_matrix_1,
        in.model_matrix_2,
        in.model_matrix_3,
    );
    
    let normal_matrix = mat3x3<f32>(
        in.normal_matrix_0,
        in.normal_matrix_1,
        in.normal_matrix_2,
    );
    
    // Apply Skinning if weights are non-zero
    var local_pos = vec4<f32>(in.position, 1.0);
    var local_normal = in.normal;
    
    let total_weight = in.weights.x + in.weights.y + in.weights.z + in.weights.w;
    if (total_weight > 0.01) {
        let skin_matrix = get_skin_matrix(in.joints, in.weights);
        local_pos = skin_matrix * local_pos;
        
        // For normals, use the 3x3 part
        let skin_rot = mat3x3<f32>(
            skin_matrix[0].xyz,
            skin_matrix[1].xyz,
            skin_matrix[2].xyz
        );
        local_normal = normalize(skin_rot * in.normal);
    }
    
    let world_pos = model_matrix * local_pos;
    let world_normal = normal_matrix * local_normal;
    
    out.world_position = world_pos.xyz;
    out.world_normal = world_normal;
    out.tex_coords = in.tex_coords;
    out.clip_position = camera.view_proj * world_pos;
    
    return out;
}

struct GBufferOutput {
    @location(0) position: vec4<f32>, // RGB16F
    @location(1) normal: vec4<f32>,   // RGB16F
    @location(2) albedo: vec4<f32>,   // RGBA8
}

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;
@group(1) @binding(2) var t_normal: texture_2d<f32>; // Material Normal Map
@group(1) @binding(3) var s_normal: sampler;

@fragment
fn fs_geometry(in: VertexOutput) -> GBufferOutput {
    var out: GBufferOutput;
    
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    if (base_color.a < 0.5) { discard; }

    // Normal Mapping would go here (TBN), for now usage vertex normal
    let normal = normalize(in.world_normal);

    out.position = vec4<f32>(in.world_position, 1.0); // W=1 for usage check?
    out.normal = vec4<f32>(normal, 1.0); // Store in [0,1]? No, F16 allows negative.
    out.albedo = base_color;
    
    return out;
}

// --- LIGHTING PASS ---

struct LightUniform {
    position: vec4<f32>,
    color: vec4<f32>,
    view_proj: mat4x4<f32>, // For shadows?
    // params: intensity, range, etc.
}

@group(0) @binding(0) var<uniform> camera_lighting: CameraUniform; // Reuse same struct
@group(1) @binding(0) var<uniform> light: LightUniform; // Single light for now, or array?

@group(2) @binding(0) var t_position: texture_2d<f32>;
@group(2) @binding(1) var s_position: sampler;
@group(2) @binding(2) var t_normal_g: texture_2d<f32>;
@group(2) @binding(3) var s_normal_g: sampler;
@group(2) @binding(4) var t_albedo: texture_2d<f32>;
@group(2) @binding(5) var s_albedo: sampler;

struct RenderSettings {
    cel_shading_factor: f32,
    p0: f32, p1: f32, p2: f32,
    p3: f32, p4: f32, p5: f32, p6: f32,
};
@group(3) @binding(0) var<uniform> settings: RenderSettings;

struct LightingVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_lighting(@builtin(vertex_index) in_vertex_index: u32) -> LightingVertexOutput {
    var out: LightingVertexOutput;
    let x = f32(i32(in_vertex_index) & 1);
    let y = f32(i32(in_vertex_index >> 1));
    out.tex_coords = vec2<f32>(x * 2.0, 1.0 - y * 2.0);
    out.position = vec4<f32>(out.tex_coords * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

@fragment
fn fs_lighting(in: LightingVertexOutput) -> @location(0) vec4<f32> {
    // Sample G-Buffer
    let position_sampled = textureSample(t_position, s_position, in.tex_coords);
    let normal_sampled = textureSample(t_normal_g, s_normal_g, in.tex_coords);
    let albedo = textureSample(t_albedo, s_albedo, in.tex_coords);
    
    let position = position_sampled.xyz;
    let normal = normalize(normal_sampled.xyz);
    
    // Lighting Calculation (Blinn-Phong)
    let view_dir = normalize(camera_lighting.view_pos.xyz - position);
    let light_dir = normalize(light.position.xyz - position);
    
    // Diffuse
    var NdotL = max(dot(normal, light_dir), 0.0);
    
    // Cel Shading Logic
    if (settings.cel_shading_factor > 0.5) {
        let bands = 4.0;
        NdotL = floor(NdotL * bands) / bands;
    }
    
    let diffuse = NdotL * light.color.rgb;
    
    // Specular
    let half_dir = normalize(light_dir + view_dir);
    var NdotH = max(dot(normal, half_dir), 0.0);
    var spec = pow(NdotH, 32.0);
    
    // Cel Specular
    if (settings.cel_shading_factor > 0.5) {
        let spec_cutoff = 0.5;
        if (spec > spec_cutoff) {
            spec = 1.0;
        } else {
            spec = 0.0;
        }
    }
    
    let specular = spec * light.color.rgb; // * material.specular
    
    let ambient = vec3<f32>(0.1, 0.1, 0.1) * albedo.rgb;
    
    let result = (ambient + diffuse + specular) * albedo.rgb;
    
    return vec4<f32>(result, 1.0);
}
