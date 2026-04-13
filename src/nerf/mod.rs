//! Neural Radiance Fields (NeRF) implementation
//! 
//! This module contains the core NeRF training and rendering logic.
//! In production, this would use candle/tch-rs for GPU-accelerated ML.

pub mod engine;
pub mod camera;
pub mod volume;

pub use engine::*;
pub use camera::*;
pub use volume::*;
