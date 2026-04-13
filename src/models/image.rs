//! Image model - represents uploaded photos for a scene

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// An image uploaded to a NeRF scene
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SceneImage {
    pub id: Uuid,
    pub scene_id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub width: i32,
    pub height: i32,
    pub camera_position_x: Option<f64>,
    pub camera_position_y: Option<f64>,
    pub camera_position_z: Option<f64>,
    pub camera_rotation_pitch: Option<f64>,
    pub camera_rotation_yaw: Option<f64>,
    pub uploaded_at: DateTime<Utc>,
}

/// Request to upload images (multipart form data handled by axum)
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadImagesRequest {
    pub scene_id: Uuid,
    pub camera_data: Option<Vec<CameraData>>,
}

/// Camera pose data for an image
#[derive(Debug, Serialize, Deserialize)]
pub struct CameraData {
    pub filename: String,
    pub position: [f64; 3],
    pub rotation: [f64; 2], // pitch, yaw
}

/// Image response with URLs
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResponse {
    pub id: Uuid,
    pub scene_id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub width: i32,
    pub height: i32,
    pub camera_position: Option<[f64; 3]>,
    pub camera_rotation: Option<[f64; 2]>,
    pub url: String,
    pub thumbnail_url: String,
    pub uploaded_at: DateTime<Utc>,
}

impl From<SceneImage> for ImageResponse {
    fn from(img: SceneImage) -> Self {
        Self {
            id: img.id,
            scene_id: img.scene_id,
            filename: img.filename,
            original_name: img.original_name,
            width: img.width,
            height: img.height,
            camera_position: img.camera_position_x.map(|x| {
                [
                    x,
                    img.camera_position_y.unwrap_or(0.0),
                    img.camera_position_z.unwrap_or(0.0),
                ]
            }),
            camera_rotation: img.camera_rotation_pitch.map(|p| {
                [p, img.camera_rotation_yaw.unwrap_or(0.0)]
            }),
            url: format!("/api/v1/images/{}/download", img.id),
            thumbnail_url: format!("/api/v1/images/{}/thumbnail", img.id),
            uploaded_at: img.uploaded_at,
        }
    }
}
