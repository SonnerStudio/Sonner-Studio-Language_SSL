// Screen Space Ambient Occlusion (SSAO) Shader

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct SSAOUniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    kernel_size: i32,   // e.g. 64
    radius: f32,        // e.g. 0.5
    bias: f32,          // e.g. 0.025
    power: f32,         // e.g. 2.0 (contrast)
    noise_scale: vec2<f32>, // screen_width/4, screen_height/4
}

@group(0) @binding(0) var<uniform> uniforms: SSAOUniforms;
@group(0) @binding(1) var t_depth: texture_depth_2d;
@group(0) @binding(2) var s_depth: sampler;
@group(0) @binding(3) var t_noise: texture_2d<f32>;  // 4x4 noise texture
@group(0) @binding(4) var s_noise: sampler;
@group(0) @binding(5) var<uniform> kernel: array<vec4<f32>, 64>; // Samples

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) & 1);
    let y = f32(i32(in_vertex_index >> 1));
    out.tex_coords = vec2<f32>(x * 2.0, 1.0 - y * 2.0);
    out.position = vec4<f32>(out.tex_coords * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

// Reconstruct View Space Position from Depth
fn view_pos_from_depth(tex_coord: vec2<f32>, depth: f32) -> vec3<f32> {
    let ndc = vec3<f32>(tex_coord.x * 2.0 - 1.0, (1.0 - tex_coord.y) * 2.0 - 1.0, depth);
    let clip_pos = vec4<f32>(ndc, 1.0);
    let view_pos = uniforms.inverse_projection * clip_pos;
    return view_pos.xyz / view_pos.w;
}

@fragment
fn fs_ssao(in: VertexOutput) -> @location(0) f32 {
    let depth = textureSample(t_depth, s_depth, in.tex_coords);
    if (depth >= 1.0) { return 1.0; } // Background is not occluded
    
    let frag_pos = view_pos_from_depth(in.tex_coords, depth);
    
    // Sample normal (assuming we have a normal texture, or reconstruct from depth)
    // For now, let's assume we can reconstruct normal from depth if normal texture is missing,
    // but typically SSAO needs good normals.
    // If t_normal is empty/white, we need to reconstruct.
    // For this implementation, let's assume valid normals are provided or we reconstruct.
    // var normal = textureSample(t_normal, s_normal, in.tex_coords).xyz;
    // normal = normalize(normal * 2.0 - 1.0); // If stored as [0,1]
    
    // Efficient Normal Reconstruction from Depth (derivative-based)
    let dx = dpdx(frag_pos);
    let dy = dpdy(frag_pos);
    let normal = normalize(cross(dx, dy));

    // Random rotation vector
    let noise_scale = uniforms.noise_scale;
    let random_vec = textureSample(t_noise, s_noise, in.tex_coords * noise_scale).xyz;
    
    // Create TBN matrix
    let tangent = normalize(random_vec - normal * dot(random_vec, normal));
    let bitangent = cross(normal, tangent);
    let tbn = mat3x3<f32>(tangent, bitangent, normal);
    
    var occlusion = 0.0;
    
    for (var i = 0; i < uniforms.kernel_size; i++) {
        var sample_pos = tbn * kernel[i].xyz; // From tangent to view-space
        sample_pos = frag_pos + sample_pos * uniforms.radius;
        
        // Project sample position to screen space
        var offset = vec4<f32>(sample_pos, 1.0);
        offset = uniforms.projection_matrix * offset;
        offset.x /= offset.w;
        offset.y /= offset.w; // Perspective divide
        offset.x = offset.x * 0.5 + 0.5;
        offset.y = offset.y * -0.5 + 0.5; // Flip Y (WGPU uses top-left origin?) -> WGPU NDC is Y-up? 
        // WGPU: Y-up in NDC (-1 bottom, 1 top), but texture coords (0,0) is top-left usually for simple quads?
        
        let sample_depth_raw = textureSample(t_depth, s_depth, offset.xy);
        let sample_depth = view_pos_from_depth(offset.xy, sample_depth_raw).z;
        
        // Range check & Accumulate
        let range_check = smoothstep(0.0, 1.0, uniforms.radius / abs(frag_pos.z - sample_depth));
        
        // Check if sample depth is closer to camera (more positive in view space Z negative? No, view space Z is negative usually)
        // WGPU View Space: Forward is -Z. 
        // If sample_depth >= sample_pos.z + bias -> Occluded
        // (Assuming Z is negative, so closer is larger number e.g. -5 > -10)
        
        if (sample_depth >= sample_pos.z + uniforms.bias) {
            occlusion += 1.0 * range_check;
        }
    }
    
    occlusion = 1.0 - (occlusion / f32(uniforms.kernel_size));
    return pow(occlusion, uniforms.power);
}

// ----------------------------------------------------------------------------
// Simple Blur Shader to reduce noise
// ----------------------------------------------------------------------------

@group(0) @binding(0) var t_ssao: texture_2d<f32>;
@group(0) @binding(1) var s_ssao: sampler;

@fragment
fn fs_blur(in: VertexOutput) -> @location(0) f32 {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(t_ssao));
    var result = 0.0;
    
    for (var x = -2; x < 2; x++) {
        for (var y = -2; y < 2; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            result += textureSample(t_ssao, s_ssao, in.tex_coords + offset).r;
        }
    }
    
    return result / 16.0;
}
