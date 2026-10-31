//! Helpers the case implementations share: SolidCraft's kernel takes planar regions on a plane
//! and returns bodies in lists; these keep a case to the solid it describes.
#![allow(dead_code)]

pub use solidcraft_geom::{Loop2, Plane, Region2, Vec2, Vec3};
pub use solidcraft_kernel::{Body, BoolOp, KernelError};
use solidcraft_kernel as k;

pub type R<T> = Result<T, KernelError>;

pub fn v3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn v2(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

/// How an error begins when the kernel answered that a boolean leaves nothing: an answer, not a
/// refusal, which the runner reports as an empty solid (graded on its measures, zero).
pub const EMPTY: &str = "the kernel's answer is empty: ";

/// A boolean whose result is a solid. The kernel answers `None` when it finds the result empty,
/// which is an answer (`EMPTY`), not a refusal.
pub fn op(a: &Body, b: &Body, o: BoolOp) -> R<Body> {
    k::boolean(a, b, o)?.ok_or_else(|| KernelError::Failed(format!("{EMPTY}{o:?} left nothing")))
}

pub fn union(a: &Body, b: &Body) -> R<Body> {
    op(a, b, BoolOp::Union)
}

pub fn cut(a: &Body, b: &Body) -> R<Body> {
    op(a, b, BoolOp::Cut)
}

pub fn common(a: &Body, b: &Body) -> R<Body> {
    op(a, b, BoolOp::Intersect)
}

/// The one body an operation that answers a list made.
pub fn one(mut bodies: Vec<Body>) -> R<Body> {
    match bodies.len() {
        1 => Ok(bodies.remove(0)),
        n => Err(KernelError::Failed(format!("{n} bodies where one was expected"))),
    }
}

/// The plane of a section drawn in x and z, z up: its sketch x is +x and its sketch y is +z,
/// so a point (x, z) of the section is (x, 0, z). (The kernel's `Plane::XZ` looks from the
/// front, its sketch y −z.)
pub fn xz() -> Plane {
    Plane { origin: Vec3::ZERO, x: Vec3::X, y: Vec3::Z }
}

/// The region inside the polygon through `pts`.
pub fn polygon(pts: &[(f64, f64)]) -> Region2 {
    let pts: Vec<Vec2> = pts.iter().map(|&(x, y)| v2(x, y)).collect();
    Region2 { outer: Loop2::polygon(&pts), holes: vec![] }
}

/// The rectangle from (x0, y0) to (x1, y1).
pub fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Region2 {
    polygon(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)])
}

/// The disc of radius `r` about `c`.
pub fn disc(c: (f64, f64), r: f64) -> Region2 {
    Region2 { outer: Loop2::circle(v2(c.0, c.1), r), holes: vec![] }
}

/// A section in x and z (`xz`) turned through `angle` degrees about the z axis, right-handed:
/// from +x towards +y. The kernel's `revolve` of a section on `xz` about its sketch y (+z) turns
/// a positive angle the other way, from +x towards −y, so the angle goes to it negated.
pub fn revolve_z(section: Region2, angle: f64) -> R<Body> {
    one(solidcraft_kernel::revolve(&xz(), &[section], Vec2::ZERO, Vec2::Y, -angle.to_radians())?)
}

/// `region` on `plane` extruded from `start` to `end` along the plane's normal.
pub fn extrude(plane: &Plane, region: Region2, start: f64, end: f64) -> R<Body> {
    one(solidcraft_kernel::extrude(plane, &[region], start, end)?)
}

/// The cylinder of radius `r` on the z axis through (x, y), from z0 to z1.
pub fn zcyl(x: f64, y: f64, r: f64, z0: f64, z1: f64) -> R<Body> {
    solidcraft_kernel::cylinder(v3(x, y, z0), Vec3::Z, r, z1 - z0)
}

/// The box from `a` to `b`.
pub fn block(a: (f64, f64, f64), b: (f64, f64, f64)) -> R<Body> {
    solidcraft_kernel::box_solid(v3(a.0, a.1, a.2), v3(b.0, b.1, b.2))
}
