//! Image processing utilities

use anyhow::Result;
use image::{DynamicImage, GenericImageView};

/// Resize an image while maintaining aspect ratio
pub fn resize_maintain_aspect(
    img: &DynamicImage,
    max_width: u32,
    max_height: u32,
) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    if width <= max_width && height <= max_height {
        return img.clone();
    }

    let ratio = (max_width as f32 / width as f32).min(max_height as f32 / height as f32);
    let new_width = (width as f32 * ratio) as u32;
    let new_height = (height as f32 * ratio) as u32;

    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// Extract EXIF orientation and rotate if needed
pub fn apply_exif_orientation(img: DynamicImage) -> Result<DynamicImage> {
    // In production, would read EXIF data
    // For now, return as-is
    Ok(img)
}

/// Calculate image statistics for quality assessment
pub struct ImageStats {
    pub mean_brightness: f64,
    pub contrast: f64,
    pub saturation: f64,
}

pub fn calculate_stats(img: &DynamicImage) -> ImageStats {
    let pixels = img.to_rgb8();
    
    let mut sum_r = 0.0;
    let mut sum_g = 0.0;
    let mut sum_b = 0.0;
    let total = pixels.len() as f64 / 3.0;

    for pixel in pixels.pixels() {
        sum_r += pixel[0] as f64;
        sum_g += pixel[1] as f64;
        sum_b += pixel[2] as f64;
    }

    let mean_r = sum_r / total / 255.0;
    let mean_g = sum_g / total / 255.0;
    let mean_b = sum_b / total / 255.0;
    
    // Mean brightness (Y component in YUV)
    let mean_brightness = 0.299 * mean_r + 0.587 * mean_g + 0.114 * mean_b;

    // Simple contrast estimate (standard deviation approximation)
    let contrast = 0.3; // Placeholder - would calculate actual std dev

    // Saturation estimate
    let max_c = mean_r.max(mean_g).max(mean_b);
    let min_c = mean_r.min(mean_g).min(mean_b);
    let saturation = if max_c > 0.0 { (max_c - min_c) / max_c } else { 0.0 };

    ImageStats {
        mean_brightness,
        contrast,
        saturation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;
    use image::Rgba;

    #[test]
    fn test_resize_maintain_aspect() {
        let img = ImageBuffer::<Rgba<u8>, _>::from_fn(100, 200, |_, _| Rgba([255, 0, 0, 255]));
        let dyn_img = DynamicImage::ImageRgba8(img);

        let resized = resize_maintain_aspect(&dyn_img, 50, 50);
        let (w, h) = resized.dimensions();

        assert!(w <= 50);
        assert!(h <= 50);
        // Should maintain aspect ratio (1:2)
        assert!((h as f32 / w as f32 - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_stats_calculation() {
        let img = ImageBuffer::<Rgba<u8>, _>::from_fn(10, 10, |_, _| Rgba([128, 128, 128, 255]));
        let dyn_img = DynamicImage::ImageRgba8(img);

        let stats = calculate_stats(&dyn_img);
        
        // Gray image should have ~0.5 brightness and low saturation
        assert!((stats.mean_brightness - 0.5).abs() < 0.1);
        assert!(stats.saturation < 0.1);
    }
}
