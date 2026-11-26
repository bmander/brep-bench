//! A plate from the origin to (40, 30, 6) with all twelve of its edges rounded to radius 3, half
//! its thickness, in one operation.
use crate::bench::{edges, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(40.0, 30.0, 6.0)).build(m)?;
    let all = edges(m, plate);
    Ok(blend::fillet(m, plate, &all, 3.0)?)
}
