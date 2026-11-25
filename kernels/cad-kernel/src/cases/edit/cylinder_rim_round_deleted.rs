//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2 by a
//! quarter of the torus whose tube has radius 2 about the circle of radius 4 at z = 8; the round
//! then deleted, the top running on from radius 4 and the side from z = 8 until they meet in the
//! circle of radius 6 at z = 10.
//!
//! The rounded cylinder is a `prism` of a `disc`, its top rim rolled by `fillet::roll` and the
//! round's piece cut away (`sketch::rolled`); the round, the face on a torus, is deleted
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = prism(&disc([0., 0., 0.], UP, 6.), 0., 10.)?;
    let rounded = rolled(&solid, &[[6., 0., 10.]], 2.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
