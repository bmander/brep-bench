//! The block under the part 13/20 ≤ u ≤ 1 of the Bézier patch of degree 4 in u and 1 in v,
//! control points (xᵢ, j, zᵢ + j/10), which folds back over itself along u = 1/2, less the box
//! 1/20 ≤ x ≤ 1/2, −1 ≤ y ≤ 1/2, −1 ≤ z ≤ 3 notching its corner at x = 9/100, y = 0.
//!
//! solidcraft-kernel's NURBS sheets are T-splines', of odd degree only, so the block is built on
//! solidcore: its faces made in the kernel and sewn by ops (`graph_block`). As the case allows,
//! the top face is built on the patch cut down to its part (the same face), by the geometry
//! crate's de Casteljau subdivision.
use super::cuspidal_patch_sliced_through_its_cusp::piece;
use crate::bench::cut;
use crate::shared::nurbs::{bezier_knots, graph_block};
use crate::solids::{Vec3, v3};
use crate::Refusal;
use solidcore_ops::{make_box, Body};

/// The block under the part u ≥ `from` of the patch of degree 4 in u and 1 in v whose control
/// points are (`xs[i]`, j, `zs[i]` + j/10).
pub fn block(xs: [f64; 5], zs: [f64; 5], from: f64) -> Result<Body, Refusal> {
    let net: Vec<Vec<Vec3>> =
        (0..5).map(|i| (0..2).map(|j| v3(xs[i], j as f64, zs[i] + j as f64 / 10.0)).collect()).collect();
    let part = piece(&net, from, 0.0)?;
    let xs: Vec<f64> = part.iter().map(|row| row[0].x).collect();
    let heights: Vec<Vec<f64>> = part.iter().map(|row| row.iter().map(|p| p.z).collect()).collect();
    graph_block(&xs, &[0.0, 1.0], &heights, &bezier_knots(4), &bezier_knots(1))
}

pub fn build() -> Result<Body, Refusal> {
    let xs = [1.0, 0.0, -1.0 / 3.0, 0.0, 1.0];
    let a = block(xs, [0.8, 0.2, 0.8, 0.2, 0.8], 0.65)?;
    cut(a, make_box([0.05, -1.0, -1.0], [0.45, 1.5, 4.0])?)
}
