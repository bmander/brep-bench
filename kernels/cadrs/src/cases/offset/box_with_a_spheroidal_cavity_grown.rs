//! The cube from (−10, −10, −10) to (10, 10, 10) less the oblate spheroid of
//! `geometry/oblate-spheroid`, (x² + y²)/36 + z²/9 ≤ 1, a closed cavity at its middle, grown by 1:
//! the solid of every point within 1 of it.
//!
//! The cube is a square extruded, the spheroid as `geometry/oblate-spheroid` makes it (the half
//! ellipse revolved) and cut out of it; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, p3};
use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(-10.0, -10.0, -10.0), p3(10.0, 10.0, 10.0))?;
    let spheroid = oblate_spheroid::build(k)?;
    let hollow = cut(k, cube, spheroid)?;
    grown(k, hollow, 1.0)
}
