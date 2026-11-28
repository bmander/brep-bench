//! The cube from (−10, −10, −10) to (10, 10, 10) with the oblate spheroid of
//! `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, cut out of its middle, a closed cavity,
//! grown by 1: every point within 1 of it, the kernel's offset with its convex edges rounded.
use crate::bench::cut;
use crate::cases::geometry::oblate_spheroid;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([-10.0, -10.0, -10.0], [20.0, 20.0, 20.0])?;
    let hollow = cut(cube, oblate_spheroid::build()?)?;
    offset_rounded(&hollow, 1.0)
}
