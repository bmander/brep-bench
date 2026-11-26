//! The solid a torus fills as it tumbles: the torus centred at (3, 0, 2), axis parallel to the
//! z axis, radii 1 and 0.22, turned by 0.7t about the vertical line through (2, 0, 0) and then by
//! −t about the line through (0, h cos 45°, h sin 45°), h = 0.7071, along
//! (cos 20°, sin 20° cos 45°, sin 20° sin 45°), for t from −100° to 100°.
//!
//! kentron's sweep of a solid (`sweep::solid`) under the tumble given by its places
//! (`shared::swept::crossed_tumble`), which refuses it: the curves where the motion grazes the tool move over it, so its envelope is
//! no product of a fixed curve and the motion, and kentron's sweep builds only those.
use crate::shared::swept::crossed_tumble;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    crossed_tumble(m, -1.0)
}
