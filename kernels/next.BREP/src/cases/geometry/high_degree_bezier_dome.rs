//! A block on the square 0 ≤ x, y ≤ 20 of the xy plane, floor at z = 0 and four upright sides,
//! under the non-rational Bézier patch of degree 15 each way whose 16 × 16 control points are
//! (20i/15, 20j/15, 4 + ((3i + 5j) mod 7)).
//!
//! The patch is the kernel's face on that surface, and the floor and the four sides flat faces
//! bounded by their edges, a side's top edge the patch's own edge curve of degree 15; the six are
//! joined into the solid they bound (`solids::sweep::block_under`).
use crate::solids::p;
use crate::solids::sweep::block_under;
use crate::Refusal;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};

const DEGREE: usize = 15;

pub fn build() -> Result<BrepSolid, Refusal> {
    let at = |k: usize| 20.0 * k as f64 / DEGREE as f64;
    let height = |i: usize, j: usize| (4 + (3 * i + 5 * j) % 7) as f64;
    let net = (0..=DEGREE)
        .map(|i| {
            (0..=DEGREE).map(|j| Vec4::from_point(p(at(i), at(j), height(i, j)), 1.0)).collect()
        })
        .collect();
    let clamped = [vec![0.0; DEGREE + 1], vec![1.0; DEGREE + 1]].concat();
    let patch = NurbsSurface::new(DEGREE, DEGREE, clamped.clone(), clamped, net)?;

    block_under(&patch)
}
