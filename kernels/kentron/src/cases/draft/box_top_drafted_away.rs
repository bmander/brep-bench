//! The box from the origin to (30, 10, 10), its four sides drafted 30° with the pull direction
//! +z about the neutral plane z = 0, inward as they rise. The long sides meet at height 5√3,
//! below the top, so the draft consumes the top face: a hip roof of five faces, the box cut by
//! the four drafted planes.
//!
//! kentron's `draft` keeps the solid's faces and edges, finding them again on the drafted
//! surfaces, so it cannot close the sides over the top: it refuses, as OCCT's does. The block is
//! the floor extruded 10 up with its sides drafted as they go, the drafted prism, which meets the
//! sides in the ridge where they close.
use crate::bench::{p, v};
use crate::shared::loft::flat;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outline = [p(0.0, 0.0, 0.0), p(30.0, 0.0, 0.0), p(30.0, 10.0, 0.0), p(0.0, 10.0, 0.0)];
    let floor = flat(m, &outline)?;
    sweep::extrude_drafted(m, floor, v(0.0, 0.0, 10.0), 30f64.to_radians())
}
