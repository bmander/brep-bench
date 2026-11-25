//! A turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, its shoulder, the circle of radius 5 at
//! z = 6, rounded to radius 1 by a piece of a torus tangent to the cylinder and to the cone; the
//! round then deleted, the cylinder running on up and the cone down until they meet again in the
//! shoulder.
//!
//! The rounded post is the cylinder, a `prism` of a `disc`, joined to a `build::cone`, the shoulder
//! rolled by `fillet::roll` and the round's piece cut away (`sketch::rolled`); the round, the face
//! on a torus, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let outer = prism(&disc([0., 0., 0.], UP, 5.), 0., 6.)?;
    let inner = cone([0., 0., 6.], UP, 5., 2., 4.)?;
    let solid = combined(&outer, &inner, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[5., 0., 6.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
