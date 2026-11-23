//! Two slabs united where each has a flat B-spline face in the plane z = 4: the box from the
//! origin to (10, 10, 4), its top the bicubic Bézier patch on the grid x = 0, 2, 7, 10 by
//! y = 0, 4, 5, 10 at z = 4; and the box from (5, 3, 4) to (15, 13, 7), its bottom the biquadratic
//! Bézier patch on x = 5, 12, 15 by y = 3, 4, 13 at z = 4. The patches overlap on
//! 5 ≤ x ≤ 10, 3 ≤ y ≤ 10.
//!
//! Refused at run time by cad-kernel 0.21.0's Boolean: each slab is built and sewn shut (volumes
//! 400 and 300, both valid), but the union does not find the two flat patches lying in one plane
//! and meets them as surfaces crossing: "a bspline and a bspline meet at 0.00° at [10.0000,
//! 10.0000, 4.0000], nearly touching: their meeting is ill-conditioned there, not built yet"
//! (`ssi`). The operands the other way round, and `boolean::combined_written`, are refused the
//! same way.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{face, sheet};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;

const PLANE: f64 = 4.;

/// The Bézier patch of degree `xs.len() − 1` both ways whose control point (i, j) is
/// (xᵢ, yⱼ, 4).
fn patch(xs: &[f64], ys: &[f64]) -> Net {
    let clamped = |n: usize| [vec![0.; n], vec![1.; n]].concat();
    Net {
        du: xs.len() - 1,
        dv: ys.len() - 1,
        uknots: clamped(xs.len()),
        vknots: clamped(ys.len()),
        poles: xs.iter().map(|&x| ys.iter().map(|&y| [x, y, PLANE]).collect()).collect(),
        weights: None,
    }
}

/// The box from `lo` to `hi`, one of its faces at z = 4 the sheet of `net` and the other five
/// planar, sewn shut.
fn slab(lo: V, hi: V, net: Net) -> Result<Brep, String> {
    let at = |x: f64, y: f64, z: f64| [x, y, z];
    let ([x0, y0, z0], [x1, y1, z1]) = (lo, hi);
    let other = if z0 == PLANE { z1 } else { z0 };
    // each side walked so Newell's rule faces it out of the box
    let mut faces = vec![
        face(&polygon(&[at(x0, y0, z0), at(x1, y0, z0), at(x1, y0, z1), at(x0, y0, z1)]))?,
        face(&polygon(&[at(x1, y0, z0), at(x1, y1, z0), at(x1, y1, z1), at(x1, y0, z1)]))?,
        face(&polygon(&[at(x1, y1, z0), at(x0, y1, z0), at(x0, y1, z1), at(x1, y1, z1)]))?,
        face(&polygon(&[at(x0, y1, z0), at(x0, y0, z0), at(x0, y0, z1), at(x0, y1, z1)]))?,
    ];
    // the planar cap opposite the sheet, walked counter-clockwise seen from outside
    let mut cap = vec![at(x0, y0, other), at(x1, y0, other), at(x1, y1, other), at(x0, y1, other)];
    if other < PLANE { cap.reverse(); }
    faces.push(face(&polygon(&cap))?);
    faces.push(sheet(net)?);
    Ok(sewn(&faces, tolerance(30.))?)
}

pub fn build() -> Result<Brep, String> {
    let top = patch(&[0., 2., 7., 10.], &[0., 4., 5., 10.]);
    let first = slab([0., 0., 0.], [10., 10., PLANE], top)?;
    let bottom = patch(&[5., 12., 15.], &[3., 4., 13.]);
    let second = slab([5., 3., PLANE], [15., 13., 7.], bottom)?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
