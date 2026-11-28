//! The five-pointed star prism, its outline the ten points alternately 10 and 5 from the origin,
//! the first (0, 10) and each next 36° further anticlockwise, extruded 2 along +z, grown by 1/2:
//! every point within 1/2 of it, its convex edges rounded and its five concave ones sharp, the
//! kernel's offset with its convex edges rounded.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let points: Vec<[f64; 3]> = (0..10)
        .map(|i| {
            let r = if i % 2 == 0 { 10.0 } else { 5.0 };
            let a = (90.0 + 36.0 * i as f64).to_radians();
            [r * a.cos(), r * a.sin(), 0.0]
        })
        .collect();
    let star = extrude(&polygon(&points), [0.0, 0.0, 1.0], 2.0, 0.0, DraftCorner::default(), true)?;
    offset_rounded(&star, 0.5)
}
