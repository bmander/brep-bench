//! A rod grooved by a torus wheel carried along a screw about its axis, as a thread is ground.
//!
//! Refused: next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and
//! its swept envelope is a circle carried along one circular arc), and the groove is the envelope
//! of a torus in a screw motion, which no elementary composition of its operations gives.
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    Err(Refusal::from(
        "next.BREP has no sweep of a solid (its sweeps carry a planar profile along a path, and \
         its swept envelope lane is only a circle along one circular arc); the groove is the \
         envelope of a leaning torus screwed about the rod, which would have to be computed \
         outside the kernel",
    ))
}
