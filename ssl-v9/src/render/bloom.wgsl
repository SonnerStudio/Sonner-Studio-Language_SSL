// Bloom Post-Processing Effect
// Bright-pass filter + Gaussian blur + additive combine

struct BloomUniforms {
    threshold: f32,
    intensity: f32,
    blur_radius: f32,
    _padding: f32,
}

// Unified Bindings to match bloom.rs BindGroupLayout
@group(0) @binding(0) var t_input: texture_2d<f32>;
@group(0) @binding(1) var s_input: sampler;
@group(0) @binding(2) var<uniform> bloom: BloomUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_fullscreen(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Fullscreen triangle
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    out.position = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, 1.0 - y);
    return out;
}

// ============ BRIGHT PASS ============
@fragment
fn fs_bright_pass(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_input, s_input, in.tex_coords).rgb;
    
    // Luminance calculation (Rec. 709)
    let brightness = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    
    if (brightness > bloom.threshold) {
        return vec4<f32>(color * (brightness - bloom.threshold), 1.0);
    }
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

// ============ GAUSSIAN BLUR ============
fn get_gaussian_weight(index: i32) -> f32 {
    switch (index) {
        case 0: { return 0.227027; }
        case 1: { return 0.1945946; }
        case 2: { return 0.1216216; }
        case 3: { return 0.054054; }
        case 4: { return 0.016216; }
        default: { return 0.0; }
    }
}

// Horizontal blur
@fragment
fn fs_blur_h(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(t_input));
    var result = vec3<f32>(0.0);
    
    // Center sample
    result += textureSample(t_input, s_input, in.tex_coords).rgb * get_gaussian_weight(0);
    
    // Horizontal samples
    for (var i = 1; i < 5; i++) {
        let offset = f32(i) * texel_size.x * bloom.blur_radius;
        let weight = get_gaussian_weight(i);
        
        result += textureSample(t_input, s_input, in.tex_coords + vec2<f32>(offset, 0.0)).rgb * weight;
        result += textureSample(t_input, s_input, in.tex_coords - vec2<f32>(offset, 0.0)).rgb * weight;
    }
    
    return vec4<f32>(result, 1.0);
}

// Vertical blur
@fragment
fn fs_blur_v(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(t_input));
    var result = vec3<f32>(0.0);
    
    // Center sample
    result += textureSample(t_input, s_input, in.tex_coords).rgb * get_gaussian_weight(0);
    
    // Vertical samples
    for (var i = 1; i < 5; i++) {
        let offset = f32(i) * texel_size.y * bloom.blur_radius;
        let weight = get_gaussian_weight(i);
        
        result += textureSample(t_input, s_input, in.tex_coords + vec2<f32>(0.0, offset)).rgb * weight;
        result += textureSample(t_input, s_input, in.tex_coords - vec2<f32>(0.0, offset)).rgb * weight;
    }
    
    return vec4<f32>(result, 1.0);
}

// ============ COMBINE (Unused by BloomPass) ============
// Stubbed to pass validation (uses same binding layout)
@fragment
fn fs_bloom_combine(in: VertexOutput) -> @location(0) vec4<f32> {
    // This pipeline is created but never used in render().
    // We just return input to be safe and valid.
    let val = textureSample(t_input, s_input, in.tex_coords);
    return val;
}
