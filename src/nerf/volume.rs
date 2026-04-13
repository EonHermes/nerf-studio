//! Volume rendering utilities for NeRF

use crate::nerf::camera::{rays, CameraIntrinsics};
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

/// Voxel grid representation (for spatial acceleration)
#[derive(Debug, Clone)]
pub struct VoxelGrid {
    pub resolution: u32,
    pub bounds_min: [f64; 3],
    pub bounds_max: [f64; 3],
    // In production: density and color values per voxel
}

impl VoxelGrid {
    pub fn new(resolution: u32, bounds_min: [f64; 3], bounds_max: [f64; 3]) -> Self {
        Self {
            resolution,
            bounds_min,
            bounds_max,
        }
    }

    /// Convert world coordinates to voxel indices
    pub fn world_to_voxel(&self, point: &Point3<f64>) -> [u32; 3] {
        let dx = (point.x - self.bounds_min[0]) / (self.bounds_max[0] - self.bounds_min[0]);
        let dy = (point.y - self.bounds_min[1]) / (self.bounds_max[1] - self.bounds_min[1]);
        let dz = (point.z - self.bounds_min[2]) / (self.bounds_max[2] - self.bounds_min[2]);

        [
            (dx * self.resolution as f64).clamp(0.0, 1.0) as u32,
            (dy * self.resolution as f64).clamp(0.0, 1.0) as u32,
            (dz * self.resolution as f64).clamp(0.0, 1.0) as u32,
        ]
    }
}

/// Volume rendering integration
pub fn integrate_along_ray(
    origin: &Point3<f64>,
    direction: &Vector3<f64>,
    samples: &[Point3<f64>],
    density_fn: &dyn Fn(&Point3<f64>) -> f64,
    color_fn: &dyn Fn(&Point3<f64>) -> [f64; 3],
) -> ([f64; 3], f64) {
    let mut accumulated_alpha = 0.0;
    let mut accumulated_color = [0.0, 0.0, 0.0];

    for (i, point) in samples.iter().enumerate() {
        let density = density_fn(point);
        let color = color_fn(point);

        // Transmittance
        let step_size = if i > 0 {
            let prev = &samples[i - 1];
            ((*point - *prev).norm())
        } else {
            0.1
        };

        let alpha = 1.0 - (-density * step_size).exp();
        
        // Composite
        let weight = (1.0 - accumulated_alpha) * alpha;
        for c in 0..3 {
            accumulated_color[c] += weight * color[c];
        }
        accumulated_alpha += weight * alpha;

        if accumulated_alpha > 0.99 {
            break; // Early termination
        }
    }

    (accumulated_color, accumulated_alpha)
}

/// Hierarchical sampling for efficient volume rendering
pub struct HierarchicalSampler {
    pub coarse_samples: u32,
    pub fine_samples: u32,
    pub near: f64,
    pub far: f64,
}

impl HierarchicalSampler {
    pub fn new(coarse: u32, fine: u32, near: f64, far: f64) -> Self {
        Self {
            coarse_samples: coarse,
            fine_samples: fine,
            near,
            far,
        }
    }

    /// Generate initial coarse samples
    pub fn generate_coarse_samples(&self) -> Vec<f64> {
        let step = (self.far - self.near) / self.coarse_samples as f64;
        
        (0..self.coarse_samples)
            .map(|i| self.near + (i as f64 + 0.5) * step)
            .collect()
    }

    /// Generate fine samples based on coarse weights
    pub fn generate_fine_samples(&self, _coarse_t: &[f64], _weights: &[f64]) -> Vec<f64> {
        // In production: use weighted sampling around high-density regions
        // For now, return additional uniform samples
        let step = (self.far - self.near) / self.fine_samples as f64;
        
        (0..self.fine_samples)
            .map(|i| self.near + (i as f64 + 0.5) * step)
            .collect()
    }
}

/// Positional encoding for NeRF networks
pub struct PositionalEncoder {
    pub num_frequencies: u32,
    pub include_original: bool,
}

impl PositionalEncoder {
    pub fn new(num_frequencies: u32, include_original: bool) -> Self {
        Self {
            num_frequencies,
            include_original,
        }
    }

    /// Apply positional encoding to a vector
    pub fn encode(&self, x: &[f64; 3]) -> Vec<f64> {
        let mut encoded = if self.include_original {
            x.to_vec()
        } else {
            Vec::new()
        };

        for i in 0..self.num_frequencies {
            let freq = 2.0_f64.powi(i as i32);
            
            for coord in *x {
                encoded.push((freq * coord).sin());
                encoded.push((freq * coord).cos());
            }
        }

        encoded
    }

    /// Get the output dimension given input dimension
    pub fn output_dim(&self, input_dim: usize) -> usize {
        let base = if self.include_original { input_dim } else { 0 };
        base + input_dim * 2 * self.num_frequencies as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_grid_conversion() {
        let grid = VoxelGrid::new(10, [0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        
        let point = Point3::new(5.0, 5.0, 5.0);
        let voxel = grid.world_to_voxel(&point);
        
        assert_eq!(voxel, [5, 5, 5]);

        let corner = Point3::new(0.0, 0.0, 0.0);
        let voxel_corner = grid.world_to_voxel(&corner);
        assert_eq!(voxel_corner, [0, 0, 0]);
    }

    #[test]
    fn test_positional_encoding() {
        let encoder = PositionalEncoder::new(4, true);
        
        let input = [1.0, 2.0, 3.0];
        let encoded = encoder.encode(&input);
        
        // Original (3) + 4 frequencies * 2 (sin/cos) * 3 dims = 3 + 24 = 27
        assert_eq!(encoded.len(), 27);
        
        // First 3 should be original
        assert_eq!(&encoded[0..3], &input);
    }

    #[test]
    fn test_hierarchical_sampler() {
        let sampler = HierarchicalSampler::new(8, 16, 1.0, 10.0);
        
        let coarse = sampler.generate_coarse_samples();
        assert_eq!(coarse.len(), 8);
        assert!(*coarse.first().unwrap() > 1.0);
        assert!(*coarse.last().unwrap() < 10.0);
    }

    #[test]
    fn test_volume_integration() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let direction = Vector3::z();
        
        let samples: Vec<Point3<f64>> = (0..10)
            .map(|i| Point3::new(0.0, 0.0, i as f64 * 0.5 + 0.25))
            .collect();

        // Simple density function (constant)
        let density_fn = |_p: &Point3<f64>| 0.1;
        
        // Simple color function (gradient)
        let color_fn = |p: &Point3<f64>| [p.z / 5.0, 0.5, 1.0 - p.z / 5.0];

        let (color, alpha) = integrate_along_ray(&origin, &direction, &samples, &density_fn, &color_fn);
        
        assert!(alpha > 0.0 && alpha <= 1.0);
        assert!(color[0] >= 0.0 && color[0] <= 1.0);
    }
}
