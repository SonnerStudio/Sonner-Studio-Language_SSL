// Particle System Shader (Render)

struct Particle {
    pos: vec4<f32>, // xyz = position, w = life
    vel: vec4<f32>, // xyz = velocity, w = size?
    col: vec4<f32>, // rgba
};

// Use read access for rendering
@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@group(0) @binding(2) var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_in: u32,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    let p = particles[instance_index];
    
    // Quad vertices (generated)
    var pos = vec2<f32>(0.0);
    var uv = vec2<f32>(0.0);
    
    let idx = vertex_in % 6u;
    if (idx == 0u) { pos = vec2<f32>(-0.5, 0.5); uv = vec2<f32>(0.0, 0.0); }
    else if (idx == 1u) { pos = vec2<f32>(-0.5, -0.5); uv = vec2<f32>(0.0, 1.0); }
    else if (idx == 2u) { pos = vec2<f32>(0.5, -0.5); uv = vec2<f32>(1.0, 1.0); }
    else if (idx == 3u) { pos = vec2<f32>(-0.5, 0.5); uv = vec2<f32>(0.0, 0.0); }
    else if (idx == 4u) { pos = vec2<f32>(0.5, -0.5); uv = vec2<f32>(1.0, 1.0); }
    else { pos = vec2<f32>(0.5, 0.5); uv = vec2<f32>(1.0, 0.0); }
    
    var out: VertexOutput;
    out.tex_coords = uv;
    out.color = p.col;
    
    // Simple point rendering for now or View-Aligned Billboard
    // Let's just output World Pos + Offset
    let world_pos = p.pos.xyz + vec3<f32>(pos.x, pos.y, 0.0) * 0.2; // Fixed size
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    
    // Don't draw if dead
    if (p.pos.w <= 0.0) {
        out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Circle or Texture
    let dist = distance(in.tex_coords, vec2<f32>(0.5, 0.5));
    if (dist > 0.5) {
        discard;
    }
    return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange Fire
}
