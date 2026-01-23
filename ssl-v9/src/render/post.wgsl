@group(0) @binding(0) var t_hdr: texture_2d<f32>;
@group(0) @binding(1) var s_hdr: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Full screen triangle
    var pos = vec2<f32>(-1.0, 1.0);
    if (in_vertex_index == 1u) {
        pos = vec2<f32>(3.0, 1.0);
    } else if (in_vertex_index == 2u) {
        pos = vec2<f32>(-1.0, -3.0);
    }
    
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.tex_coords = pos * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5, 0.5);
    return out;
}

// FXAA (Fast Approximate Anti-Aliasing) - N64 Hardware AA Emulation
fn fxaa(uv: vec2<f32>, tex_size: vec2<f32>) -> vec3<f32> {
    let inv_tex_size = 1.0 / tex_size;
    
    // Sample neighbors
    let rgbNW = textureSample(t_hdr, s_hdr, uv + vec2<f32>(-1.0, -1.0) * inv_tex_size).rgb;
    let rgbNE = textureSample(t_hdr, s_hdr, uv + vec2<f32>(1.0, -1.0) * inv_tex_size).rgb;
    let rgbSW = textureSample(t_hdr, s_hdr, uv + vec2<f32>(-1.0, 1.0) * inv_tex_size).rgb;
    let rgbSE = textureSample(t_hdr, s_hdr, uv + vec2<f32>(1.0, 1.0) * inv_tex_size).rgb;
    let rgbM = textureSample(t_hdr, s_hdr, uv).rgb;
    
    // Luma calculation
    let luma = vec3<f32>(0.299, 0.587, 0.114);
    let lumaNW = dot(rgbNW, luma);
    let lumaNE = dot(rgbNE, luma);
    let lumaSW = dot(rgbSW, luma);
    let lumaSE = dot(rgbSE, luma);
    let lumaM = dot(rgbM, luma);
    
    let lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
    let lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));
    
    let range = lumaMax - lumaMin;
    
    if (range < 0.05) {
        return rgbM; // No aliasing detected
    }
    
    // Simple blur for edges
    return (rgbNW + rgbNE + rgbSW + rgbSE + rgbM) / 5.0;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // N64 Hardware Anti-Aliasing (Coverage AA emulated via FXAA)
    let tex_size = textureDimensions(t_hdr);
    let hdr_color = fxaa(in.tex_coords, vec2<f32>(f32(tex_size.x), f32(tex_size.y)));
    
    // ACES Tone Mapping
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    
    let mapped = clamp((hdr_color * (a * hdr_color + b)) / (hdr_color * (c * hdr_color + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
    
    // Gamma correction
    let gamma = 2.2;
    var final_color = pow(mapped, vec3<f32>(1.0 / gamma));
    
    // N64 VI Dithering (Simulated 5-5-5-1 format)
    // Bayer Matrix 4x4
    let dither_threshold_matrix = array<mat4x4<f32>, 1>(
        mat4x4<f32>(
           1.0,  9.0,  3.0, 11.0,
           13.0, 5.0, 15.0,  7.0,
           4.0, 12.0,  2.0, 10.0,
           16.0, 8.0, 14.0,  6.0
        )
    );
    
    let x = u32(in.position.x) % 4u;
    let y = u32(in.position.y) % 4u;
    let dither_val = (dither_threshold_matrix[0][x][y] / 17.0) - 0.5;
    
    // Quantize to 5 bits (32 levels)
    let colors = 32.0;
    final_color = floor(final_color * colors + dither_val) / colors;
    
    return vec4<f32>(final_color, 1.0);
}
