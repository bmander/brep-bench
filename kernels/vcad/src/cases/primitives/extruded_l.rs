//! An L-shaped plate: the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in
//! the xy plane, extruded 8 along +z.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
const DEPTH: f64 = 8.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let corner = |i: usize| Point2::new(OUTLINE[i % 6].0, OUTLINE[i % 6].1);
    let sides = (0..6).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    let outline = SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::y(), sides.collect())?;
    Ok(Solid::extrude(outline, Vec3::new(0.0, 0.0, DEPTH))?)
}
