//! A wheel cut by a plus-shaped tool turning in mesh with it, as a gear shaper cuts.
//!
//! Refused: next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and
//! its swept envelope is a circle carried along one circular arc), and the cut is the envelope of
//! the tool in a rolling motion, which no elementary composition of its operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and \
         its swept envelope lane is only a circle along one circular arc); the cut is the envelope \
         of a plus turning in mesh with the wheel, which would have to be computed outside the \
         kernel",
    ))
}
