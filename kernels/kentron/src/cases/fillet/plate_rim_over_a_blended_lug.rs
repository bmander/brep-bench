//! A plate from (−40, −25, 0) to (40, 25, 13), its four upright edges rounded to radius 10, joined
//! to a round lug, the cylinder of radius 9 about the upright through (0, 25) from z = 0 to 13; the
//! two concave upright edges where the lug meets the north side, at (±9, 25), rounded to radius 8;
//! then the rim of the bottom, twelve edges in one tangent chain over the lug and its blends,
//! rounded to radius 2.
use crate::bench::{edge_through, frame, p};
use crate::shared::draft::rounded_box;
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = rounded_box(m, p(-40.0, -25.0, 0.0), p(40.0, 25.0, 13.0), 10.0)?;
    let lug = Cylinder::new(9.0, 13.0).with_frame(frame(p(0.0, 25.0, 0.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, plate, lug)?;
    let east = edge_through(m, part, p(9.0, 25.0, 6.5))?;
    let west = edge_through(m, part, p(-9.0, 25.0, 6.5))?;
    let part = blend::fillet(m, part, &[east, west], 8.0)?;
    // the bottom rim under the middle of the south side: the round runs on round the whole chain
    let rim = edge_through(m, part, p(0.0, -25.0, 0.0))?;
    Ok(blend::fillet(m, part, &[rim], 2.0)?)
}
