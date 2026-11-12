//! The block under the Coons patch of four parabolic arcs, each the quadratic Bézier curve of its
//! three control points in a vertical side of the square from (0, 0) to (10, 10), filled by the
//! kernel's bilinearly blended Coons patch (`coons_bilinear`, its boundaries in cyclic order, u
//! along the arc on y = 0 and v along the arc on x = 0), with the four upright sides and the floor
//! z = 0 under it.
//!
//! The patch is the kernel's face on that surface, the floor and the four sides are flat faces
//! bounded by their edges, and the six are joined into the solid they bound
//! (`solids::sweep::block_under`).
use crate::solids::p;
use crate::solids::sweep::{bezier, block_under};
use crate::Refusal;
use brep_kernel::{coons_bilinear, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let arcs = [
        bezier(&[p(0.0, 0.0, 10.0), p(5.0, 0.0, 14.0), p(10.0, 0.0, 10.0)])?,
        bezier(&[p(10.0, 0.0, 10.0), p(10.0, 5.0, 6.0), p(10.0, 10.0, 10.0)])?,
        bezier(&[p(10.0, 10.0, 10.0), p(5.0, 10.0, 6.0), p(0.0, 10.0, 10.0)])?,
        bezier(&[p(0.0, 10.0, 10.0), p(0.0, 5.0, 14.0), p(0.0, 0.0, 10.0)])?,
    ];
    let patch = coons_bilinear(arcs).map_err(|e| format!("no Coons patch: {e:?}"))?;
    block_under(&patch)
}
