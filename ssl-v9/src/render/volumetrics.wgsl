// Volumetric Lighting Shader (God Rays)

struct VolumetricUniforms {
    light_pos: vec4<f32>,        // Light position in world space
    light_color: vec4<f32>,      // Light color and intensity
    scattering: f32,             // Scattering coefficient
    density: f32,                // Fog/volume density
    num_samples: u32,            // Raymarching samples
    max_distance: f32,           // Max raymarch distance
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<uniform> volumetric: VolumetricUniforms;

@group(1) @binding(0) var depth_texture: texture_2d<f32>;
@group(1) @binding(1) var depth_sampler: sampler;
@group(1) @binding(2) var shadow_map: texture_depth_2d;
@group(1) @binding(3) var shadow_sampler: sampler_comparison;

// 3D Noise function for procedural fog
fn hash(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise3d(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(
        mix(
            mix(hash(i + vec3<f32>(0.0, 0.0, 0.0)), hash(i + vec3<f32>(1.0, 0.0, 0.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 0.0)), hash(i + vec3<f32>(1.0, 1.0, 0.0)), u.x),
            u.y
        ),
        mix(
            mix(hash(i + vec3<f32>(0.0, 0.0, 1.0)), hash(i + vec3<f32>(1.0, 0.0, 1.0)), u.x),
            mix(hash(i + vec3<f32>(0.0, 1.0, 1.0)), hash(i + vec3<f32>(1.0, 1.0, 1.0)), u.x),
            u.y
        ),
        u.z
    );
}

fn fbm(p: vec3<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var pp = p;
    
    for (var i = 0; i < 3; i++) {
        value += amplitude * noise3d(pp);
        pp = pp * 2.0;
        amplitude *= 0.5;
    }
    
    return value;
}

// Vertex shader (fullscreen quad)
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(vertex_index) & 1);
    let y = f32(i32(vertex_index >> 1u));
    out.uv = vec2<f32>(x, 1.0 - y);
    out.position = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

// Reconstruct world position from depth
fn reconstruct_world_pos(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    // Convert to NDC
    let ndc = vec4<f32>(uv.x * 2.0 - 1.0, (1.0 - uv.y) * 2.0 - 1.0, depth, 1.0);
    
    // Inverse view-projection
    // We need inverse matrices - for now use approximation
    // In real implementation, pass inv_view_proj as uniform
    
    // Simplified: assume we have view_pos and can trace from there
    return camera.view_pos.xyz;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let depth = textureSample(depth_texture, depth_sampler, in.uv).r;
    
    // Ray direction from camera to pixel
    let ndc = vec2<f32>(in.uv.x * 2.0 - 1.0, (1.0 - in.uv.y) * 2.0 - 1.0);
    let aspect = camera.resolution.x / camera.resolution.y;
    let ray_dir = normalize(vec3<f32>(ndc.x * aspect, ndc.y, -1.0));
    
    let ray_origin = camera.view_pos.xyz;
    let ray_end_dist = min(depth * 100.0, volumetric.max_distance); // Scale depth
    
    // Raymarch
    let step_size = ray_end_dist / f32(volumetric.num_samples);
    var accumulated_light = vec3<f32>(0.0);
    var transmittance = 1.0;
    
    for (var i = 0u; i < volumetric.num_samples; i++) {
        let t = f32(i) * step_size;
        let sample_pos = ray_origin + ray_dir * t;
        
        // Sample density (using noise)
        let density_sample = fbm(sample_pos * 0.1) * volumetric.density;
        
        // Light calculation
        let to_light = normalize(volumetric.light_pos.xyz - sample_pos);
        let light_dist = length(volumetric.light_pos.xyz - sample_pos);
        
        // Simplified lighting (no shadow check for now)
        let light_attenuation = 1.0 / (1.0 + light_dist * 0.01);
        let inscatter = density_sample * volumetric.scattering * light_attenuation;
        
        accumulated_light += transmittance * inscatter * volumetric.light_color.rgb;
        transmittance *= exp(-density_sample * step_size);
    }
    
    return vec4<f32>(accumulated_light, 1.0 - transmittance);
}
