//! A turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, the cone made turned 1 radian about the
//! axis before the two are joined, so the shoulder is two edges and its round of radius 1 two
//! pieces of a torus; both pieces then deleted, the cylinder running on up and the cone down until
//! they meet again in the circle of radius 5 at z = 6.
//!
//! The rounded post is the cylinder, a `prism` of a `disc`, and the frustum, a `build::cone` turned
//! by a `Rigid`, joined, the shoulder rolled by `fillet::roll` and the round's piece cut away
//! (`sketch::rolled`); the round's two pieces, the faces on a torus, are deleted together
//! (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let top = cone([0., 0., 6.], UP, 5., 2., 4.)?.moved(&Rigid::turn([0.; 3], UP, 1.));
    let solid = combined(&prism(&disc([0., 0., 0.], UP, 5.), 0., 6.)?, &top, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[5., 0., 6.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
