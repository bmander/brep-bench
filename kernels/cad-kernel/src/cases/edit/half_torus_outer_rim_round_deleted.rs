//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the circle
//! of radius 6, its flat the ring 4 ≤ r ≤ 8 at z = 0, its outer rim rounded to radius 1/2 by a
//! piece of the torus about the circle of radius 6 + √2 at z = 1/2; the round then deleted, the
//! flat running on outward and the torus down until they meet in the circle of radius 8 at z = 0.
//!
//! The rounded half torus is a `build::torus` `kept` above z = 0, its outer rim rolled by
//! `fillet::roll` and the round's piece cut away (`sketch::rolled`); the round, the face
//! `fillet::roll` fits as a B-spline, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::{kept, torus};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = kept(&torus([0., 0., 0.], UP, 6., 2.)?, [0., 0., 0.], UP, 0.)?;
    let rounded = rolled(&solid, &[[8., 0., 0.]], 0.5)?;
    deleted_where(&rounded, |s| matches!(s, Surface::BSpline(..)))
}
