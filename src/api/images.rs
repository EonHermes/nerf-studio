//! Image upload and management API

use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json, Router,
};
use image::imageops;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{models::SceneImage, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/upload", axum::routing::post(upload))
        .route("/:id/download", axum::routing::get(download))
        .route("/:id/thumbnail", axum::routing::get(thumbnail))
        .route("/:id", axum::routing::delete(delete))
}

/// Upload images to a scene
pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Vec<ImageUploadResponse>>, AppError> {
    let mut uploaded = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.bytes().await?;
        
        // Parse camera data if provided
        let camera_data = parse_camera_data(&name);

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let extension = get_extension(&field.file_name());
        let filename = format!("{}.{}", file_id, extension);
        
        // Save original image
        let file_path = state.uploads_dir.join(&filename);
        std::fs::write(&file_path, &data)?;

        // Process image to get dimensions and create thumbnail
        let img = image::load_from_memory(&data)
            .map_err(|e| AppError::Internal(format!("Failed to load image: {}", e)))?;
        
        let width = img.width() as i32;
        let height = img.height() as i32;

        // Create thumbnail (256px max dimension)
        let thumb_filename = format!("thumb_{}.{}", file_id, extension);
        let thumb_path = state.uploads_dir.join(&thumb_filename);
        
        let thumb = imageops::thumbnail(&img, 256);
        thumb.save(&thumb_path)?;

        // Get scene ID from filename convention (scene-{uuid}-image.png)
        let scene_id = camera_data
            .and_then(|c| c.scene_id)
            .unwrap_or_else(Uuid::new_v4);

        // Verify scene exists
        let scene_exists = sqlx::query("SELECT 1 FROM scenes WHERE id = ?")
            .bind(scene_id)
            .fetch_optional(&state.db)
            .await?;

        if scene_exists.is_none() {
            warn!("Scene {} not found for image upload", scene_id);
            // Continue anyway, just note the mismatch
        }

        // Save to database
        let image_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO scene_images 
            (id, scene_id, filename, original_name, width, height, 
             camera_position_x, camera_position_y, camera_position_z,
             camera_rotation_pitch, camera_rotation_yaw, uploaded_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(image_id)
        .bind(scene_id)
        .bind(&filename)
        .bind(name.clone())
        .bind(width)
        .bind(height)
        .bind(camera_data.as_ref().map(|c| c.position[0]))
        .bind(camera_data.as_ref().map(|c| c.position[1]))
        .bind(camera_data.as_ref().map(|c| c.position[2]))
        .bind(camera_data.as_ref().map(|c| c.rotation[0]))
        .bind(camera_data.as_ref().map(|c| c.rotation[1]))
        .bind(chrono::Utc::now())
        .execute(&state.db)
        .await?;

        // Update scene image count
        sqlx::query("UPDATE scenes SET image_count = image_count + 1, updated_at = ? WHERE id = ?")
            .bind(chrono::Utc::now())
            .bind(scene_id)
            .execute(&state.db)
            .await?;

        uploaded.push(ImageUploadResponse {
            id: image_id,
            scene_id,
            filename,
            original_name: name,
            width,
            height,
            url: format!("/api/v1/images/{}/download", image_id),
            thumbnail_url: format!("/api/v1/images/{}/thumbnail", image_id),
        });

        info!("Uploaded image: {} ({}x{})", name, width, height);
    }

    Ok(Json(uploaded))
}

/// Download an image
pub async fn download(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Vec<u8>), AppError> {
    let record = sqlx::query_as::<_, SceneImage>(
        "SELECT * FROM scene_images WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    let record = record.ok_or(AppError::NotFound("Image not found"))?;

    let file_path = state.uploads_dir.join(&record.filename);
    let data = std::fs::read(&file_path)
        .map_err(|_| AppError::NotFound("File not found on disk"))?;

    Ok((StatusCode::OK, data))
}

/// Get thumbnail for an image
pub async fn thumbnail(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Vec<u8>), AppError> {
    let record = sqlx::query_as::<_, SceneImage>(
        "SELECT * FROM scene_images WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    let record = record.ok_or(AppError::NotFound("Image not found"))?;

    let thumb_filename = format!("thumb_{}", record.filename);
    let file_path = state.uploads_dir.join(&thumb_filename);
    
    let data = std::fs::read(&file_path)
        .map_err(|_| AppError::NotFound("Thumbnail not found"))?;

    Ok((StatusCode::OK, data))
}

/// Delete an image
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let record = sqlx::query_as::<_, SceneImage>(
        "SELECT * FROM scene_images WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    let record = record.ok_or(AppError::NotFound("Image not found"))?;

    // Delete files
    let file_path = state.uploads_dir.join(&record.filename);
    let thumb_path = state.uploads_dir.join(format!("thumb_{}", record.filename));
    
    let _ = std::fs::remove_file(file_path);
    let _ = std::fs::remove_file(thumb_path);

    // Update scene image count and delete from DB
    sqlx::query(
        r#"
        UPDATE scenes SET image_count = image_count - 1, updated_at = ? WHERE id = ?
        "#,
    )
    .bind(chrono::Utc::now())
    .bind(record.scene_id)
    .execute(&state.db)
    .await?;

    sqlx::query("DELETE FROM scene_images WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    info!("Deleted image: {}", id);
    
    Ok(StatusCode::NO_CONTENT)
}

/// Response for successful upload
#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    pub id: Uuid,
    pub scene_id: Uuid,
    pub filename: String,
    pub original_name: String,
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub thumbnail_url: String,
}

/// Parse camera data from field name
/// Format: scene-{uuid}-image.png or image-{pitch}-{yaw}.png
fn parse_camera_data(field_name: &str) -> Option<CameraData> {
    // Try to extract scene ID from filename pattern
    let parts: Vec<&str> = field_name.split('-').collect();
    
    if parts.len() >= 2 && parts[0] == "scene" {
        if let Ok(scene_id) = Uuid::parse_str(parts[1]) {
            return Some(CameraData {
                scene_id: Some(scene_id),
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0],
            });
        }
    }

    None
}

fn get_extension(file_name: Option<&str>) -> String {
    file_name
        .and_then(|n| n.split('.').last())
        .unwrap_or("png")
        .to_string()
}

#[derive(Debug)]
struct CameraData {
    scene_id: Option<Uuid>,
    position: [f64; 3],
    rotation: [f64; 2],
}

/// Custom error type
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(&'static str),
    Internal(String),
    Io(std::io::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(e) => {
                warn!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error")).into_response()
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg).into_response()
            }
            AppError::Internal(msg) => {
                warn!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::Io(e) => {
                warn!("IO error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "File operation failed").into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_camera_data() {
        let data = parse_camera_data("scene-550e8400-e29b-41d4-a716-446655440000-image.png");
        assert!(data.is_some());
        
        let data = data.unwrap();
        assert_eq!(
            data.scene_id.unwrap().to_string(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension(Some("image.png")), "png");
        assert_eq!(get_extension(Some("photo.JPG")), "JPG");
        assert_eq!(get_extension(None), "png");
    }
}
