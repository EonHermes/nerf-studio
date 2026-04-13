//! Health check endpoint

use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: &'static str,
    pub timestamp: String,
    pub instance_id: String,
}

/// Health check handler
pub async fn health() -> Json<HealthResponse> {
    use chrono::Utc;
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now().to_rfc3339(),
        instance_id: Uuid::new_v4().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use axum::{Router, routing::get};

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = Router::new().route("/health", get(health));
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;
        
        assert_eq!(response.status_code(), 200);
        
        let body: HealthResponse = response.json();
        assert_eq!(body.status, "healthy");
        assert!(!body.instance_id.is_empty());
    }
}
