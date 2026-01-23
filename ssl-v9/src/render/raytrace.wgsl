struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_inv: mat4x4<f32>,
    proj_inv: mat4x4<f32>,
    origin: vec4<f32>,
};
@group(0) @binding(0) var<uniform> camera: CameraUniform;

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
};

@group(0) @binding(1) var output_texture: texture_storage_2d<rgba8unorm, write>;

// Simple scene data hardcoded or passed via buffer
const SPHERE_COUNT: i32 = 3;
var<private> spheres: array<Sphere, 3> = array<Sphere, 3>(
    Sphere(vec3<f32>(0.0, 0.0, -5.0), 1.0, vec3<f32>(1.0, 0.0, 0.0)),
    Sphere(vec3<f32>(2.0, 0.0, -5.0), 1.0, vec3<f32>(0.0, 1.0, 0.0)),
    Sphere(vec3<f32>(-2.0, 0.0, -5.0), 1.0, vec3<f32>(0.0, 0.0, 1.0))
);

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

struct HitInfo {
    hit: bool,
    dist: f32,
    point: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
};

fn intersect_sphere(ray: Ray, sphere: Sphere) -> HitInfo {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;

    var info: HitInfo;
    info.hit = false;

    if (discriminant > 0.0) {
        let t = (-b - sqrt(discriminant)) / (2.0 * a);
        if (t > 0.001) {
            info.hit = true;
            info.dist = t;
            info.point = ray.origin + t * ray.direction;
            info.normal = normalize(info.point - sphere.center);
            info.color = sphere.color;
        }
    }
    return info;
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(output_texture);
    let coords = vec2<i32>(global_id.xy);

    if (coords.x >= i32(dims.x) || coords.y >= i32(dims.y)) {
        return;
    }

    let uv = vec2<f32>(coords) / vec2<f32>(f32(dims.x), f32(dims.y)) * 2.0 - 1.0;
    
    // Generate Ray from Camera
    // Simple perspective projection inversion
    // Note: Use ViewInv/ProjInv for proper camera implementation
    let aspect_ratio = f32(dims.x) / f32(dims.y);
    // Hardcoded ray gen for demo if uniforms not fully set
    let ray_dir = normalize(vec3<f32>(uv.x * aspect_ratio, -uv.y, -1.0)); 
    var ray = Ray(vec3<f32>(0.0, 0.0, 5.0), ray_dir);

    // Trace
    var closest_hit: HitInfo;
    closest_hit.dist = 1e30;
    closest_hit.hit = false;

    for (var i = 0; i < SPHERE_COUNT; i++) {
        let hit = intersect_sphere(ray, spheres[i]);
        if (hit.hit && hit.dist < closest_hit.dist) {
            closest_hit = hit;
        }
    }

    var color = vec3<f32>(0.0, 0.0, 0.0);
    if (closest_hit.hit) {
        // Simple lighting
        let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
        let diffuse = max(dot(closest_hit.normal, light_dir), 0.1);
        color = closest_hit.color * diffuse;
    } else {
        // Sky gradient
        color = vec3<f32>(0.1, 0.1, 0.2) * (uv.y * 0.5 + 0.5);
    }

    textureStore(output_texture, coords, vec4<f32>(color, 1.0));
}
