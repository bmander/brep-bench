//! A dog-bone plate, from z = 0 to 10, with a round boss of the same radius standing on one of its
//! lobes. The plate's outline is drawn with kentron's pen: two round lobes of radius 10 about
//! (∓30, 0), a waist between the lines y = ±4, and four concave blends of radius 6 between each
//! side and each lobe, every piece running on tangent into the next. Each blend's centre is 16
//! from its lobe's and 6 off its side, so it turns through the angle whose cosine is 10/16, and
//! each lobe's outer arc through 180° and twice that. The boss, the cylinder of radius 10 about
//! the upright through (30, 0) from z = 0 to 30, is joined to the plate: its side and the lobe's
//! are the same cylinder up to the plate's top.
use crate::bench::{frame, p};
use crate::shared::fillet::plan_prism;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::sketch::Step::{Arc, Forward, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // where each blend meets its side, ±(30 − 2√39) along it
    let a = 30.0 - 2.0 * 39f64.sqrt();
    let blend = (10.0f64 / 16.0).acos().to_degrees();
    let lobe = 180.0 + 2.0 * blend;
    // anticlockwise from the south side's west end
    let outline = [
        Start(-a, -4.0),
        Forward(2.0 * a), Arc { radius: -6.0, angle: blend }, Arc { radius: 10.0, angle: lobe }, Arc { radius: -6.0, angle: blend },
        Forward(2.0 * a), Arc { radius: -6.0, angle: blend }, Arc { radius: 10.0, angle: lobe }, Arc { radius: -6.0, angle: blend },
    ];
    let plate = plan_prism(m, 0.0, &outline, 10.0)?;
    let boss = Cylinder::new(10.0, 30.0).with_frame(frame(p(30.0, 0.0, 0.0), Direction::Z)).build(m)?;
    boolean::union(m, plate, boss)
}
