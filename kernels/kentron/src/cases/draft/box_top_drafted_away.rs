//! The box from the origin to (30, 10, 10), its four sides drafted 30° with the pull direction
//! +z about the neutral plane z = 0, inward as they rise. The long sides meet at height 5√3,
//! below the top, so the draft consumes the top face: a hip roof of five faces, the box cut by
//! the four drafted planes.
//!
//! kentron's `draft`, which closes the part where sides drafted until they meet take its top
//! away.
use crate::bench::p;
use crate::shared::draft::{box_sides, drafted};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (lo, hi) = (p(0.0, 0.0, 0.0), p(30.0, 10.0, 10.0));
    let block = Cuboid::from_corners(lo, hi).build(m)?;
    let sides = box_sides(m, block, lo, hi, 0.0)?;
    drafted(m, block, &sides, 30.0, p(0.0, 0.0, 0.0), Direction::Z)
}
