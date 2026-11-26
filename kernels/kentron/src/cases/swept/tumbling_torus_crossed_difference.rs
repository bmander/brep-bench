//! The difference of the two torus tumbles of `swept/tumbling-torus-crossed-forward` and
//! `swept/tumbling-torus-crossed-reverse`: the torus centred at (3, 0, 2), radii 1 and 0.22,
//! turned by 0.7t about the vertical line through (2, 0, 0) and then by ∓t about a second, skew
//! line, for t from −100° to 100°, each sweep over its own t.
//!
//! Each tumble is kentron's sweep of a solid (`sweep::solid`) under the tumble given by its places
//! (`shared::swept::crossed_tumble`); the difference is kentron's boolean of the two.
//! kentron's sweep refuses each tumble: the curves where the motion grazes the torus move over it,
//! so its envelope is no product of a fixed curve and the motion.
use crate::shared::swept::crossed_tumble;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let forward = crossed_tumble(m, -1.0)?;
    let reverse = crossed_tumble(m, 1.0)?;
    kentron::boolean::difference(m, forward, reverse)
}
