
// Depth of Field Shader

struct GlobalUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    resolution: vec2<f32>,
    time: f32,
    padding: f32,
}

struct DOFUniforms {
    focus_distance: f32,
    focus_range: f32,
    bokeh_radius: f32,
    padding: f32,
}

@group(0) @binding(0) var t_scene: texture_2d<f32>;
@group(0) @binding(1) var s_scene: sampler;
@group(0) @binding(2) var t_depth: texture_depth_2d;
@group(0) @binding(3) var s_depth: sampler; // Non-filtering sampler for depth
@group(0) @binding(4) var<uniform> dof: DOFUniforms;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in.vertex_index) & 1);
    let y = f32(i32(in.vertex_index) >> 1);
    out.tex_coords = vec2<f32>(x * 2.0, 1.0 - y * 2.0);
    out.clip_position = vec4<f32>(x * 4.0 - 1.0, 1.0 - y * 4.0, 0.0, 1.0);
    return out;
}

fn linearize_depth(depth: f32) -> f32 {
    let z_near = 0.1;
    let z_far = 100.0;
    return (2.0 * z_near) / (z_far + z_near - depth * (z_far - z_near));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let depth_raw = textureSample(t_depth, s_depth, in.tex_coords); // Depth is f32
    let depth = linearize_depth(depth_raw);
    
    // Calculate Circle of Confusion
    // depth is 0..1 linear? wait, linearize_depth usually returns view space Z or 0..1.
    // Standard perspective projection depth:
    // z_ndc = 2.0 * depth - 1.0;
    // z_view = (2.0 * near * far) / (far + near - z_ndc * (far - near));
    // Let's assume standard linearization or tune it.
    // If linearize_depth returns 0..1 range:
    
    let coc = abs(depth - dof.focus_distance) / dof.focus_range;
    let coc_clamped = clamp(coc, 0.0, 1.0);
    
    if (coc_clamped < 0.05) {
        // In focus
        return textureSample(t_scene, s_scene, in.tex_coords);
    }
    
    // Hexagonal Bokeh Blur
    let texel_size = 1.0 / vec2<f32>(textureDimensions(t_scene));
    var result = vec3<f32>(0.0);
    var total_weight = 0.0;
    
    let num_samples = 16;
    let golden_angle = 2.39996323;
    
    for (var i = 0; i < num_samples; i++) {
        let angle = f32(i) * golden_angle;
        let radius = sqrt(f32(i) / f32(num_samples)) * coc_clamped * dof.bokeh_radius;
        let offset = vec2<f32>(cos(angle), sin(angle)) * radius; // scale offset?
        // Adjust for aspect ratio if needed, or just pixel space
        let uv_offset = offset * texel_size;
        
        let sample_color = textureSample(t_scene, s_scene, in.tex_coords + uv_offset).rgb;
        result += sample_color;
        total_weight += 1.0;
    }
    
    return vec4<f32>(result / total_weight, 1.0);
}
