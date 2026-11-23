//! A block over the square from (0, 0) to (10, 10), capped by the bilinearly blended Coons patch
//! of four parabolic arcs (quadratic Bézier curves in the four side planes), a saddle.
//!
//! The patch is `fill::coons` of the four arcs; the four sides under them and the floor are flat
//! faces, all sewn into the block's shell.
use crate::bench::p;
use crate::shared::geometry::{flat, segment, solid_of};
use crate::shared::spline::bezier;
use crate::Refusal;
use kentron::topology::{Model, Shape};
use kentron::{fill, make};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // Each arc by its control points, end to end round the square.
    let arcs = [
        [(0.0, 0.0, 10.0), (5.0, 0.0, 14.0), (10.0, 0.0, 10.0)],
        [(10.0, 0.0, 10.0), (10.0, 5.0, 6.0), (10.0, 10.0, 10.0)],
        [(10.0, 10.0, 10.0), (5.0, 10.0, 6.0), (0.0, 10.0, 10.0)],
        [(0.0, 10.0, 10.0), (0.0, 5.0, 14.0), (0.0, 0.0, 10.0)],
    ];
    let edges = arcs.iter().map(|a| make::edge_from_curve(m, bezier(a))).collect::<kentron::Result<Vec<_>>>()?;
    let mut faces = vec![fill::coons(m, &edges)?];
    // Under each arc, the side down to the floor.
    for (a, &arc) in arcs.iter().zip(&edges) {
        let ((x0, y0, z0), (x1, y1, z1)) = (a[0], a[2]);
        let down = segment(m, p(x1, y1, z1), p(x1, y1, 0.0))?;
        let along = segment(m, p(x1, y1, 0.0), p(x0, y0, 0.0))?;
        let up = segment(m, p(x0, y0, 0.0), p(x0, y0, z0))?;
        faces.push(flat(m, &[arc, down, along, up])?);
    }
    let corners = [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 10.0, 0.0), p(0.0, 10.0, 0.0)];
    let floor = (0..4).map(|i| segment(m, corners[i], corners[(i + 1) % 4])).collect::<kentron::Result<Vec<_>>>()?;
    faces.push(flat(m, &floor)?);
    solid_of(m, &faces)
}
