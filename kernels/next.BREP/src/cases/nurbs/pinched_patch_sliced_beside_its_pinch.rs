//! A block under a bicubic Bézier patch pinched to a point beside it, sliced by a plane past the
//! pinch.
//!
//! The patch is S(u, v) = (s³, t³, 1/2 + st), s = 2u − 1, t = 2v − 1: singular at u = v = 1/2
//! alone. The block stands on z = 0 under the part 11/20 ≤ u, v ≤ 1, its top a face on the patch
//! cut down to that square, which the case allows as the same face (`bezier_part`), and the part where −x + y + z/10 ≤ 13/250
//! is kept (`split_solid_by_plane`).
use crate::solids::p;
use crate::solids::sweep::{bezier_part, block_under};
use crate::Refusal;
use brep_kernel::{split_solid_by_plane, BrepSolid, NurbsSurface, Vec4};

pub fn build() -> Result<BrepSolid, Refusal> {
    let xs = [-1.0, 1.0, -1.0, 1.0];
    let ss = [-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0];
    let net = (0..4)
        .map(|i| (0..4).map(|j| Vec4::from_point(p(xs[i], xs[j], 0.5 + ss[i] * ss[j]), 1.0)).collect())
        .collect();
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let patch = NurbsSurface::new(3, 3, bezier.clone(), bezier, net)?;
    let block = block_under(&bezier_part(&patch, 11.0 / 20.0, 11.0 / 20.0)?)?;
    // the plane −x + y + z/10 = 13/250 through (0, 13/250, 0), its normal (−1, 1, 1/10)
    let (inside, _outside) = split_solid_by_plane(&block, p(0.0, 13.0 / 250.0, 0.0), p(-1.0, 1.0, 0.1))?;
    Ok(inside)
}
