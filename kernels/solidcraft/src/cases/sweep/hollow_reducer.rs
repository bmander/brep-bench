use crate::solids::*;
use solidcraft_kernel::loft_regions;

/// The hollow square of outer half-width `h`, its wall 2 thick, in the plane y = `y`, drawn in
/// x and z.
fn section(y: f64, h: f64) -> (Plane, Region2) {
    let mut r = rect(-h, -h, h, h);
    r.holes.push(rect(2.0 - h, 2.0 - h, h - 2.0, h - 2.0).outer.reversed());
    (Plane { origin: v3(0.0, y, 0.0), x: Vec3::X, y: Vec3::Z }, r)
}

/// A loft through the two hollow sections, outline to outline and hole to hole: between two
/// sections the kernel rules its walls, so each corner runs straight and each wall is flat.
pub fn build() -> R<Body> {
    loft_regions(&[section(0.0, 12.0), section(40.0, 6.0)])
}
