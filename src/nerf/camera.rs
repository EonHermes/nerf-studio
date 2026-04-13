//! Camera utilities for NeRF

use nalgebra::{Matrix4, Point3, Vector3};
use std::f64::consts::PI;

/// Camera intrinsics (projection matrix parameters)
#[derive(Debug, Clone)]
pub struct CameraIntrinsics {
    pub focal_length_x: f64,
    pub focal_length_y: f64,
    pub principal_point_x: f64,
    pub principal_point_y: f64,
    pub image_width: u32,
    pub image_height: u32,
}

impl CameraIntrinsics {
    pub fn new(fx: f64, fy: f64, cx: f64, cy: f64, width: u32, height: u32) -> Self {
        Self {
            focal_length_x: fx,
            focal_length_y: fy,
            principal_point_x: cx,
            principal_point_y: cy,
            image_width: width,
            image_height: height,
        }
    }

    /// Create default intrinsics for a given resolution
    pub fn default_for_resolution(width: u32, height: u32) -> Self {
        let focal = (width as f64 + height as f64) / 4.0;
        Self::new(
            focal,
            focal,
            width as f64 / 2.0,
            height as f64 / 2.0,
            width,
            height,
        )
    }

    /// Get the projection matrix
    pub fn projection_matrix(&self) -> Matrix4<f64> {
        let fx = self.focal_length_x;
        let fy = self.focal_length_y;
        let cx = self.principal_point_x;
        let cy = self.principal_point_y;
        let w = self.image_width as f64;
        let h = self.image_height as f64;

        Matrix4::new(
            2.0 * fx / w, 0.0, (w - 2.0 * cx) / w, 0.0,
            0.0, 2.0 * fy / h, (h - 2.0 * cy) / h, 0.0,
            0.0, 0.0, 0.0, -1.0,
            0.0, 0.0, -1.0, 0.0,
        )
    }
}

/// Convert pitch/yaw to view direction vector
pub fn rotation_to_direction(pitch: f64, yaw: f64) -> Vector3<f64> {
    let cos_pitch = pitch.cos();
    let sin_pitch = pitch.sin();
    let cos_yaw = yaw.cos();
    let sin_yaw = yaw.sin();

    Vector3::new(
        cos_pitch * sin_yaw,
        sin_pitch,
        -cos_pitch * cos_yaw,
    )
}

/// Create view matrix from camera pose
pub fn create_view_matrix(position: [f64; 3], pitch: f64, yaw: f64) -> Matrix4<f64> {
    let cam_pos = Point3::new(position[0], position[1], position[2]);
    let direction = rotation_to_direction(pitch, yaw);
    
    // Simple look-at matrix (assuming up is +Y)
    let target = cam_pos + direction;
    let up = Vector3::y();

    // Create view matrix using nalgebra's look_at_lh
    Matrix4::look_at_rh(&cam_pos, &target, &up)
}

/// Ray casting utilities for NeRF sampling
pub mod rays {
    use super::*;

    /// Generate a ray from pixel coordinates
    pub fn pixel_to_ray(
        x: u32,
        y: u32,
        intrinsics: &CameraIntrinsics,
        view_matrix: &Matrix4<f64>,
    ) -> (Point3<f64>, Vector3<f64>) {
        let fx = intrinsics.focal_length_x;
        let fy = intrinsics.focal_length_y;
        let cx = intrinsics.principal_point_x;
        let cy = intrinsics.principal_point_y;

        // Convert pixel to normalized camera coordinates
        let ndc_x = (x as f64 - cx) / fx;
        let ndc_y = (y as f64 - cy) / fy;

        // Ray direction in camera space
        let dir_cam = Vector3::new(ndc_x, -ndc_y, -1.0).normalize();

        // Transform to world space
        let dir_world = view_matrix.transform_vector(&dir_cam).normalize();
        
        // Camera position is the ray origin
        let origin = Point3::new(
            view_matrix[(0, 3)],
            view_matrix[(1, 3)],
            view_matrix[(2, 3)],
        );

        (origin, dir_world)
    }

    /// Sample points along a ray for NeRF evaluation
    pub fn sample_ray(
        origin: &Point3<f64>,
        direction: &Vector3<f64>,
        near: f64,
        far: f64,
        num_samples: u32,
    ) -> Vec<Point3<f64>> {
        let mut points = Vec::with_capacity(num_samples as usize);
        
        // Use stratified sampling for better quality
        let step = (far - near) / num_samples as f64;
        
        for i in 0..num_samples {
            let t = near + (i as f64 + 0.5) * step;
            let point = origin + direction * t;
            points.push(point);
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_intrinsics_default() {
        let intrinsics = CameraIntrinsics::default_for_resolution(512, 512);
        assert_eq!(intrinsics.image_width, 512);
        assert_eq!(intrinsics.image_height, 512);
        assert!((intrinsics.focal_length_x - 256.0).abs() < 1.0);
    }

    #[test]
    fn test_rotation_to_direction() {
        // Looking straight ahead (pitch=0, yaw=0)
        let dir = rotation_to_direction(0.0, 0.0);
        assert!((dir.y - 0.0).abs() < 1e-6);
        assert!((dir.z - (-1.0)).abs() < 1e-6);

        // Looking up (pitch=PI/2)
        let dir_up = rotation_to_direction(PI / 2.0, 0.0);
        assert!((dir_up.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ray_sampling() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vector3::z();
        
        let samples = rays::sample_ray(&origin, &direction, 1.0, 10.0, 10);
        
        assert_eq!(samples.len(), 10);
        // First sample should be near=1.0
        assert!((samples[0].z - 1.5).abs() < 0.1);
        // Last sample should be near far=10.0
        assert!((samples[9].z - 9.5).abs() < 0.1);
    }
}
