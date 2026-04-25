//! PNG diff routine. Decodes both images, compares pixels with a per-channel
//! tolerance, writes a red-tinted overlay where pixels differ, and reports
//! mismatch percentage.

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

pub struct Report {
    pub pct: f64,
    pub diff_px: u64,
    pub total_px: u64,
    pub diff_png_written: bool,
}

const CHANNEL_TOLERANCE: u8 = 4;

pub fn compare(a: &Path, b: &Path, out: &Path) -> Result<Report, String> {
    let img_a = image::open(a).map_err(|e| format!("open {}: {e}", a.display()))?.to_rgba8();
    let img_b = image::open(b).map_err(|e| format!("open {}: {e}", b.display()))?.to_rgba8();

    let (wa, ha) = img_a.dimensions();
    let (wb, hb) = img_b.dimensions();

    if (wa, ha) != (wb, hb) {
        return Err(format!(
            "dimension mismatch: a={wa}x{ha} b={wb}x{hb} (resize before comparing)"
        ));
    }

    let total_px = u64::from(wa) * u64::from(ha);
    let mut diff_px: u64 = 0;
    let mut overlay: RgbaImage = ImageBuffer::new(wa, ha);

    for y in 0..ha {
        for x in 0..wa {
            let pa = img_a.get_pixel(x, y);
            let pb = img_b.get_pixel(x, y);
            if pixel_differs(pa, pb, CHANNEL_TOLERANCE) {
                diff_px += 1;
                overlay.put_pixel(x, y, Rgba([255, 0, 64, 200])); // red flag
            } else {
                // faded original to give context
                let faded = Rgba([pa[0] / 3, pa[1] / 3, pa[2] / 3, 255]);
                overlay.put_pixel(x, y, faded);
            }
        }
    }

    let pct = if total_px == 0 {
        0.0
    } else {
        (diff_px as f64 / total_px as f64) * 100.0
    };

    let mut diff_png_written = false;
    if diff_px > 0 {
        overlay
            .save(out)
            .map_err(|e| format!("write {}: {e}", out.display()))?;
        diff_png_written = true;
    }

    Ok(Report {
        pct,
        diff_px,
        total_px,
        diff_png_written,
    })
}

fn pixel_differs(a: &Rgba<u8>, b: &Rgba<u8>, tol: u8) -> bool {
    (0..4).any(|i| a[i].abs_diff(b[i]) > tol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    fn write_solid(path: &Path, w: u32, h: u32, color: [u8; 4]) {
        let img: RgbaImage = ImageBuffer::from_pixel(w, h, Rgba(color));
        img.save(path).unwrap();
    }

    #[test]
    fn identical_images_match() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        let out = dir.path().join("diff.png");
        write_solid(&a, 16, 16, [255, 255, 255, 255]);
        write_solid(&b, 16, 16, [255, 255, 255, 255]);
        let r = compare(&a, &b, &out).unwrap();
        assert_eq!(r.diff_px, 0);
        assert_eq!(r.total_px, 256);
        assert!(!r.diff_png_written);
    }

    #[test]
    fn fully_different_images() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        let out = dir.path().join("diff.png");
        write_solid(&a, 8, 8, [0, 0, 0, 255]);
        write_solid(&b, 8, 8, [255, 255, 255, 255]);
        let r = compare(&a, &b, &out).unwrap();
        assert_eq!(r.diff_px, 64);
        assert_eq!(r.total_px, 64);
        assert!((r.pct - 100.0).abs() < 1e-6);
        assert!(r.diff_png_written);
    }

    #[test]
    fn dimension_mismatch_errors() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        let out = dir.path().join("diff.png");
        write_solid(&a, 8, 8, [0, 0, 0, 255]);
        write_solid(&b, 16, 16, [0, 0, 0, 255]);
        assert!(compare(&a, &b, &out).is_err());
    }

    #[test]
    fn tolerance_absorbs_tiny_delta() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        let out = dir.path().join("diff.png");
        write_solid(&a, 8, 8, [100, 100, 100, 255]);
        write_solid(&b, 8, 8, [103, 103, 103, 255]); // within CHANNEL_TOLERANCE=4
        let r = compare(&a, &b, &out).unwrap();
        assert_eq!(r.diff_px, 0);
    }
}
