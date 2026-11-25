//! The cone frustum on the z axis from radius 6 at z = 0 to radius 3 at z = 6, its side the cone
//! 2r + z = 12, both rims rounded to radius 3/2 by pieces of tori tangent to each end and to the
//! cone; the top round alone then deleted, the top running on outward and the cone up until they
//! meet in the circle of radius 3 at z = 6, the base's round staying.
//!
//! The rounded frustum is a `build::cone`, each rim rolled by `fillet::roll` and its piece cut away
//! (`sketch::rolled`); the top round, the face on the torus whose tube runs round above the
//! frustum's middle, is deleted alone (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::build::cone;
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let solid = cone([0., 0., 0.], UP, 6., 3., 6.)?;
    let rounded = rolled(&solid, &[[3., 0., 6.], [6., 0., 0.]], 1.5)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(f, ..) if f.o[2] > 3.))
}
