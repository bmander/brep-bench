//! Two cone frustums on the z axis made one solid, from radius 3 at z = 0 to radius 5 at z = 4 and
//! from there to radius 2 at z = 8, the upper made turned 2 radians about the axis before the two
//! are joined, so their ridge is two edges and its round of radius 1 two pieces of a torus; both
//! pieces then deleted, the lower cone running on up and the upper one down until they meet again
//! in the circle of radius 5 at z = 4.
//!
//! The rounded frustums are two `build::cone`s, the upper turned by a `Rigid`, joined, the ridge
//! rolled by `fillet::roll` and the round's piece cut away (`sketch::rolled`); the round's two
//! pieces, the faces on a torus, are deleted together (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::geom::Rigid;
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let upper = cone([0., 0., 4.], UP, 5., 2., 4.)?.moved(&Rigid::turn([0.; 3], UP, 2.));
    let solid = combined(&cone([0., 0., 0.], UP, 3., 5., 4.)?, &upper, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[5., 0., 4.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
