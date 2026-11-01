//! A curved block grooved by a leaning torus cutter turned past it about the block's own axis.
//!
//! Refused: next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and
//! its swept envelope is a circle carried along one circular arc), and with the cutter's axis skew
//! to the turn the groove is an envelope none of the torus's own circles traces, which no
//! elementary composition of its operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and \
         its swept envelope lane is only a circle along one circular arc); a torus turned about an \
         axis skew to its own sweeps an envelope that would have to be computed outside the kernel",
    ))
}
