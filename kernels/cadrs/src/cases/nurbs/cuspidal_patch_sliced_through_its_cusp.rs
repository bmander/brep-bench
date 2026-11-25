//! A block (1/100 ≤ x ≤ 1, 0 ≤ y ≤ 1, from the floor) under the patch S(u, v) = (s², v, 1/2 + s³ +
//! v/10), s = 2u − 1 (a cubic in u, a line in v), trimmed to 11/20 ≤ u ≤ 1, then cut down to the
//! plane z = 11y/10, which passes through the patch's cusp a hundredth beyond the top's edge.
//!
//! The patch is cadrs's `NurbsSurface` from the case's control points, cut down to that part of
//! it (its control polygon along u split there), which the case allows: the same face. The block
//! is a rectangle extruded whose top is given it (Replace face); the
//! part above the plane is cut away with a block on it.
use crate::bench::{p3, v3};
use crate::shared::nurbs::{ruled_patch_from, trimmed_block};
use crate::shared::singular::cut_beyond;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let patch = ruled_patch_from(&[1.0, -1.0 / 3.0, -1.0 / 3.0, 1.0], &[-0.5, 1.5, -0.5, 1.5], 0.55)?;
    let block = trimmed_block(k, (0.01, 0.0), (1.0, 1.0), 1.0, patch, ((0.0, 1.0), (0.0, 1.0)))?;
    cut_beyond(k, block, p3(0.5, 0.5, 0.55), v3(0.0, -1.1, 1.0))
}
