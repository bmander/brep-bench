//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges rounded to radius 2 in one fillet (each
//! corner closed by a patch of the sphere touching its four faces).
//!
//! The octahedron is two square pyramids lofted to points and united (as
//! `chamfer/octahedron-all-edges`); its edges are filleted together.
use crate::shared::chamfer::octahedron;
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let o = octahedron(k, 10.0)?;
    let edges = k.edges_but_seams(o)?;
    fillet(k, o, &edges, 2.0)
}
