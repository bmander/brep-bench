//! A square plate, the box from (−50, −50, 0) to (50, 50, 20), joined to a round post standing
//! across its edge, the cylinder of radius 15 about the upright through (45, 0) from z = 0 to 40;
//! a hole of radius 6 drilled right through along the post's axis (a cylinder reaching past the
//! bottom and the post's top), and both of the hole's rims chamfered 1.5 at 45°. The lower rim
//! crosses the line x = 50 where the plate's bottom and the post's overhanging foot meet in one
//! plane.
use crate::bench::{edge_through, frame, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-50.0, -50.0, 0.0), p(50.0, 50.0, 20.0)).build(m)?;
    let post = Cylinder::new(15.0, 40.0).with_frame(frame(p(45.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, plate, post)?;
    let drill = Cylinder::new(6.0, 50.0).with_frame(frame(p(45.0, 0.0, -5.0), Direction::Z)).build(m)?;
    let part = boolean::difference(m, part, drill)?;
    // each rim picked at one point, 135° round from +x: a chamfer runs on along the edges
    // continuing it tangentially, round the whole rim
    let k = 3.0 * 2f64.sqrt();
    let bottom = edge_through(m, part, p(45.0 - k, k, 0.0))?;
    let top = edge_through(m, part, p(45.0 - k, k, 40.0))?;
    let rims = [bottom, top];
    Ok(blend::chamfer(m, part, &rims, 1.5)?)
}
