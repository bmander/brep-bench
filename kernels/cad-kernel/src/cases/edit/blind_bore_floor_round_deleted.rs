//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a flat floor at z = 4, the concave edge where the floor meets the wall rounded
//! to radius 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 3 at
//! z = 5; the round then deleted, the floor running on outward from radius 3 and the wall down from
//! z = 5 until they meet in the circle of radius 4 at z = 4.
//!
//! The rounded bore is the block, a `prism`, the bore, a `prism` of a `disc`, cut away, and the
//! circle at its floor rolled by `fillet::roll`, the round's piece joined (`sketch::rolled`); the
//! round, the face on a torus, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let outer = block([-10., -10., 0.], [10., 10., 10.])?;
    let inner = prism(&disc([0., 0., 4.], UP, 4.), 0., 7.)?;
    let solid = combined(&outer, &inner, Op::Cut, 0.)?;
    let rounded = rolled(&solid, &[[4., 0., 4.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
