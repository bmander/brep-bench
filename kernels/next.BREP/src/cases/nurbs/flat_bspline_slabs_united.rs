//! Two slabs united along the plane z = 4, where each has a flat B-spline face: the box from the
//! origin to (10, 10, 4) whose top is the bicubic Bézier patch on the grid x = 0, 2, 7, 10 by
//! y = 0, 4, 5, 10 at z = 4, and the box from (5, 3, 4) to (15, 13, 7) whose bottom is the
//! biquadratic Bézier patch on the grid x = 5, 12, 15 by y = 3, 4, 13 at z = 4.
//!
//! Each slab's patch is the kernel's face on that surface, its other five faces flat faces bounded
//! by their edges, and the six are joined into the solid they bound (`solids::sweep::block_to`):
//! the first slab between its patch and the floor z = 0, the second between its patch and the
//! plane z = 7 above it.
use crate::solids::fillet::union;
use crate::solids::p;
use crate::solids::sweep::{block_to, block_under};
use crate::Refusal;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};

/// The flat Bézier patch on the grid `xs` by `ys` at height `z`.
fn flat_patch(xs: &[f64], ys: &[f64], z: f64) -> Result<NurbsSurface, String> {
    let net = xs.iter().map(|&x| ys.iter().map(|&y| Vec4::from_point(p(x, y, z), 1.0)).collect())
        .collect();
    let degree = xs.len() - 1;
    let knots = [vec![0.0; degree + 1], vec![1.0; degree + 1]].concat();
    NurbsSurface::new(degree, degree, knots.clone(), knots, net)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = block_under(&flat_patch(&[0.0, 2.0, 7.0, 10.0], &[0.0, 4.0, 5.0, 10.0], 4.0)?)?;
    let second = block_to(&flat_patch(&[5.0, 12.0, 15.0], &[3.0, 4.0, 13.0], 4.0)?, 7.0)?;
    Ok(union(&first, &second)?)
}
