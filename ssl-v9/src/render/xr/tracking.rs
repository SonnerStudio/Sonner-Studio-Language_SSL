use cgmath::{Quaternion, Vector3, Matrix4};

/// 6DOF pose (position + orientation)
#[derive(Debug, Clone, Copy)]
pub struct XRPose {
    pub position: Vector3<f32>,
    pub orientation: Quaternion<f32>,
}

impl XRPose {
    pub fn identity() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
        }
    }
    
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * Matrix4::from(self.orientation)
    }
}

/// Controller input state
#[derive(Debug, Clone)]
pub struct XRController {
    pub hand: Hand,
    pub pose: XRPose,
    pub is_active: bool,
    
    // Buttons
    pub trigger: f32,           // 0.0 - 1.0
    pub grip: f32,              // 0.0 - 1.0
    pub thumbstick: (f32, f32), // -1.0 to 1.0
    pub button_a: bool,
    pub button_b: bool,
    pub button_x: bool,
    pub button_y: bool,
}

impl XRController {
    pub fn new(hand: Hand) -> Self {
        Self {
            hand,
            pose: XRPose::identity(),
            is_active: false,
            trigger: 0.0,
            grip: 0.0,
            thumbstick: (0.0, 0.0),
            button_a: false,
            button_b: false,
            button_x: false,
            button_y: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

/// Hand tracking joint
#[derive(Debug, Clone, Copy)]
pub struct HandJoint {
    pub joint_type: JointType,
    pub pose: XRPose,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    Palm,
    Wrist,
    ThumbMetacarpal,
    ThumbProximal,
    ThumbDistal,
    ThumbTip,
    IndexMetacarpal,
    IndexProximal,
    IndexIntermediate,
    IndexDistal,
    IndexTip,
    // ... (simplified, full list has 26 joints per hand)
}

/// Hand tracking state
#[derive(Debug, Clone)]
pub struct HandTracking {
    pub hand: Hand,
    pub is_active: bool,
    pub joints: Vec<HandJoint>,
}

impl HandTracking {
    pub fn new(hand: Hand) -> Self {
        Self {
            hand,
            is_active: false,
            joints: Vec::new(),
        }
    }
    
    pub fn get_joint(&self, joint_type: JointType) -> Option<&HandJoint> {
        self.joints.iter().find(|j| j.joint_type == joint_type)
    }
}

/// Gesture recognition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gesture {
    None,
    Point,
    Grab,
    Pinch,
    ThumbsUp,
    OpenPalm,
}

pub fn recognize_gesture(hand: &HandTracking) -> Gesture {
    if !hand.is_active || hand.joints.is_empty() {
        return Gesture::None;
    }
    
    // Simplified gesture recognition
    // Full implementation would analyze joint angles and positions
    
    Gesture::None
}
