//! Render model - represents rendering requests and results

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to render a novel view
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderRequest {
    pub scene_id: Uuid,
    /// Camera position [x, y, z]
    pub camera_position: [f64; 3],
    /// Camera rotation [pitch, yaw] in radians
    pub camera_rotation: [f64; 2],
    /// Output image width (default: 512)
    #[serde(default = "default_width")]
    pub width: i32,
    /// Output image height (default: 512)
    #[serde(default = "default_height")]
    pub height: i32,
}

fn default_width() -> i32 {
    512
}

fn default_height() -> i32 {
    512
}

/// Response with rendered image
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderResponse {
    pub scene_id: Uuid,
    pub camera_position: [f64; 3],
    pub camera_rotation: [f64; 2],
    pub width: i32,
    pub height: i32,
    pub image_url: String,
    pub render_time_ms: f64,
}

/// Request for a sequence of renders (animation)
#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceRenderRequest {
    pub scene_id: Uuid,
    pub frames: Vec<FrameData>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// Single frame in a sequence
#[derive(Debug, Serialize, Deserialize)]
pub struct FrameData {
    pub camera_position: [f64; 3],
    pub camera_rotation: [f64; 2],
}

/// Response for sequence render
#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceRenderResponse {
    pub scene_id: Uuid,
    pub frame_count: i32,
    pub width: i32,
    pub height: i32,
    pub video_url: String,
    pub total_render_time_ms: f64,
}
