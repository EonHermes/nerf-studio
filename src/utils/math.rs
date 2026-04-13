//! Mathematical utilities for 3D transformations and NeRF computations

use nalgebra::{Matrix4, Point3, Vector3};
use std::f64::consts::PI;

/// Convert degrees to radians
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// Convert radians to degrees
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

/// Create a rotation matrix from Euler angles (pitch, yaw, roll)
pub fn euler_to_rotation_matrix(pitch: f64, yaw: f64, roll: f64) -> Matrix4<f64> {
    let cos_p = pitch.cos();
    let sin_p = pitch.sin();
    let cos_y = yaw.cos();
    let sin_y = yaw.sin();
    let cos_r = roll.cos();
    let sin_r = roll.sin();

    // Rotation around X (pitch)
    let rot_x = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_p, -sin_p, 0.0,
        0.0, sin_p, cos_p, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // Rotation around Y (yaw)
    let rot_y = Matrix4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    // Rotation around Z (roll)
    let rot_z = Matrix4::new(
        cos_r, -sin_r, 0.0, 0.0,
        sin_r, cos_r, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    rot_z * rot_y * rot_x
}

/// Spherical linear interpolation between two quaternions
pub fn slerp(q0: [f64; 4], q1: [f64; 4], t: f64) -> [f64; 4] {
    let dot = q0[0] * q1[0] + q0[1] * q1[1] + q0[2] * q1[2] + q0[3] * q1[3];
    
    let (q0, q1) = if dot < 0.0 {
        (q0.map(|x| -x), q1)
    } else {
        (q0, q1)
    };

    let theta_0 = f64::acos(dot.clamp(-1.0, 1.0));
    let theta = theta_0 * t;
    
    let sin_theta = theta.sin();
    let sin_theta_0 = theta_0.sin();
    
    let s0 = (theta_0 - theta).sin() / sin_theta_0;
    let s1 = sin_theta / sin_theta_0;

    [
        q0[0] * s0 + q1[0] * s1,
        q0[1] * s0 + q1[1] * s1,
        q0[2] * s0 + q1[2] * s1,
        q0[3] * s0 + q1[3] * s1,
    ]
}

/// Calculate bounding box from a set of points
pub fn calculate_bounding_box(points: &[Point3<f64>]) -> ([f64; 3], [f64; 3]) {
    if points.is_empty() {
        return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
    }

    let mut min = points[0].coords;
    let mut max = points[0].coords;

    for point in points.iter().skip(1) {
        for i in 0..3 {
            if point[i] < min[i] {
                min[i] = point[i];
            }
            if point[i] > max[i] {
                max[i] = point[i];
            }
        }
    }

    ([min[0], min[1], min[2]], [max[0], max[1], max[2]])
}

/// Lerp between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Smoothstep interpolation
pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Calculate camera-to-point distance
pub fn camera_distance(camera_pos: [f64; 3], point: &Point3<f64>) -> f64 {
    let dx = point[0] - camera_pos[0];
    let dy = point[1] - camera_pos[1];
    let dz = point[2] - camera_pos[2];
    
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deg_to_rad() {
        assert!((deg_to_rad(180.0) - PI).abs() < 1e-10);
        assert!((deg_to_rad(90.0) - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_smoothstep() {
        assert!((smoothstep(0.0, 1.0, 0.0) - 0.0).abs() < 1e-10);
        assert!((smoothstep(0.0, 1.0, 0.5) - 0.5).abs() < 0.01);
        assert!((smoothstep(0.0, 1.0, 1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(-1.0, -1.0, -1.0),
        ];

        let (min, max) = calculate_bounding_box(&points);
        
        assert_eq!(min, [-1.0, -1.0, -1.0]);
        assert_eq!(max, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_camera_distance() {
        let camera = [0.0, 0.0, 5.0];
        let point = Point3::new(0.0, 0.0, 0.0);
        
        assert!((camera_distance(camera, &point) - 5.0).abs() < 1e-10);
    }
}
