//! The cone frustum about the z axis of radius 6 at z = 0 narrowing to radius 3 at z = 20, a ring
//! groove turned into it, everything outside radius 4 from z = 8 to z = 12 cut away: a floor, the
//! cylinder of radius 4, and two walls, the flat rings at z = 8 and z = 12; the floor and both
//! walls then deleted together, the cone below the groove and the cone above it, one surface,
//! running on across it until they are one face again.
//!
//! The grooved frustum is `build::cone` cut by a ring, a `prism` of a `disc` of radius 6 (past the
//! cone throughout) less a `prism` of a `disc` of radius 4, from z = 8 to 12; the floor and both
//! walls are picked by a point on each and deleted together (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let frustum = cone([0., 0., 0.], UP, 6., 3., 20.)?;
    let outer = prism(&disc([0., 0., 8.], UP, 6.), 0., 4.)?;
    let inner = prism(&disc([0., 0., 7.], UP, 4.), 0., 6.)?;
    let ring = combined(&outer, &inner, Op::Cut, 0.)?;
    let grooved = combined(&frustum, &ring, Op::Cut, 0.)?;
    // the cone is 4.8 out at z = 8 and 4.2 at z = 12
    deleted_through(&grooved, &[[4., 0., 10.], [4.4, 0., 8.], [4.1, 0., 12.]])
}
