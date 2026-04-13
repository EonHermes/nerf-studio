//! Export model - represents export requests for 3D formats

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Export format options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    /// OBJ file with MTL material
    Obj,
    /// GLTF 2.0 format
    Gltf,
    /// GLB binary format
    Glb,
    /// PLY point cloud format
    Ply,
}

/// Request to export a scene
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub scene_id: Uuid,
    pub format: ExportFormat,
    /// Include texture maps (for OBJ/GLTF)
    #[serde(default = "default_true")]
    pub include_textures: bool,
    /// Point cloud density (for PLY export)
    pub point_density: Option<u32>,
}

fn default_true() -> bool {
    true
}

/// Response with export download URL
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResponse {
    pub scene_id: Uuid,
    pub format: ExportFormat,
    pub file_url: String,
    pub file_size_bytes: Option<u64>,
    pub expires_at: Option<String>,
}

/// 3D mesh data structure for export
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub tex_coords: Vec<[f32; 2]>,
    pub faces: Vec<[u32; 3]>,
}
