//! The common part of two barrel-vaulted blocks on the square |x|, |y| ≤ 12, the second the first
//! turned a quarter turn about z. Each vault's top is the rational quadratic arc through
//! (y, z) = (−12, 10), (0, 26), (12, 10) with weights 1, 3/5, 1, carried straight along x.
//!
//! vcad makes no face on a rational B-spline surface given by its control points, but that arc
//! is exactly the circle of radius 15 about (y, z) = (0, 1), as the case says, and vcad's sketch
//! has circular arcs: the vault is its section, an exact arc, extruded, so its top is vcad's
//! exact cylinder.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const HALF: f64 = 12.0;
/// The height of the vault's springing, at the sides y = ±12.
const SPRING: f64 = 10.0;
/// The centre (y, z) of the arch's circle, of radius 15 through both springings.
const ARCH_CENTRE: (f64, f64) = (0.0, 1.0);

/// The first block: its section in the plane x = −12 (the sketch's x along y and its y along z),
/// floor, sides and arch, extruded along +x to x = 12.
pub fn vault() -> Result<Solid, crate::Refusal> {
    let p = Point2::new;
    let section = vec![
        SketchSegment::Line { start: p(-HALF, 0.0), end: p(HALF, 0.0) },
        SketchSegment::Line { start: p(HALF, 0.0), end: p(HALF, SPRING) },
        SketchSegment::Arc {
            start: p(HALF, SPRING),
            end: p(-HALF, SPRING),
            center: p(ARCH_CENTRE.0, ARCH_CENTRE.1),
            ccw: true,
        },
        SketchSegment::Line { start: p(-HALF, SPRING), end: p(-HALF, 0.0) },
    ];
    let profile = SketchProfile::new(Point3::new(-HALF, 0.0, 0.0), Vec3::y(), Vec3::z(), section)?;
    Ok(Solid::extrude(profile, Vec3::new(2.0 * HALF, 0.0, 0.0))?)
}

// workaround: no NURBS surface
pub fn build() -> Result<Solid, crate::Refusal> {
    let first = vault()?;
    let second = first.rotate(0.0, 0.0, 90.0);
    Ok(first.try_intersection(&second)?)
}
