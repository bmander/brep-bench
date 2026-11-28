//! The five-pointed star prism, its outline the ten points alternately 10 and 5 from the origin,
//! the first (0, 10) and each next 36° further anticlockwise, extruded 2 along +z, grown by 1/2:
//! every point within 1/2 of it, its convex edges rounded and its five concave ones sharp,
//! SolidCraft's offset of the whole solid (`offset_solid`), its convex edges rounded.
use crate::solids::*;

pub fn build() -> R<Body> {
    let points: Vec<(f64, f64)> = (0..10)
        .map(|i| {
            let r = if i % 2 == 0 { 10.0 } else { 5.0 };
            let a = (90.0 + 36.0 * i as f64).to_radians();
            (r * a.cos(), r * a.sin())
        })
        .collect();
    let star = extrude(&Plane::XY, polygon(&points), 0.0, 2.0)?;
    solidcraft_kernel::offset_solid(&star, 0.5, true)
}
