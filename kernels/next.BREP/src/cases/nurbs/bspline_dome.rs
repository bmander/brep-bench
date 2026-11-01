//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, its four sides upright, and its top
//! a clamped bicubic B-spline surface of 6 × 5 control points, with a double knot at u = 2/5.
//!
//! The surface is a kernel `NurbsSurface` with the description's knots and poles (unit weights);
//! next.BREP makes no solid from a surface, so the block is assembled under it by
//! `solids::sweep::block_under`.
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};

const XS: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const YS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
/// zᵢⱼ: row i at x = xᵢ, column j at y = yⱼ.
const HEIGHTS: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

pub fn build() -> Result<BrepSolid, Refusal> {
    let net = (0..6)
        .map(|i| (0..5).map(|j| Vec4::from_point(p(XS[i], YS[j], HEIGHTS[i][j]), 1.0)).collect())
        .collect();
    let along_u = vec![0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0];
    let along_v = vec![0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0];
    block_under(&NurbsSurface::new(3, 3, along_u, along_v, net)?)
}
