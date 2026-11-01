//! Small helpers shared by the case implementations: points and closed polygons as the kernel's
//! curves.
use brep_kernel::{make_line, NurbsCurve, Vec3};

/// The point (x, y, z).
pub fn p(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// The closed polygon through `corners`, as one line per side.
pub fn polygon(corners: &[Vec3]) -> Result<Vec<NurbsCurve>, String> {
    (0..corners.len()).map(|i| make_line(corners[i], corners[(i + 1) % corners.len()])).collect()
}

// Helpers of one group of categories each, so that each group's cases share them.
pub mod boolean;
pub mod fillet;
pub mod geometry;
pub mod offset;
pub mod shell;
pub mod sweep;
