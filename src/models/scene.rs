//! Scene model - represents a 3D scene being built from photos

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A NeRF scene containing photos and trained model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scene {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: SceneStatus,
    pub image_count: i32,
    pub training_progress: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Scene processing status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SceneStatus {
    /// Initial state - photos uploaded but not processed
    Pending,
    /// Currently training the NeRF model
    Training,
    /// Model trained and ready for rendering
    Ready,
    /// Error occurred during processing
    Error(String),
}

impl Default for SceneStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Request to create a new scene
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSceneRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Response with scene details
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: SceneStatus,
    pub image_count: i32,
    pub training_progress: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub image_urls: Vec<String>,
}

impl From<Scene> for SceneResponse {
    fn from(scene: Scene) -> Self {
        Self {
            id: scene.id,
            name: scene.name,
            description: scene.description,
            status: scene.status,
            image_count: scene.image_count,
            training_progress: scene.training_progress,
            created_at: scene.created_at,
            updated_at: scene.updated_at,
            image_urls: Vec::new(), // Will be populated by handler
        }
    }
}

/// Update scene request
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSceneRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
