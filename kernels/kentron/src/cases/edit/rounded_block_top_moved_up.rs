//! The box from the origin to (40, 30, 10), the edge where its top, z = 10, meets its back,
//! y = 30, rounded to radius 3. Its top is moved up by 6, to z = 16, carrying the round with it:
//! the round stays a quarter cylinder of radius 3 tangent to the top and the back, its axis moved
//! from y = 27, z = 7 up to z = 13, and the back and the ends run on up to meet them.
//!
//! The top and its round are moved together, rigidly, by kentron's `draft::move_faces`: their
//! surfaces carried up 6, the faces round them kept on their own surfaces and met again.
use crate::bench::{edge_through, face_through, p, v};
use crate::Refusal;
use kentron::blend;
use kentron::draft;
use kentron::math::Transform;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(40.0, 30.0, 10.0)).build(m)?;
    let edge = edge_through(m, block, p(20.0, 30.0, 10.0))?;
    let rounded = blend::fillet(m, block, &[edge], 3.0)?;
    let k = 3.0 * (1.0 - 0.5f64.sqrt());
    let round = face_through(m, rounded, p(20.0, 30.0 - k, 10.0 - k))?;
    let top = face_through(m, rounded, p(20.0, 15.0, 10.0))?;
    Ok(draft::move_faces(m, rounded, &[top, round], &Transform::translation(v(0.0, 0.0, 6.0)))?)
}
