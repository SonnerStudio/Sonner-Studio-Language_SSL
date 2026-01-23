// Simple Post-Processing Effects
// Vignette, Film Grain, Chromatic Aberration

@group(0) @binding(0) var t_scene: texture_2d<f32>;
@group(0) @binding(1) var s_scene: sampler;
@group(0) @binding(2) var<uniform> time: f32;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_fullscreen(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    
    out.position = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, 1.0 - y);
    
    return out;
}

// ============ VIGNETTE ============
// Darkens the edges of the screen

@fragment
fn fs_vignette(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_scene, s_scene, in.tex_coords).rgb;
    
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(in.tex_coords - center);
    
    // Vignette parameters
    let vignette_strength = 0.5;
    let vignette_extent = 0.7;
    
    let vignette = smoothstep(vignette_extent, 0.2, dist);
    let final_color = color * mix(1.0 - vignette_strength, 1.0, vignette);
    
    return vec4<f32>(final_color, 1.0);
}

// ============ FILM GRAIN ============
// Adds analog film grain noise

fn random(uv: vec2<f32>, seed: f32) -> f32 {
    return fract(sin(dot(uv + seed, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_film_grain(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_scene, s_scene, in.tex_coords).rgb;
    
    // Grain parameters
    let grain_strength = 0.08;
    
    // Generate noise (time-varying for animation)
    let grain = random(in.tex_coords, time) * 2.0 - 1.0;
    
    let final_color = color + grain * grain_strength;
    
    return vec4<f32>(final_color, 1.0);
}

// ============ CHROMATIC ABERRATION ============
// RGB color channel separation (lens aberration)

@fragment
fn fs_chromatic_aberration(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    
    // Aberration strength (stronger at edges)
    let aberration_strength = 0.003;
    let dist_from_center = length(in.tex_coords - center);
    let offset = (in.tex_coords - center) * aberration_strength * dist_from_center;
    
    // Sample each color channel with offset
    let r = textureSample(t_scene, s_scene, in.tex_coords - offset).r;
    let g = textureSample(t_scene, s_scene, in.tex_coords).g;
    let b = textureSample(t_scene, s_scene, in.tex_coords + offset).b;
    
    return vec4<f32>(r, g, b, 1.0);
}

// ============ COMBINED POST-PROCESSING ============
// Applies multiple effects in one pass

struct PostProcessUniforms {
    vignette_strength: f32,
    grain_strength: f32,
    aberration_strength: f32,
    time: f32,
}

@group(0) @binding(3) var<uniform> post_uniforms: PostProcessUniforms;

@fragment
fn fs_combined_post(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist_from_center = length(in.tex_coords - center);
    
    // Chromatic Aberration
    let offset = (in.tex_coords - center) * post_uniforms.aberration_strength * dist_from_center;
    let r = textureSample(t_scene, s_scene, in.tex_coords - offset).r;
    let g = textureSample(t_scene, s_scene, in.tex_coords).g;
    let b = textureSample(t_scene, s_scene, in.tex_coords + offset).b;
    var color = vec3<f32>(r, g, b);
    
    // Film Grain
    let grain = random(in.tex_coords, post_uniforms.time) * 2.0 - 1.0;
    color += grain * post_uniforms.grain_strength;
    
    // Vignette
    let vignette = smoothstep(0.7, 0.2, dist_from_center);
    color *= mix(1.0 - post_uniforms.vignette_strength, 1.0, vignette);
    
    return vec4<f32>(color, 1.0);
}
