//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1 by a
//! piece of the torus whose tube has radius 1 about the circle of radius √15 at z = 1; the round
//! then deleted, the flat running on outward and the ball down until they meet in the circle of
//! radius 5 at z = 0.
//!
//! The rounded hemisphere is a `sphere` `kept` above z = 0, its rim rolled by `fillet::roll` and
//! the round's piece cut away (`sketch::rolled`); the round, the face `fillet::roll` fits as a
//! B-spline, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::{kept, sphere};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = kept(&sphere([0., 0., 0.], UP, 5.)?, [0., 0., 0.], UP, 0.)?;
    let rounded = rolled(&solid, &[[5., 0., 0.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::BSpline(..)))
}
