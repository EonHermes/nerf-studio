//! Rendering API for novel view synthesis

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    models::{RenderRequest, RenderResponse},
    nerf::{CameraPose, NerfEngine, NerfModel},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::post(render))
}

/// Render a novel view from the trained NeRF model
pub async fn render(
    State(state): State<AppState>,
    Json(request): Json<RenderRequest>,
) -> Result<Json<RenderResponse>, AppError> {
    let start = Instant::now();

    // Verify scene exists and is ready
    let scene = sqlx::query_as::<_, crate::models::Scene>(
        "SELECT * FROM scenes WHERE id = ?",
    )
    .bind(request.scene_id)
    .fetch_optional(&state.db)
    .await?;

    let scene = match scene {
        Some(s) => s,
        None => return Err(AppError::NotFound("Scene not found")),
    };

    // Check if scene is ready for rendering
    if !matches!(scene.status, crate::models::SceneStatus::Ready) {
        return Err(AppError::BadRequest(
            "Scene must be in 'Ready' status to render".to_string(),
        ));
    }

    // Load or create NeRF model (in production, this would load from disk)
    let model = NerfModel::new(request.scene_id);
    
    if !model.is_ready() {
        return Err(AppError::BadRequest(
            "NeRF model not trained yet".to_string(),
        ));
    }

    // Create camera pose from request
    let camera_pose = CameraPose::new(request.camera_position, request.camera_rotation);

    // Initialize rendering engine
    let engine = NerfEngine::new(state.uploads_dir.clone());

    // Render the image
    let image_data = match engine.render(&model, &camera_pose, request.width as u32, request.height as u32) {
        Ok(data) => data,
        Err(e) => return Err(AppError::Internal(format!("Render failed: {}", e))),
    };

    // Save rendered image temporarily
    let render_id = Uuid::new_v4();
    let filename = format!("render_{}.png", render_id);
    let file_path = state.uploads_dir.join(&filename);
    
    std::fs::write(&file_path, &image_data)
        .map_err(|e| AppError::Internal(format!("Failed to save render: {}", e)))?;

    let render_time = start.elapsed().as_secs_f64() * 1000.0;

    info!(
        "Rendered novel view for scene {} at position [{}, {}, {}] in {:.2}ms",
        request.scene_id,
        request.camera_position[0],
        request.camera_position[1],
        request.camera_position[2],
        render_time
    );

    Ok(Json(RenderResponse {
        scene_id: request.scene_id,
        camera_position: request.camera_position,
        camera_rotation: request.camera_rotation,
        width: request.width,
        height: request.height,
        image_url: format!("/api/v1/render/download/{}", render_id),
        render_time_ms: render_time,
    }))
}

/// Download a rendered image
pub async fn download(
    State(state): State<AppState>,
    Path(render_id): Path<Uuid>,
) -> Result<(StatusCode, Vec<u8>), AppError> {
    let filename = format!("render_{}.png", render_id);
    let file_path = state.uploads_dir.join(&filename);

    let data = std::fs::read(&file_path)
        .map_err(|_| AppError::NotFound("Render not found"))?;

    Ok((StatusCode::OK, data))
}

/// Custom error type
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(&'static str),
    BadRequest(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(e) => {
                warn!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg).into_response()
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            AppError::Internal(msg) => {
                warn!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_request_serialization() {
        let request = RenderRequest {
            scene_id: Uuid::new_v4(),
            camera_position: [0.0, 0.0, 2.0],
            camera_rotation: [0.0, 0.0],
            width: 512,
            height: 512,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("scene_id"));
        assert!(json.contains("camera_position"));
    }
}
