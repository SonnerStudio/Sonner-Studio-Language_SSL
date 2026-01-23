// Outline Post-Processing Shader

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) & 1);
    let y = f32(i32(in_vertex_index >> 1));
    out.tex_coords = vec2<f32>(x * 2.0, 1.0 - y * 2.0);
    out.position = vec4<f32>(out.tex_coords * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

@group(0) @binding(0) var t_depth: texture_depth_2d;
@group(0) @binding(1) var s_depth: sampler; // Not really used for depth load usually, but we might sample
@group(0) @binding(2) var t_normal: texture_2d<f32>;
@group(0) @binding(3) var s_normal: sampler;
@group(0) @binding(4) var t_color: texture_2d<f32>; // The scene color to draw outlines ON TOP of
@group(0) @binding(5) var s_color: sampler;

struct OutlineUniforms {
    screen_size: vec2<f32>,
    threshold_depth: f32,
    threshold_normal: f32,
    outline_thickness: f32,
    p0: f32, p1: f32, p2: f32,
};
@group(0) @binding(6) var<uniform> uniforms: OutlineUniforms;

fn get_depth(uv: vec2<f32>) -> f32 {
    return textureSample(t_depth, s_depth, uv);
}

fn get_normal(uv: vec2<f32>) -> vec3<f32> {
    return textureSample(t_normal, s_normal, uv).xyz;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let original_color = textureSample(t_color, s_color, in.tex_coords);
    
    // Simple Edge Detection (Roberts Cross or Sobel)
    // Using Sobel for better quality
    
    let texel_size = vec2<f32>(1.0 / uniforms.screen_size.x, 1.0 / uniforms.screen_size.y) * uniforms.outline_thickness;
    
    // Depth Edge
    let d_center = get_depth(in.tex_coords);
    let d_up = get_depth(in.tex_coords + vec2<f32>(0.0, texel_size.y));
    let d_right = get_depth(in.tex_coords + vec2<f32>(texel_size.x, 0.0));
    
    let depth_diff = abs(d_center - d_up) + abs(d_center - d_right);
    // Linearize depth? For now raw depth diff might work for close objects, but fails far away.
    // Let's assume raw depth is ok for a start.
    
    var edge_depth = 0.0;
    if (depth_diff > uniforms.threshold_depth) {
        edge_depth = 1.0;
    }
    
    // Normal Edge
    let n_center = get_normal(in.tex_coords);
    let n_up = get_normal(in.tex_coords + vec2<f32>(0.0, texel_size.y));
    let n_right = get_normal(in.tex_coords + vec2<f32>(texel_size.x, 0.0));
    
    let normal_diff = distance(n_center, n_up) + distance(n_center, n_right);
    
    var edge_normal = 0.0;
    if (normal_diff > uniforms.threshold_normal) {
        edge_normal = 1.0;
    }
    
    let edge = max(edge_depth, edge_normal);
    
    if (edge > 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black Outline
    }
    
    return original_color;
}
