use crate::solids::*;
use solidcraft_kernel::loft_regions;

/// The most sections a SolidCraft loft takes.
const SECTIONS: usize = 50;

/// The hollow section an angle `t` (degrees) round the bend from the xz plane: in the plane
/// through the z axis there, drawn with its x away from the axis and its y up, the square of
/// half-width h = 9 − 3·t/90° about the arc of radius 30, less the square of half-width h − 2.
fn section(t: f64) -> (Plane, Region2) {
    let (s, c) = t.to_radians().sin_cos();
    let plane = Plane { origin: Vec3::ZERO, x: v3(c, s, 0.0), y: Vec3::Z };
    let h = 9.0 - 3.0 * t / 90.0;
    let mut r = rect(30.0 - h, -h, 30.0 + h, h);
    let g = h - 2.0;
    r.holes.push(rect(30.0 - g, -g, 30.0 + g, g).outer.reversed());
    (plane, r)
}

/// SolidCraft's sweep changes its section only by a uniform end scale, about the path, and
/// this section's hole narrows by another factor than its outside (7 to 4 while the outside
/// goes 9 to 6). So the duct is a loft through as many of its sections as a loft takes, evenly
/// round the bend: one smooth face per side through them all, the outline's and the hole's,
/// the end sections its flat ends in the xz and yz planes. Each section's sides are straight,
/// and so is every section of the smooth faces between them.
pub fn build() -> R<Body> {
    let sections: Vec<_> =
        (0..SECTIONS).map(|i| section(90.0 * i as f64 / (SECTIONS - 1) as f64)).collect();
    loft_regions(&sections)
}
