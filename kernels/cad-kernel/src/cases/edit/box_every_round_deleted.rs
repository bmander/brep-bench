//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2, twelve quarter
//! cylinders along its edges and eight eighths of a ball at its corners; all twenty rounds then
//! deleted, the six flat faces running on until they meet in the box's twelve edges and eight
//! corners.
//!
//! The rounded box is the box, a `prism`, its twelve edges rounded together by `rounds::rounded`,
//! which closes each corner with its ball; every face of it not a plane, the twenty rounds, is
//! deleted together (`defeature::deleted`).
use crate::sketch::block;
use cad_kernel::defeature::deleted;
use cad_kernel::geom::{tolerance, Surface};
use cad_kernel::rounds::rounded;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = block([0., 0., 0.], [20., 16., 12.])?;
    let edges: Vec<usize> = (0..b.edges.len()).collect();
    let rounded = rounded(&b, &edges, 2., tolerance(b.size()))?;
    let rounds: Vec<usize> = (0..rounded.faces.len())
        .filter(|&f| !matches!(rounded.faces[f].surface, Surface::Plane(_)))
        .collect();
    Ok(deleted(&rounded, &rounds, tolerance(rounded.size()))?)
}
