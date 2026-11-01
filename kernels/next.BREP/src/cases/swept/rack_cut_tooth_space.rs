//! A gear blank with one tooth space cut by a rack tooth rolled round it, as a gear is
//! generated.
//!
//! Refused: next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and
//! its swept envelope is a circle carried along one circular arc), and the space is the envelope
//! of the rack tooth in a rolling motion, which no elementary composition of its operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and \
         its swept envelope lane is only a circle along one circular arc); the tooth space is the \
         envelope of a rack tooth rolling on the blank, which would have to be computed outside \
         the kernel",
    ))
}
