//! Two cone frustums on the z axis made one solid, from radius 3 at z = 0 to radius 5 at z = 4 and
//! from there to radius 2 at z = 8, their ridge, the circle of radius 5 at z = 4, rounded to radius
//! 1 by a piece of a torus tangent to both cones; the round then deleted, the lower cone running on
//! up and the upper one down until they meet again in the ridge.
//!
//! The rounded frustums are two `build::cone`s joined, the ridge rolled by `fillet::roll` and the
//! round's piece cut away (`sketch::rolled`); the round, the face on a torus, is deleted
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let outer = cone([0., 0., 0.], UP, 3., 5., 4.)?;
    let inner = cone([0., 0., 4.], UP, 5., 2., 4.)?;
    let solid = combined(&outer, &inner, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[5., 0., 4.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
