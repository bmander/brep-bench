//! A post, the cylinder of radius 4 on the z axis from z = 0 to z = 6, capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the rim where they meet, the circle of radius 4
//! at z = 6, rounded to radius 1 by a piece of a torus tangent to both; the round then deleted, the
//! cylinder running on up and the dome down until they meet again in that circle.
//!
//! The rounded post is the cylinder, a `prism` of a `disc`, joined to a `sphere` `kept` above z =
//! 6, the rim rolled by `fillet::roll` and the round's piece cut away (`sketch::rolled`); the
//! round, the face `fillet::roll` fits as a B-spline, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism, sphere};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let post = prism(&disc([0., 0., 0.], UP, 4.), 0., 6.)?;
    let cap = kept(&sphere([0., 0., 3.], UP, 5.)?, [0., 0., 6.], UP, 0.)?;
    let solid = combined(&post, &cap, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[4., 0., 6.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::BSpline(..)))
}
