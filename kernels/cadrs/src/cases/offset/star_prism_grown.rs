//! The five-pointed star prism: its outline the polygon through ten points about the origin,
//! alternately 10 and 5 from it, the first (0, 10) and each next 36° further anticlockwise,
//! extruded 2 along +z, grown by 1/2: the solid of every point within 1/2 of it. The points'
//! edges round in pieces of cylinders; the five concave vertical edges stay sharp.
//!
//! The outline extruded, then cadrs's offset with rounded edges.
use crate::bench::extrude;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let star: Vec<[f64; 2]> = (0..10)
        .map(|i| {
            let r = if i % 2 == 0 { 10.0 } else { 5.0 };
            let a = (90.0 + 36.0 * i as f64).to_radians();
            [r * a.cos(), r * a.sin()]
        })
        .collect();
    let prism = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::polygon(&star)]), 2.0)?;
    grown(k, prism, 0.5)
}
