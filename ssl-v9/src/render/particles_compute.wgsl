// Particle System Shader (Compute)

struct Particle {
    pos: vec4<f32>, // xyz = position, w = life
    vel: vec4<f32>, // xyz = velocity, w = size?
    col: vec4<f32>, // rgba
};

struct SimUniforms {
    delta_time: f32,
    seed: f32,
    emit_count: u32,
    total_particles: u32,
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> uniforms: SimUniforms;

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
