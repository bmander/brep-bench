//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (15.0, 20.0), (15.0, 5.0), (0.0, 5.0)];

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = shell::plate(m, &OUTLINE, 20.0)?;
    // joined by intersection: the moved faces extended until they meet
    shell::outward(m, block, &[p(7.5, 5.0, 10.0)], 1.0, Join::Intersection)
}
