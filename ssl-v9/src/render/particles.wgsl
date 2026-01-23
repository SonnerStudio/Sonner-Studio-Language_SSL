// Particle System Shader (Compute + Render)

struct Particle {
    pos: vec4<f32>, // xyz = position, w = life
    vel: vec4<f32>, // xyz = velocity, w = size?
    col: vec4<f32>, // rgba
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> uniforms: SimUniforms;

struct SimUniforms {
    delta_time: f32,
    seed: f32,
    emit_count: u32,
    total_particles: u32,
};

// Compute Shader
@compute @workgroup_size(64)
fn simulate(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= uniforms.total_particles) {
        return;
    }

    var p = particles[index];

    // Update Life
    p.pos.w -= uniforms.delta_time;

    // Simulate Physics
    if (p.pos.w > 0.0) {
        p.pos.x += p.vel.x * uniforms.delta_time;
        p.pos.y += p.vel.y * uniforms.delta_time;
        p.pos.z += p.vel.z * uniforms.delta_time;
        // Gravity
        p.vel.y -= 9.8 * uniforms.delta_time; 
    } else {
        // Respawn logic (naive random)
        // Ideally we use random function based on seed + index
        let rnd = hash(index + u32(uniforms.seed * 1000.0));
        p.pos.x = (rnd.x - 0.5) * 10.0;
        p.pos.y = 10.0;
        p.pos.z = (rnd.y - 0.5) * 10.0;
        p.pos.w = 5.0; // Life
        
        p.vel.x = (rnd.z - 0.5) * 2.0;
        p.vel.y = 5.0;
        p.vel.z = (rnd.w - 0.5) * 2.0;
    }

    particles[index] = p;
}

// Helper: Pseudo-random generator
fn hash(value: u32) -> vec4<f32> {
    var state = value * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    let res = (word >> 22u) ^ word;
    
    // Return 4 floats in [0, 1]
    let x = f32(res & 0xFFu) / 255.0;
    let y = f32((res >> 8u) & 0xFFu) / 255.0;
    let z = f32((res >> 16u) & 0xFFu) / 255.0;
    let w = f32((res >> 24u) & 0xFFu) / 255.0;
    return vec4<f32>(x, y, z, w);
}

// Render Shader (Instanced Billboards)
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) particle_pos: vec4<f32>, // from storage buffer used as vertex buffer? 
    // Wait, we probably stick to storage buffer reading in vertex shader or Vertex Buffer attributes.
    // Let's use Storage Buffer reading for modern approach (or attributes if we want standard pipeline).
    // Using Storage Buffer in Vertex Shader is cleaner for Compute Interop.
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@group(0) @binding(2) var<uniform> camera: CameraUniform;
// Note: We need correct binding slots. 
// Compute Group: 0 (Storage, Uniforms)
// Render Group: 0 (Storage - reuse?), 1 (Camera)?
// Let's assume we bind Storage at Render Group 1, Binding 0.

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
    
    // Billboard Logic (Face Camera)
    // Needs Camera View Matrix to extract Right/Up vectors.
    // Simplified: Just add to position in Screen Space? No, we need World Space Billboard.
    
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

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Circle or Texture
    let dist = distance(in.tex_coords, vec2<f32>(0.5, 0.5));
    if (dist > 0.5) {
        discard;
    }
    return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange Fire
}
