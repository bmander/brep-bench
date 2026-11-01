//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, and its top
//! the bicubic Bézier patch whose sixteen control points stand over an even 4 × 4 grid of the
//! square at the heights `HEIGHTS`.
//!
//! The patch is a kernel `NurbsSurface` with the description's poles (Bézier knots, unit weights);
//! next.BREP makes no solid from a surface, so the block is assembled under it by
//! `solids::sweep::block_under`.
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};

/// zᵢⱼ: row i at x = 20i/3, column j at y = 20j/3.
pub const HEIGHTS: [[f64; 4]; 4] = [
    [4.0, 6.0, 7.0, 5.0],
    [6.0, 14.0, 12.0, 7.0],
    [5.0, 13.0, 16.0, 8.0],
    [3.0, 6.0, 8.0, 6.0],
];

/// The dome: S(u, v) = Σᵢ Σⱼ Bᵢ(u) Bⱼ(v) Pᵢⱼ, Pᵢⱼ = (20i/3, 20j/3, zᵢⱼ).
pub fn dome() -> Result<NurbsSurface, String> {
    let net = (0..4)
        .map(|i| (0..4).map(|j| {
            Vec4::from_point(p(20.0 * i as f64 / 3.0, 20.0 * j as f64 / 3.0, HEIGHTS[i][j]), 1.0)
        }).collect())
        .collect();
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    NurbsSurface::new(3, 3, bezier.clone(), bezier, net)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    block_under(&dome()?)
}
