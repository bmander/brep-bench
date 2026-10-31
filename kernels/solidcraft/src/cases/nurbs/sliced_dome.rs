use super::bezier_dome::block;
use crate::solids::*;

/// The block under the dome of `nurbs/bezier-dome`, kept on the negative side of the plane
/// z = 4 + x/2: the plane through the corner (0, 0, 4), rising along (2, 0, 1) and level along
/// y, whose normal x × y = (−1, 0, 2)/√5 points up and away from the kept part.
pub fn build() -> Result<Body, crate::Refusal> {
    let rise = v3(2.0, 0.0, 1.0) * (1.0 / 5f64.sqrt());
    let plane = Plane { origin: v3(0.0, 0.0, 4.0), x: rise, y: Vec3::Y };
    let (below, _) = solidcraft_kernel::half_space(&block()?, &plane, false)?;
    Ok(below.ok_or("nothing of the block lies below the plane")?)
}
