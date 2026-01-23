// OpenXR Extended Reality Support
#![cfg(not(target_arch = "wasm32"))]

use cgmath::{Matrix4, Quaternion, Vector3, Point3};

pub mod session;
pub mod stereoscopic;
pub mod tracking;

pub use session::XRSession;
pub use stereoscopic::StereoscopicRenderer;
pub use tracking::{XRPose, XRController};

/// XR eye identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Eye {
    Left,
    Right,
}

/// XR device capabilities
#[derive(Debug, Clone)]
pub struct XRCapabilities {
    pub supports_hand_tracking: bool,
    pub supports_eye_tracking: bool,
    pub supports_passthrough: bool,
    pub refresh_rates: Vec<f32>,
    pub resolution_per_eye: (u32, u32),
}

/// XR frame data
#[derive(Debug, Clone)]
pub struct XRFrame {
    pub predicted_display_time: i64,
    pub views: Vec<XRView>,
}

/// Individual eye view
#[derive(Debug, Clone)]
pub struct XRView {
    pub eye: Eye,
    pub pose: XRPose,
    pub fov: XRFov,
    pub viewport: (u32, u32, u32, u32), // x, y, width, height
}

/// Field of View
#[derive(Debug, Clone, Copy)]
pub struct XRFov {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}

impl XRFov {
    pub fn to_projection_matrix(&self, near: f32, far: f32) -> Matrix4<f32> {
        let tan_left = self.angle_left.tan();
        let tan_right = self.angle_right.tan();
        let tan_up = self.angle_up.tan();
        let tan_down = self.angle_down.tan();

        let tan_width = tan_right - tan_left;
        let tan_height = tan_up - tan_down;

        Matrix4::new(
            2.0 / tan_width, 0.0, 0.0, 0.0,
            0.0, 2.0 / tan_height, 0.0, 0.0,
            (tan_right + tan_left) / tan_width, (tan_up + tan_down) / tan_height, -(far + near) / (far - near), -1.0,
            0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0,
        )
    }
}

/// Input source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSource {
    Controller,
    Hand,
    Gaze,
}

/// Haptic feedback parameters
#[derive(Debug, Clone, Copy)]
pub struct HapticPulse {
    pub amplitude: f32,    // 0.0 - 1.0
    pub duration_ms: u64,
    pub frequency: f32,    // Hz
}
