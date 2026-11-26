//! The plus-shaped plate whose outline is the polygon through (3, 0), (6, 0), (6, 3), (9, 3),
//! (9, 6), (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3), extruded 2 along +z, grown
//! by 1: the solid of every point within 1 of it. Convex edges become quarter cylinders and
//! corners eighths of balls; the four concave vertical edges stay sharp.
//!
//! The outline extruded, then cadrs's offset with rounded edges.
use crate::bench::extrude;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub const PLUS: [[f64; 2]; 12] = [
    [3.0, 0.0], [6.0, 0.0], [6.0, 3.0], [9.0, 3.0], [9.0, 6.0], [6.0, 6.0],
    [6.0, 9.0], [3.0, 9.0], [3.0, 6.0], [0.0, 6.0], [0.0, 3.0], [3.0, 3.0],
];

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::polygon(&PLUS)]), 2.0)?;
    grown(k, plate, 1.0)
}
