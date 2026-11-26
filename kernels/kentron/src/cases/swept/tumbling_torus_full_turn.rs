//! The solid a torus fills as it tumbles: the torus centred at (3, 0, 2), axis parallel to the
//! z axis, radii 1 and 0.16, turned by 0.375t about the vertical line through (2, 0, 0) and then
//! by −t about the line through (0, h cos 30°, h sin 30°), h = 0.7071, along
//! (cos 20°, sin 20° cos 30°, sin 20° sin 30°), for t from −180° to 180°.
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

/// The line through (0, h cos a, h sin a), h = 0.7071, along (cos 20°, sin 20° cos a, sin 20° sin a).
fn skew(a: f64) -> Axis {
    let (sa, ca) = a.to_radians().sin_cos();
    let (s20, c20) = 20f64.to_radians().sin_cos();
    Axis::new(p(0.0, 0.7071 * ca, 0.7071 * sa), d(c20, s20 * ca, s20 * sa))
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let tool = Torus::new(1.0, 0.16).with_frame(frame(p(3.0, 0.0, 2.0), Direction::Z)).build(m)?;
    let first = Axis::new(p(2.0, 0.0, 0.0), Direction::Z);
    let motion = tumble(first, 0.375, skew(30f64), -1.0, -180.0, 180.0);
    sweep::solid(m, tool, &motion)
}
