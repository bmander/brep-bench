//! A disc with a crowned top, the cylinder of radius 100 about the z axis from z = 0 up past the
//! crown in common with the ball of radius 800 about (0, 0, 3 − 100√63), whose sphere passes
//! through the rim's top circle at z = 3; joined to an obround vane, the plan within 2 of the
//! segment from (22, 0) to (88, 0) drawn with kentron's pen at z = 1 and extruded to z = 31. The
//! loop where the vane's sides meet the crown, one tangent chain, rounded to radius 2.
use crate::bench::{edge_through, frame, p};
use crate::shared::fillet::plan_prism;
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::sketch::Step::{Arc, Forward, Start};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let centre = 3.0 - 100.0 * 63f64.sqrt();
    let drum = Cylinder::new(100.0, 20.0).build(m)?;
    let ball = Sphere::new(800.0).with_frame(frame(p(0.0, 0.0, centre), Direction::Z)).build(m)?;
    let disc = boolean::intersection(m, drum, ball)?;
    let foot = [
        Start(22.0, -2.0),
        Forward(66.0), Arc { radius: 2.0, angle: 180.0 },
        Forward(66.0), Arc { radius: 2.0, angle: 180.0 },
    ];
    let vane = plan_prism(m, 1.0, &foot, 30.0)?;
    let part = boolean::union(m, disc, vane)?;
    // the loop where the vane's south side, y = −2, meets the sphere, at x = 55
    let z = centre + (800.0f64 * 800.0 - 55.0 * 55.0 - 4.0).sqrt();
    let root = edge_through(m, part, p(55.0, -2.0, z))?;
    Ok(blend::fillet(m, part, &[root], 2.0)?)
}
