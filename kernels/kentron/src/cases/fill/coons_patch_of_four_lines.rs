//! A block over the square from (0, 0) to (10, 10), capped by the Coons patch of the four
//! straight edges (0, 0, 5) → (10, 0, 5) → (10, 10, 15) → (0, 10, 5) → (0, 0, 5): the hyperbolic
//! paraboloid z = 5 + xy/10.
//!
//! The patch is `fill::coons` of the four edges; the four sides under them and the floor are flat
//! faces, all sewn into the block's shell.
use crate::bench::p;
use crate::shared::geometry::{flat, segment, solid_of};
use crate::Refusal;
use kentron::fill;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let top = [p(0.0, 0.0, 5.0), p(10.0, 0.0, 5.0), p(10.0, 10.0, 15.0), p(0.0, 10.0, 5.0)];
    let floor = top.map(|c| p(c.x, c.y, 0.0));
    let edges = (0..4).map(|i| segment(m, top[i], top[(i + 1) % 4])).collect::<kentron::Result<Vec<_>>>()?;
    let mut faces = vec![fill::coons(m, &edges)?];
    // Under each edge, the side down to the floor.
    for i in 0..4 {
        let j = (i + 1) % 4;
        let down = segment(m, top[j], floor[j])?;
        let along = segment(m, floor[j], floor[i])?;
        let up = segment(m, floor[i], top[i])?;
        faces.push(flat(m, &[edges[i], down, along, up])?);
    }
    let bottom = (0..4).map(|i| segment(m, floor[i], floor[(i + 1) % 4])).collect::<kentron::Result<Vec<_>>>()?;
    faces.push(flat(m, &bottom)?);
    solid_of(m, &faces)
}
