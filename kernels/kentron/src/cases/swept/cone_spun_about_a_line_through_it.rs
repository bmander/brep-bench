//! The solid a cone fills as it is spun about a line through it: the cone about the line through
//! the origin along w = (cos 30°, 0, sin 30°), its point at −2w, opening at 25° from its axis to a
//! flat base at 5w of radius 7 tan 25°, turned rigidly about the z axis by 150°.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis passes
//! through the cone, and kentron's sweep refuses a turn about an axis meeting the tool.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Cone;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (sin, cos) = 30f64.to_radians().sin_cos();
    let base = 7.0 * 25f64.to_radians().tan();
    let cone = Cone::new(0.0, base, 7.0).with_frame(frame(p(-2.0 * cos, 0.0, -2.0 * sin), Direction::new(cos, 0.0, sin))).build(m)?;
    sweep::solid(m, cone, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: 150f64.to_radians() })
}
