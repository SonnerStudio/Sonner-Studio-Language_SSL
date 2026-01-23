// Screen Space Reflections (SSR) Shader

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct SSRUniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    max_steps: i32,
    step_size: f32,
    max_distance: f32, // Max ray distance not screen distance
    thickness: f32, // Thickness for acceptance
}

@group(0) @binding(0) var<uniform> uniforms: SSRUniforms;
@group(0) @binding(1) var t_depth: texture_depth_2d;
@group(0) @binding(2) var s_depth: sampler;
@group(0) @binding(3) var t_normal: texture_2d<f32>;
@group(0) @binding(4) var s_normal: sampler;
@group(0) @binding(5) var t_scene: texture_2d<f32>; // HDR Input
@group(0) @binding(6) var s_scene: sampler;

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

fn project_to_screen(view_pos: vec3<f32>) -> vec2<f32> {
    let clip_pos = uniforms.projection_matrix * vec4<f32>(view_pos, 1.0);
    let ndc = clip_pos.xy / clip_pos.w;
    return vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let depth = textureSample(t_depth, s_depth, in.tex_coords);
    if (depth >= 1.0) { return vec4<f32>(0.0); } // Skybox/Far plane

    let view_pos = view_pos_from_depth(in.tex_coords, depth);
    let normal_raw = textureSample(t_normal, s_normal, in.tex_coords).xyz;
    let normal = normalize(normal_raw * 2.0 - 1.0); // Assuming normals are [0,1] encoded

    // Reflection Vector in View Space
    let view_dir = normalize(view_pos);
    let reflect_dir = reflect(view_dir, normal);

    // Early exit if reflecting away from camera (facing backwards)?
    // Usually we want reflections even at glancing angles?
    
    // Ray Marching (Linear + Binary Search)
    var current_pos = view_pos;
    let step = reflect_dir * uniforms.step_size;
    
    var hit = false;
    var coords = vec2<f32>(0.0);
    
    for (var i = 0; i < uniforms.max_steps; i++) {
        current_pos += step;
        
        // Project to screen
        coords = project_to_screen(current_pos);
        
        // Check bounds
        if (coords.x < 0.0 || coords.x > 1.0 || coords.y < 0.0 || coords.y > 1.0) {
            break;
        }
        
        // Sample Depth
        let sample_depth = textureSample(t_depth, s_depth, coords);
        let sample_pos = view_pos_from_depth(coords, sample_depth);
        
        // Check Intersection
        // Ray is 'current_pos', Surface is 'sample_pos'
        // Since we are in View Space (Z negative), closer to camera is larger Z (e.g. -5 > -10)
        // Ray Z > Surface Z => Ray is behind surface? 
        // WGPU View Space: -Z forward.
        // If current_pos.z < sample_pos.z (Ray is more negative, i.e. further away) -> Hit?
        
        // Let's think:
        // Camera at 0. ViewDir -Z.
        // Surface at -10. Ray goes from -10 to -20.
        // Sample at -5.
        // Ray(-12) < Sample(-5). Behind.
        
        let depth_diff = current_pos.z - sample_pos.z; // -12 - (-5) = -7. 
        // If depth_diff < 0 (Ray is behind surface) AND depth_diff > -thickness (not too far behind)
        
        if (depth_diff < 0.0 && depth_diff > -uniforms.thickness) {
            hit = true;
            // Binary Search refinement here
            break;
        }
    }
    
    if (hit) {
        // Sample Scene Color
        let reflection_color = textureSample(t_scene, s_scene, coords).rgb;
        
        // Fresnel Fade
        let fresnel = pow(1.0 - max(dot(normal, -view_dir), 0.0), 3.0);
        
        // Edge Fade (Fade out near screen edges)
        let dist = abs(coords - vec2<f32>(0.5));
        let dx = smoothstep(0.4, 0.5, dist.x);
        let dy = smoothstep(0.4, 0.5, dist.y);
        let edge_fade = 1.0 - saturate(dx + dy);
        
        return vec4<f32>(reflection_color * fresnel * edge_fade, 1.0);
    }

    return vec4<f32>(0.0);
}
