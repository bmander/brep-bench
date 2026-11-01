//! The solid a torus fills as it tumbles, turning about one axis while seen from a frame turning
//! about another.
//!
//! Refused: next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and
//! its swept envelope is a circle carried along one circular arc), and the boundary is an envelope
//! of no simpler kind, which no elementary composition of its operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and \
         its swept envelope lane is only a circle along one circular arc); the tumbling torus's \
         boundary is an envelope that would have to be computed outside the kernel",
    ))
}
