
use cgmath::{InnerSpace, Matrix4, Vector3, SquareMatrix, vec3};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vector3<f32>,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vector3<f32>, distance: f32) -> Self {
        Self { normal, distance }
    }

    pub fn normalize(&mut self) {
        let mag = self.normal.magnitude();
        if mag > 0.0 {
            self.normal /= mag;
            self.distance /= mag;
        }
    }

    pub fn distance_to_point(&self, point: Vector3<f32>) -> f32 {
        self.normal.dot(point) + self.distance
    }
}

pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    pub fn from_matrix(m: Matrix4<f32>) -> Self {
        // Extraction based on Gribb/Hartmann method
        // Row 4 (w) +/- Row i
        
        let mut planes = [
            // Left
            Plane::new(
                vec3(m.w.x + m.x.x, m.w.y + m.x.y, m.w.z + m.x.z),
                m.w.w + m.x.w
            ),
            // Right
            Plane::new(
                vec3(m.w.x - m.x.x, m.w.y - m.x.y, m.w.z - m.x.z),
                m.w.w - m.x.w
            ),
            // Bottom
            Plane::new(
                vec3(m.w.x + m.y.x, m.w.y + m.y.y, m.w.z + m.y.z),
                m.w.w + m.y.w
            ),
            // Top
            Plane::new(
                vec3(m.w.x - m.y.x, m.w.y - m.y.y, m.w.z - m.y.z),
                m.w.w - m.y.w
            ),
            // Near
            Plane::new(
                vec3(m.w.x + m.z.x, m.w.y + m.z.y, m.w.z + m.z.z),
                m.w.w + m.z.w
            ),
            // Far
            Plane::new(
                vec3(m.w.x - m.z.x, m.w.y - m.z.y, m.w.z - m.z.z),
                m.w.w - m.z.w
            ),
        ];

        for plane in &mut planes {
            plane.normalize();
        }

        Self { planes }
    }

    pub fn contains_sphere(&self, center: Vector3<f32>, radius: f32) -> bool {
        for plane in &self.planes {
            // Distance positive if inside (assuming normals point inward? Standard is usually inward)
            // Wait, Gribb/Hartmann usually extracts such that normals point INWARD.
            // If distance < -radius, it's completely outside.
            if plane.distance_to_point(center) < -radius {
                return false;
            }
        }
        true
    }
}
