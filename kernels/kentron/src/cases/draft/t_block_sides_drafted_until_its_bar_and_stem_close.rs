//! A T-block, the outline through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4)
//! and (0, 4) extruded from z = 0 to z = 8, its eight sides drafted 45° with the pull direction
//! +z about the neutral plane z = 0, inward as they rise: they meet in ridges over the bar and
//! the stem, below the top, which is gone.
//!
//! kentron's `draft` keeps the solid's faces and edges, finding them again on the drafted
//! surfaces, so it cannot close the sides over the top.
//! The block is the floor extruded 8 up with its sides drafted as they go, the drafted prism,
//! which meets the sides in ridges where they close.
use crate::bench::{p, v};
use crate::shared::loft::flat;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outline = [
        p(0.0, 0.0, 0.0),
        p(20.0, 0.0, 0.0),
        p(20.0, 4.0, 0.0),
        p(13.0, 4.0, 0.0),
        p(13.0, 16.0, 0.0),
        p(7.0, 16.0, 0.0),
        p(7.0, 4.0, 0.0),
        p(0.0, 4.0, 0.0),
    ];
    let floor = flat(m, &outline)?;
    sweep::extrude_drafted(m, floor, v(0.0, 0.0, 8.0), 45f64.to_radians())
}
