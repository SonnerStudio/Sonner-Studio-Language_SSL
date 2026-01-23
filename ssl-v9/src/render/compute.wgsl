struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) {
        return;
    }

    var vPos = particles[index].position;
    var vVel = particles[index].velocity;
    
    // Simple Physics: Move
    vPos = vPos + vVel * 0.01;

    // Bounce off walls (-1.0 to 1.0)
    if (vPos.x < -1.0 || vPos.x > 1.0) {
        vVel.x = -vVel.x;
    }
    if (vPos.y < -1.0 || vPos.y > 1.0) {
        vVel.y = -vVel.y;
    }

    particles[index].position = vPos;
    particles[index].velocity = vVel;
}
