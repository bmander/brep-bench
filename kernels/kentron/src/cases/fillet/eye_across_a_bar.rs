//! A bar, the box from (−40, −8, −5) to (40, 8, 5), joined to an eye across its middle, the
//! cylinder of radius 14 about the y axis from y = −8 to 8, its ends flush with the bar's sides;
//! the four straight concave edges where the eye's side meets the bar's top and bottom, at
//! x = ±√171, z = ±5, rounded to radius 3.
use crate::bench::{edge_through, frame, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let bar = Cuboid::from_corners(p(-40.0, -8.0, -5.0), p(40.0, 8.0, 5.0)).build(m)?;
    let eye = Cylinder::new(14.0, 16.0).with_frame(frame(p(0.0, -8.0, 0.0), Direction::Y)).build(m)?;
    let part = boolean::union(m, bar, eye)?;
    let x = 171f64.sqrt();
    let mut feet = Vec::new();
    for (sx, sz) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
        feet.push(edge_through(m, part, p(sx * x, 0.0, sz * 5.0))?);
    }
    Ok(blend::fillet(m, part, &feet, 3.0)?)
}
