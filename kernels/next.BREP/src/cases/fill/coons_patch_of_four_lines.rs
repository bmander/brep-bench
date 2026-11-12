//! The block under the Coons patch of four straight edges: the skew quadrilateral from (0, 0, 5) to
//! (10, 0, 5), (10, 10, 15) and (0, 10, 5), filled by the kernel's bilinearly blended Coons patch
//! (`coons_bilinear`, its boundaries in cyclic order, u along the edge on y = 0 and v along the
//! edge on x = 0), with the four upright sides and the floor z = 0 under it.
//!
//! The patch is the kernel's face on that surface, the floor and the four sides are flat faces
//! bounded by their edges, and the six are joined into the solid they bound
//! (`solids::sweep::block_under`).
use crate::solids::{p, polygon};
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{coons_bilinear, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let edges = polygon(&[p(0.0, 0.0, 5.0), p(10.0, 0.0, 5.0), p(10.0, 10.0, 15.0),
        p(0.0, 10.0, 5.0)])?;
    let patch = coons_bilinear([edges[0].clone(), edges[1].clone(), edges[2].clone(),
        edges[3].clone()]).map_err(|e| format!("no Coons patch: {e:?}"))?;
    block_under(&patch)
}
