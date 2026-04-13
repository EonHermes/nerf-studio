//! Scene management API

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    models::{CreateSceneRequest, Scene, SceneResponse, SceneStatus, UpdateSceneRequest},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(list).post(create))
        .route("/:id", axum::routing::get(get).put(update).delete(delete))
        .route("/:id/images", axum::routing::get(get_images))
}

/// List all scenes
pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<SceneResponse>>, AppError> {
    let scenes = sqlx::query_as::<_, Scene>(
        r#"
        SELECT * FROM scenes 
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    let responses: Vec<SceneResponse> = scenes.into_iter().map(|s| s.into()).collect();
    
    Ok(Json(responses))
}

/// Create a new scene
pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateSceneRequest>,
) -> Result<Json<SceneResponse>, AppError> {
    let id = Uuid::new_v4();
    
    sqlx::query(
        r#"
        INSERT INTO scenes (id, name, description, status, image_count, training_progress, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id)
    .bind(&request.name)
    .bind(&request.description)
    .bind(SceneStatus::Pending.to_string())
    .bind(0i32)
    .bind(None::<f64>)
    .bind(chrono::Utc::now())
    .bind(chrono::Utc::now())
    .execute(&state.db)
    .await?;

    let scene = sqlx::query_as::<_, Scene>(
        "SELECT * FROM scenes WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    info!("Created new scene: {} ({})", request.name, id);
    
    Ok(Json(scene.into()))
}

/// Get a specific scene
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SceneResponse>, AppError> {
    let scene = sqlx::query_as::<_, Scene>(
        "SELECT * FROM scenes WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    let scene = scene.ok_or(AppError::NotFound("Scene not found"))?;
    
    Ok(Json(scene.into()))
}

/// Update a scene
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateSceneRequest>,
) -> Result<Json<SceneResponse>, AppError> {
    // Check if scene exists
    let existing = sqlx::query_as::<_, Scene>(
        "SELECT * FROM scenes WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    if existing.is_none() {
        return Err(AppError::NotFound("Scene not found"));
    }

    // Update fields that were provided
    if let Some(name) = &request.name {
        sqlx::query("UPDATE scenes SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(&state.db)
            .await?;
    }

    if let Some(description) = &request.description {
        sqlx::query("UPDATE scenes SET description = ?, updated_at = ? WHERE id = ?")
            .bind(description)
            .bind(chrono::Utc::now())
            .bind(id)
            .execute(&state.db)
            .await?;
    }

    let scene = sqlx::query_as::<_, Scene>(
        "SELECT * FROM scenes WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(scene.into()))
}

/// Delete a scene
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // First delete associated images
    sqlx::query("DELETE FROM scene_images WHERE scene_id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    // Then delete the scene
    let result = sqlx::query("DELETE FROM scenes WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Scene not found"));
    }

    info!("Deleted scene: {}", id);
    
    Ok(StatusCode::NO_CONTENT)
}

/// Get images for a scene
pub async fn get_images(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<crate::models::ImageResponse>>, AppError> {
    let images = sqlx::query_as::<_, crate::models::SceneImage>(
        r#"
        SELECT * FROM scene_images 
        WHERE scene_id = ?
        ORDER BY uploaded_at ASC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let responses: Vec<crate::models::ImageResponse> = images.into_iter().map(|i| i.into()).collect();
    
    Ok(Json(responses))
}

/// Custom error type for API errors
#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound(&'static str),
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
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e)).into_response()
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, format!("Not found: {}", msg)).into_response()
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
    use axum_test::TestServer;
    use tempfile::TempDir;

    async fn setup_test_app() -> (TestServer, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
        
        let state = AppState::new(&db_url, temp_dir.path().to_str().unwrap()).await.unwrap();
        let app = router().with_state(state);
        
        let server = TestServer::new(app).unwrap();
        (server, temp_dir)
    }

    #[tokio::test]
    async fn test_create_and_get_scene() {
        let (server, _temp) = setup_test_app().await;

        // Create scene
        let create_req = CreateSceneRequest {
            name: "Test Scene".to_string(),
            description: Some("A test scene".to_string()),
        };

        let response = server.post("/").json(&create_req).await;
        assert_eq!(response.status_code(), 200);

        let scene: SceneResponse = response.json();
        assert_eq!(scene.name, "Test Scene");
        assert_eq!(scene.status, SceneStatus::Pending);

        // Get scene
        let response = server.get(&format!("/{}", scene.id)).await;
        assert_eq!(response.status_code(), 200);

        let fetched: SceneResponse = response.json();
        assert_eq!(fetched.id, scene.id);
    }

    #[tokio::test]
    async fn test_list_scenes() {
        let (server, _temp) = setup_test_app().await;

        // Create two scenes
        for i in 0..2 {
            let req = CreateSceneRequest {
                name: format!("Scene {}", i),
                description: None,
            };
            server.post("/").json(&req).await;
        }

        let response = server.get("/").await;
        assert_eq!(response.status_code(), 200);

        let scenes: Vec<SceneResponse> = response.json();
        assert_eq!(scenes.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_scene() {
        let (server, _temp) = setup_test_app().await;

        // Create scene
        let req = CreateSceneRequest {
            name: "To Delete".to_string(),
            description: None,
        };
        let create_response = server.post("/").json(&req).await;
        let scene: SceneResponse = create_response.json();

        // Delete scene
        let response = server.delete(&format!("/{}", scene.id)).await;
        assert_eq!(response.status_code(), 204);

        // Verify deleted
        let response = server.get(&format!("/{}", scene.id)).await;
        assert_eq!(response.status_code(), 404);
    }
}
