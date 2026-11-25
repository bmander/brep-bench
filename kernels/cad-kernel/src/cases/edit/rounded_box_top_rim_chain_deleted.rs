//! The box from the origin to (20, 12, 8), its four upright edges rounded to radius 3, then the
//! whole rim of its top rounded to radius 1, four quarter cylinders and four pieces of tori running
//! into one another in a closed chain; the eight faces of the chain then deleted together, the top
//! running on outward and the walls up until they meet in the top's rim again.
//!
//! The rounded box as the round's ball sweeps it: the box below the round, its outline a
//! `sketch::rounded_rectangle` swept up to z = 7 by `prism`, the box inset by the round's radius
//! stood on it up to the top, and the tube the ball sweeps joined to both — a cylinder along each
//! straight spine, a `build::torus` round each corner's — so the chain's faces are the quarter
//! cylinders and pieces of tori the description names. (Rolled by `fillet::roll` against the box,
//! the chain is fitted as B-splines, and the top and the walls run on meet them along their contact
//! lines only to the fit's tolerance: the deletion is refused, the two said to coincide there.) The
//! chain's eight faces, those on tori and on cylinders of radius 1, are deleted together
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let below = prism(&rounded_rectangle([0., 0.], [20., 12.], 3., 0.), 0., 7.)?;
    let inset = prism(&rounded_rectangle([1., 1.], [19., 11.], 2., 7.), 0., 1.)?;
    let mut rounded = combined(&below, &inset, Op::Union, 0.)?;
    // the tube's middle runs 1 in from the walls at z = 7: a cylinder along each straight spine, a
    // torus round each corner's (the rest of each torus inside the box)
    let tubes = [
        prism(&disc([3., 1., 7.], [1., 0., 0.], 1.), 0., 14.)?,
        prism(&disc([3., 11., 7.], [1., 0., 0.], 1.), 0., 14.)?,
        prism(&disc([1., 3., 7.], [0., 1., 0.], 1.), 0., 6.)?,
        prism(&disc([19., 3., 7.], [0., 1., 0.], 1.), 0., 6.)?,
        torus([3., 3., 7.], UP, 2., 1.)?,
        torus([17., 3., 7.], UP, 2., 1.)?,
        torus([17., 9., 7.], UP, 2., 1.)?,
        torus([3., 9., 7.], UP, 2., 1.)?,
    ];
    for t in &tubes {
        rounded = combined(&rounded, t, Op::Union, 0.)?;
    }
    deleted_where(&rounded, |s| match s {
        Surface::Torus(..) => true,
        Surface::Cylinder(_, r) => *r < 2.,
        _ => false,
    })
}
