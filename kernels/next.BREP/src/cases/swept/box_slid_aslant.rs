//! Everything the box from (0, 0, 0) to (10, 6, 4) passes through as it slides without turning
//! along the vector (8, 5, 3).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the box as a convex polyhedron, its
//! eight corners and six faces, slid by (8, 5, 3). The kernel bounds the sweep by the box's back
//! faces where it starts, its front faces where it stops, and the sheets its silhouette edges sweep
//! between.
use crate::solids::p;
use crate::solids::swept::{box_tool, slide_by};
use crate::Refusal;
use brep_kernel::{sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let tool = box_tool(p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(0.0, 6.0, 0.0), p(0.0, 0.0, 4.0));
    Ok(sweep_solid(&tool, &slide_by(p(8.0, 5.0, 3.0)))?)
}
