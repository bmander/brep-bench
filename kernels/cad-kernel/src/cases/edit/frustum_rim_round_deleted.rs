//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its top rim rounded
//! to radius 1 by a piece of the torus whose tube has radius 1 about the circle of radius
//! (7 − √5)/2 at z = 5; the round then deleted, the top running on outward and the cone up until
//! they meet in the circle of radius 3 at z = 6.
//!
//! The rounded frustum is a `build::cone`, its top rim rolled by `fillet::roll` and the round's
//! piece cut away (`sketch::rolled`); the round, the face on a torus, is deleted
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::cone;
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = cone([0., 0., 0.], UP, 6., 3., 6.)?;
    let rounded = rolled(&solid, &[[3., 0., 6.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
