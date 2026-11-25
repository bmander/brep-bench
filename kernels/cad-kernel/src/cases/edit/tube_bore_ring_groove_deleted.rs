//! A tube about the z axis from z = 0 to z = 20, outer radius 10 and bore of radius 4, with a ring
//! groove turned into the bore, everything inside radius 6 from z = 8 to z = 12 cut away: a floor,
//! the cylinder of radius 6, and two walls, the flat rings from radius 4 to 6 at z = 8 and z = 12;
//! the floor and both walls then deleted together, the bore below and above the groove, one
//! surface, running on across it until it is one face again.
//!
//! The grooved tube is a `prism` of a `disc` of radius 10 cut by a `prism` of a `disc` of radius 4
//! right through and one of radius 6 from z = 8 to 12; the floor and both walls are picked by a
//! point on each and deleted together (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let outer = prism(&disc([0., 0., 0.], UP, 10.), 0., 20.)?;
    let inner = prism(&disc([0., 0., -1.], UP, 4.), 0., 22.)?;
    let tube = combined(&outer, &inner, Op::Cut, 0.)?;
    let grooved = combined(&tube, &prism(&disc([0., 0., 8.], UP, 6.), 0., 4.)?, Op::Cut, 0.)?;
    deleted_through(&grooved, &[[6., 0., 10.], [5., 0., 8.], [5., 0., 12.]])
}
