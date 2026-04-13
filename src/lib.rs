//! Neural Radiance Fields (NeRF) Studio
//! 
//! A web application for creating photorealistic 3D scenes from 2D photo collections
//! using neural radiance fields technology.

pub mod api;
pub mod models;
pub mod nerf;
pub mod storage;
pub mod utils;

use anyhow::Result;
use sqlx::SqlitePool;
use tracing::info;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub uploads_dir: std::path::PathBuf,
}

impl AppState {
    pub async fn new(database_url: &str, uploads_dir: &str) -> Result<Self> {
        let db = SqlitePool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&db).await?;
        
        let uploads_path = std::path::PathBuf::from(uploads_dir);
        std::fs::create_dir_all(&uploads_path)?;
        
        info!("NeRF Studio initialized with uploads dir: {:?}", uploads_path);
        
        Ok(Self {
            db,
            uploads_dir: uploads_path,
        })
    }
}

/// Create the Axum router
pub fn create_router(state: AppState) -> axum::Router {
    use axum::Router;
    use axum::middleware;
    use tower_http::trace::TraceLayer;
    
    Router::new()
        // API routes
        .nest("/api/v1/scenes", api::scenes::router())
        .nest("/api/v1/images", api::images::router())
        .nest("/api/v1/render", api::render::router())
        .nest("/api/v1/export", api::export::router())
        
        // Health check
        .route("/api/v1/health", axum::routing::get(api::health))
        
        // Static files (frontend)
        .fallback_service(tower_http::services::ServeDir::new("./dist"))
        
        // Middleware
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
