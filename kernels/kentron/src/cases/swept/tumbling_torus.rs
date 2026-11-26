//! The solid a torus fills as it tumbles: the torus centred at (3, 0, 2), axis parallel to the
//! z axis, radii 1 and 0.5, turned by t/4 about the vertical line through (2, 0, 0) and then by
//! −t about the line through (0, 0.5, 0.5) parallel to the x axis, for t from −75° to 75°.
//!
//! The motion is given by its places (`shared::swept::tumble`, a `sweep::RigidMotion::Carried`)
//! for kentron's sweep of a solid (`sweep::solid`), which refuses it: the curves where the motion grazes the tool move over it, so its envelope is
//! no product of a fixed curve and the motion, and kentron's sweep builds only those.
use crate::bench::{frame, p};
use crate::shared::swept::tumble;
use kentron::math::{Axis, Direction};
use kentron::primitives::Torus;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let tool = Torus::new(1.0, 0.5).with_frame(frame(p(3.0, 0.0, 2.0), Direction::Z)).build(m)?;
    let first = Axis::new(p(2.0, 0.0, 0.0), Direction::Z);
    let motion = tumble(first, 0.25, Axis::new(p(0.0, 0.5, 0.5), Direction::X), -1.0, -75.0, 75.0);
    sweep::solid(m, tool, &motion)
}
