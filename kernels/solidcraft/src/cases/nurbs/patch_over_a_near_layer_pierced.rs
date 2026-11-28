//! The block under the part 13/20 ≤ u ≤ 1 of the Bézier patch of degree 4 in u and 1 in v,
//! control points (xᵢ, j, zᵢ + j/10), whose folded-back layer lies a few ten-thousandths beneath
//! it, less the box 2/5 ≤ x ≤ 7/10, 3/10 ≤ y ≤ 3/5, −1 ≤ z ≤ 3: a square hole through it.
//!
//! Built as `nurbs/folded-patch-notched-at-its-rim` builds its block, on solidcore: the kernel's
//! NURBS sheets are of odd degree only. As the case allows, the top face is built on the patch
//! cut down to its part (the same face).
use super::folded_patch_notched_at_its_rim::block;
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let xs = [1.0, 0.0, -1.0 / 3.0, 0.0, 1.0];
    let zs = [799.0 / 1000.0, 401.0 / 2000.0, 0.8, 399.0 / 2000.0, 801.0 / 1000.0];
    let a = block(xs, zs, 0.65)?;
    cut(a, make_box([0.4, 0.3, -1.0], [0.3, 0.3, 4.0])?)
}
