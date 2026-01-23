// Deferred Shading - Lighting Pass
// --- LIGHTING PASS ---

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<f32>,
    flags: u32,
    _padding: u32,
    fog_color: vec4<f32>,
    fog_params: vec4<f32>,
};

struct LightUniform {
    position: vec4<f32>,
    color: vec4<f32>,
    view_proj: mat4x4<f32>, // For shadows?
    // params: intensity, range, etc.
};

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
