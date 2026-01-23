// Screen Space Global Illumination (SSGI)

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<f32>,
}

struct SSGIUniforms {
    num_samples: u32,
    max_distance: f32,
    intensity: f32,
    bias: f32,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(0) @binding(1) var<uniform> ssgi: SSGIUniforms;

// G-Buffer inputs
@group(1) @binding(0) var position_texture: texture_2d<f32>;
@group(1) @binding(1) var position_sampler: sampler;
@group(1) @binding(2) var normal_texture: texture_2d<f32>;
@group(1) @binding(3) var normal_sampler: sampler;
@group(1) @binding(4) var albedo_texture: texture_2d<f32>;
@group(1) @binding(5) var albedo_sampler: sampler;

// Random number generation
fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn random_dir(seed: vec2<f32>, n: vec3<f32>) -> vec3<f32> {
    // Generate random direction in hemisphere oriented by normal
    let phi = hash(seed) * 6.28318530718; // 2*PI
    let cos_theta = hash(seed + vec2<f32>(1.0, 1.0));
    let sin_theta = sqrt(1.0 - cos_theta * cos_theta);
    
    let h = vec3<f32>(
        cos(phi) * sin_theta,
        sin(phi) * sin_theta,
        cos_theta
    );
    
    // Orient to normal
    let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.999);
    let tangent = normalize(cross(up, n));
    let bitangent = cross(n, tangent);
    
    return normalize(tangent * h.x + bitangent * h.y + n * h.z);
}

// Vertex shader
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

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample G-Buffer
    let position = textureSample(position_texture, position_sampler, in.uv).xyz;
    let normal = normalize(textureSample(normal_texture, normal_sampler, in.uv).xyz);
    let albedo = textureSample(albedo_texture, albedo_sampler, in.uv).rgb;
    
    // Skip pixels with no geometry
    if (length(normal) < 0.1) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    
    var indirect_light = vec3<f32>(0.0);
    var total_weight = 0.0;
    
    // Sample hemisphere
    for (var i = 0u; i < ssgi.num_samples; i++) {
        let seed = in.uv * camera.resolution + vec2<f32>(f32(i), f32(i) * 1.618);
        let sample_dir = random_dir(seed, normal);
        
        // Ray march in screen space
        let ray_origin = position;
        let ray_step = sample_dir * ssgi.max_distance / 16.0;
        
        var found_intersection = false;
        var intersection_pos = vec3<f32>(0.0);
        var intersection_uv = vec2<f32>(0.0);
        
        for (var step = 1; step < 16; step++) {
            let sample_pos = ray_origin + ray_step * f32(step);
            
            // Project to screen space
            let clip_pos = camera.view_proj * vec4<f32>(sample_pos, 1.0);
            var ndc = clip_pos.xyz / clip_pos.w;
            let sample_uv = vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
            
            // Check if in screen bounds
            if (sample_uv.x < 0.0 || sample_uv.x > 1.0 || sample_uv.y < 0.0 || sample_uv.y > 1.0) {
                break;
            }
            
            // Sample depth at this screen position
            let buffer_pos = textureSample(position_texture, position_sampler, sample_uv).xyz;
            let distance_to_buffer = length(sample_pos - buffer_pos);
            
            // Check for intersection
            if (distance_to_buffer < ssgi.bias) {
                found_intersection = true;
                intersection_pos = buffer_pos;
                intersection_uv = sample_uv;
                break;
            }
        }
        
        if (found_intersection) {
            // Sample albedo at intersection
            let intersection_albedo = textureSample(albedo_texture, albedo_sampler, intersection_uv).rgb;
            let intersection_normal = normalize(textureSample(normal_texture, normal_sampler, intersection_uv).xyz);
            
            // Calculate contribution
            let to_intersection = normalize(intersection_pos - position);
            let distance_sq = dot(intersection_pos - position, intersection_pos - position);
            let attenuation = 1.0 / (1.0 + distance_sq * 0.1);
            
            // Lambertian diffuse
            let ndotl = max(dot(normal, to_intersection), 0.0);
            let contribution = intersection_albedo * ndotl * attenuation;
            
            indirect_light += contribution;
            total_weight += 1.0;
        }
    }
    
    if (total_weight > 0.0) {
        indirect_light /= total_weight;
    }
    
    return vec4<f32>(indirect_light * ssgi.intensity, 1.0);
}
