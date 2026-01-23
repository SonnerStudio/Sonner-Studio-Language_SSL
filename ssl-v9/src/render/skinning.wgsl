struct BoneMatrices {
    matrices: array<mat4x4<f32>>,
}

@group(2) @binding(0)
var<storage, read> bone_matrices: BoneMatrices;

fn get_bone_matrix(joints: vec4<u32>, weights: vec4<f32>, bone_offset: u32) -> mat4x4<f32> {
    let m0 = bone_matrices.matrices[bone_offset + joints.x];
    let m1 = bone_matrices.matrices[bone_offset + joints.y];
    let m2 = bone_matrices.matrices[bone_offset + joints.z];
    let m3 = bone_matrices.matrices[bone_offset + joints.w];

    return m0 * weights.x + m1 * weights.y + m2 * weights.z + m3 * weights.w;
}

fn skin_position(position: vec3<f32>, skin_matrix: mat4x4<f32>) -> vec4<f32> {
    return skin_matrix * vec4<f32>(position, 1.0);
}

fn skin_normal(normal: vec3<f32>, skin_matrix: mat4x4<f32>) -> vec3<f32> {
    // For normals, we need the inverse transpose if there is non-uniform scaling.
    // However, for pure rotation/translation, the matrix itself (or its 3x3 part) works.
    // If we assume no skew/non-uniform scale in bones, we can just use the 3x3 rotation.
    let rot_mat = mat3x3<f32>(
        skin_matrix[0].xyz,
        skin_matrix[1].xyz,
        skin_matrix[2].xyz
    );
    return normalize(rot_mat * normal);
}
