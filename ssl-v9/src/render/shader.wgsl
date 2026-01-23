// Vertex Shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
    resolution: vec2<f32>,
    flags: u32, // Bit 0: Retro, Bit 1: Fog
    _padding: u32,
    fog_color: vec4<f32>, // RGB + Mode
    fog_params: vec4<f32>, // start, end, density, _padding
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;

// N64 Hardware Fog (RDP)
fn apply_fog(color: vec3<f32>, view_distance: f32) -> vec3<f32> {
    if ((camera.flags & 2u) == 0u) {
        return color; // Fog disabled
    }
    
    let fog_mode = u32(camera.fog_color.w);
    var fog_factor = 0.0;
    
    if (fog_mode == 0u) {
        // Linear Fog
        fog_factor = clamp((camera.fog_params.y - view_distance) / (camera.fog_params.y - camera.fog_params.x), 0.0, 1.0);
    } else if (fog_mode == 1u) {
        // Exponential Fog
        fog_factor = exp(-camera.fog_params.z * view_distance);
        fog_factor = clamp(fog_factor, 0.0, 1.0);
    } else {
        // Exponential Squared Fog
        let exponent = camera.fog_params.z * view_distance;
        fog_factor = exp(-(exponent * exponent));
        fog_factor = clamp(fog_factor, 0.0, 1.0);
    }
    
    return mix(camera.fog_color.rgb, color, fog_factor);
}

struct Light {
    position: vec4<f32>,
    color: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(2) @binding(0) var<uniform> light: Light;
@group(2) @binding(1) var t_shadow: texture_depth_2d;
@group(2) @binding(2) var s_shadow: sampler_comparison;
@group(2) @binding(3) var t_env: texture_cube<f32>;
@group(2) @binding(4) var s_env: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) shadow_pos: vec4<f32>,
    @location(3) tangent_view_pos: vec3<f32>, // For Parallax if needed, or just light calc
    @location(4) tangent_light_pos: vec3<f32>,
    @location(5) tbn_0: vec3<f32>, // Passing TBN matrix columns
    @location(6) tbn_1: vec3<f32>,
    @location(7) tbn_2: vec3<f32>,
};

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    
    let world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    
    // TBN Matrix Calculation
    let T = normalize(normal_matrix * model.tangent);
    let B = normalize(normal_matrix * model.bitangent);
    let N = normalize(normal_matrix * model.normal);
    let TBN = mat3x3<f32>(T, B, N);
    
    // Pass TBN to fragment shader to transform normals from map to world space
    out.tbn_0 = T;
    out.tbn_1 = B;
    out.tbn_2 = N;

    // Shadow Coord
    let flip_correction = mat4x4<f32>(
        0.5, 0.0, 0.0, 0.0,
        0.0, -0.5, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.5, 0.5, 0.0, 1.0,
    );
    out.shadow_pos = flip_correction * light.view_proj * world_position;
    
    // Tangent Space Positions (Optional, useful for Parallax)
    let TBN_inv = transpose(TBN);
    out.tangent_light_pos = TBN_inv * light.position.xyz;
    out.tangent_view_pos  = TBN_inv * camera.view_pos.xyz;

    return out;
}

// Fragment Shader

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;
@group(1) @binding(2) var t_normal: texture_2d<f32>;
@group(1) @binding(3) var s_normal: sampler;

fn shadow_calculation(shadow_pos: vec4<f32>) -> f32 {
    let proj_coords = shadow_pos.xyz / shadow_pos.w;
    if (proj_coords.z > 1.0) {
        return 0.0;
    }
    let current_depth = proj_coords.z;
    
    var shadow = 0.0;
    let size = textureDimensions(t_shadow);
    let texel_size = vec2<f32>(1.0 / f32(size.x), 1.0 / f32(size.y));
    
    // PCF (Percentage Closer Filtering) 3x3
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let pcf_depth = textureSampleCompare(
                t_shadow, s_shadow,
                proj_coords.xy + vec2<f32>(f32(x), f32(y)) * texel_size,
                current_depth - 0.005
            );
            shadow += pcf_depth;
        }
    }
    shadow /= 9.0;
    
    return 1.0 - shadow;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let object_color: vec4<f32> = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    // Normal Mapping
    // 1. Sample Normal Map [0,1]
    let normal_map_val = textureSample(t_normal, s_normal, in.tex_coords).rgb;
    // 2. Expand to [-1, 1]
    let normal_tangent = normalize(normal_map_val * 2.0 - 1.0);
    // 3. Transform to World Space using TBN
    let TBN = mat3x3<f32>(in.tbn_0, in.tbn_1, in.tbn_2);
    let normal_world = normalize(TBN * normal_tangent);
    
    // Lighting Vectors
    let light_dir = normalize(light.position.xyz - in.world_position);
    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir + light_dir);
    let reflect_dir = reflect(-view_dir, normal_world);

    // IBL (Environment Reflection)
    let env_color = textureSample(t_env, s_env, reflect_dir).rgb;

    // Ambient (IBL + Constant)
    let ambient_strength = 0.1;
    let ambient_color = (light.color.xyz * ambient_strength) + (env_color * 0.2); // Blend IBL

    // Diffuse
    let diffuse_strength = max(dot(normal_world, light_dir), 0.0);
    let diffuse_color = light.color.xyz * diffuse_strength;

    // Specular (Blinn-Phong)
    let specular_strength = 0.5;
    let specular_per = pow(max(dot(normal_world, half_dir), 0.0), 32.0);
    let specular_color = light.color.xyz * specular_strength * specular_per;
    
    // Shadow
    let shadow = shadow_calculation(in.shadow_pos);

    var result = (ambient_color + (diffuse_color + specular_color) * shadow) * object_color.xyz;

    // N64 Hardware Fog
    let view_distance = length(camera.view_pos.xyz - in.world_position);
    result = apply_fog(result, view_distance);

    // N64 Alpha-Test (Cutout Transparency)
    let alpha_threshold = 0.5; // Adjustable
    if (object_color.a < alpha_threshold) {
        discard;
    }

    return vec4<f32>(result, object_color.a);
}
