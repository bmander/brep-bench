//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut by a ball of radius 3
//! whose centre swings a quarter turn about the z axis in the top face, from (12, 0, 0) to
//! (0, 12, 0).
//!
//! solidcraft-kernel has no sweep of a solid, so the ball's sweep is built by elementary geometry:
//! every point within 3 of the arc its centre runs along, which is the ball's great disc in the
//! plane through the axis turned the quarter turn (a quarter of a torus's tube) with the ball at
//! each end. Half of each end ball lies inside the tube; the other half is the half ball on the
//! tube's end disc, outside it, joined to the tube on that disc. (The whole ball joined or cut
//! meets the tube only tangentially, along its end disc's rim, and the kernel's boolean finds no
//! result there.)
use crate::solids::*;

/// The half of the ball of `radius` about `centre` on the side of the plane through `centre` that
/// `outward` points to.
pub fn half_ball(centre: Vec3, outward: Vec3, radius: f64) -> R<Body> {
    let ball = solidcraft_kernel::sphere(centre, radius)?;
    let cap = Plane::from_normal(centre, outward)
        .ok_or_else(|| KernelError::Invalid("the half ball's direction".into()))?;
    solidcraft_kernel::half_space(&ball, &cap, true)?
        .0
        .ok_or_else(|| KernelError::Failed("the half ball is empty".into()))
}

pub fn build() -> R<Body> {
    let block = block((-6.0, -6.0, -8.0), (18.0, 18.0, 0.0))?;
    // the disc about (12, 0, 0) in the xz plane, turned a quarter turn from +x towards +y
    let mut swept = revolve_z(disc((12.0, 0.0), 3.0), 90.0)?;
    // the half balls beyond the tube's ends: it leaves (12, 0, 0) along +y and reaches (0, 12, 0)
    // along −x
    for (end, outward) in [(v3(12.0, 0.0, 0.0), -Vec3::Y), (v3(0.0, 12.0, 0.0), -Vec3::X)] {
        swept = union(&swept, &half_ball(end, outward, 3.0)?)?;
    }
    cut(&block, &swept)
}
