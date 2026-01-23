struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct LightUniform {
    position: vec3<f32>,
    color: vec3<f32>,
    view_proj: mat4x4<f32>, // Light's ViewProj
};
@group(1) @binding(0) var<uniform> light: LightUniform;

struct ModelInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    model: ModelInput,
) -> @builtin(position) vec4<f32> {
    let model_matrix = mat4x4<f32>(
        model.model_matrix_0,
        model.model_matrix_1,
        model.model_matrix_2,
        model.model_matrix_3,
    );
    // Transform World Position to Light Space
    return light.view_proj * model_matrix * vec4<f32>(position, 1.0);
}
