//! The solid a thin torus fills as it tumbles: the torus centred at (2.02, 0, 1), axis parallel to
//! the z axis, radii 1 and 0.08, turned by 1.01t about the vertical line through (2, 0, 0) and then
//! by −t about the line through the origin along (cos 89°, sin 89° cos 1°, sin 89° sin 1°), for t
//! from −180° to 180°.
//!
//! The motion is given by its places (`shared::swept::tumble`, a `sweep::RigidMotion::Carried`)
//! for kentron's sweep of a solid (`sweep::solid`), which refuses it: the curves where the motion grazes the tool move over it, so its envelope is
//! no product of a fixed curve and the motion, and kentron's sweep builds only those.
use crate::bench::{d, frame, p};
use crate::shared::swept::tumble;
use kentron::math::{Axis, Direction};
use kentron::primitives::Torus;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let tool = Torus::new(1.0, 0.08).with_frame(frame(p(2.02, 0.0, 1.0), Direction::Z)).build(m)?;
    let first = Axis::new(p(2.0, 0.0, 0.0), Direction::Z);
    let motion = tumble(first, 1.01, Axis::new(p(0.0, 0.0, 0.0), d(89f64.to_radians().cos(), 89f64.to_radians().sin() * 1f64.to_radians().cos(), 89f64.to_radians().sin() * 1f64.to_radians().sin())), -1.0, -180.0, 180.0);
    sweep::solid(m, tool, &motion)
}
