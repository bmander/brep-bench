//! A block (9/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, from the floor) under the patch S(u, v) = (s², v, 1/2 +
//! 3s⁴/10 + v/10), s = 2u − 1 (a quartic in u, a line in v, folding back on itself along u = 1/2),
//! trimmed to 13/20 ≤ u ≤ 1, less the box 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3.
//!
//! The patch is cadrs's `NurbsSurface` from the case's control points, cut down to that part of
//! it (its control polygon along u split there), which the case allows: the same face. The block
//! is a rectangle extruded whose top is given it (Replace face); the
//! notch a box cut from it.
use crate::bench::{block, cut, p3};
use crate::shared::nurbs::{ruled_patch_from, trimmed_block};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let patch = ruled_patch_from(&[1.0, 0.0, -1.0 / 3.0, 0.0, 1.0], &[0.8, 0.2, 0.8, 0.2, 0.8], 0.65)?;
    let top = trimmed_block(k, (0.09, 0.0), (1.0, 1.0), 1.0, patch, ((0.0, 1.0), (0.0, 1.0)))?;
    let notch = block(k, p3(0.05, -1.0, -1.0), p3(0.5, 0.5, 3.0))?;
    cut(k, top, notch)
}
