//! A cylinder of radius 3 about the z axis from z = −10 up to the plane z = x·tan 45°, grown by 1:
//! every point within 1 of it, its bottom rim a piece of a torus and its elliptic top rim a tube
//! about the ellipse, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges
//! rounded.
use crate::solids::*;

/// The cylinder of radius 3 from z = −10, cut off by the plane z = x·tan `degrees`: it runs up
/// past the plane, and the half-space above the plane, a block on z > 0 turned −`degrees` about
/// the y axis (its face's normal then (−sin, 0, cos)), is cut from it.
pub fn slant_cut_cylinder(degrees: f64) -> R<Body> {
    let top = 3.0 * degrees.to_radians().tan();
    let cylinder = zcyl(0.0, 0.0, 3.0, -10.0, 1.0 + top)?;
    let above = block((-30.0, -30.0, 0.0), (30.0, 30.0, 30.0))?;
    let above = solidcraft_kernel::transform(&above, Vec3::ZERO, Vec3::ZERO, Vec3::Y, -degrees.to_radians())?;
    cut(&cylinder, &above)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&slant_cut_cylinder(45.0)?, 1.0, true)
}
