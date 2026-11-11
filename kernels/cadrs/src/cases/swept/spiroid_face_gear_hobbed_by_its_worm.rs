//! A face gear hobbed whole by a tapered single-start worm: its thread a rack's trapezoid carried
//! along a conical helix, each section kept in its half-plane through the worm's axis.
//!
//! Refused: cadrs sweeps a section along a conical helix keeping its angle to the path (or its
//! orientation, or a direction in its plane), and none of these keeps it in the plane through the
//! axis as the radius grows, so the tapered thread itself is not built; and what a solid that is no
//! polyhedron, torus or solid of revolution fills under two turns, cadrs finds only as a mesh.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no sweep that carries a section along a conical helix in its plane through the \
         axis (a tapered thread), and sweeps such a tool under two turns only as a mesh",
    ))
}
