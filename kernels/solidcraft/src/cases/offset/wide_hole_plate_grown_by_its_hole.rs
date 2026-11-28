//! The plate from (−30, −30, 0) to (30, 30, 4) with a hole of radius 20 drilled through it along
//! the z axis, grown by 20, the hole's radius: every point within 20 of it, the kernel's offset
//! with its convex edges rounded. The hole closes onto its axis, and over each end the grown face
//! dips to a cusp, the inside of a horn torus about the rim.
use crate::bench::cut;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([-30.0, -30.0, 0.0], [60.0, 60.0, 4.0])?;
    // the drill runs past both faces
    let drill = make_cylinder([0.0, 0.0, -1.0], 20.0, 6.0)?;
    offset_rounded(&cut(plate, drill)?, 20.0)
}
