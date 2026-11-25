//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, both rims rounded to radius 2 by
//! quarters of the tori whose tubes have radius 2 about the circles of radius 4 at z = 8 and at
//! z = 2; both rounds then deleted together, the top and the bottom running on outward from radius
//! 4 and the side up from z = 8 and down from z = 2 until they meet in the circles of radius 6 at
//! z = 10 and at z = 0.
//!
//! The rounded cylinder is a `prism` of a `disc`, each rim rolled by `fillet::roll` and its piece
//! cut away (`sketch::rolled`); the two rounds, the faces on tori, are deleted together
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = prism(&disc([0., 0., 0.], UP, 6.), 0., 10.)?;
    let rounded = rolled(&solid, &[[6., 0., 10.], [6., 0., 0.]], 2.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
