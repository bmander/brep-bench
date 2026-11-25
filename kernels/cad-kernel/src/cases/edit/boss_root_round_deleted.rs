//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge at the boss's foot rounded to radius
//! 1 by a quarter of the torus whose tube has radius 1 about the circle of radius 4 at z = 1; the
//! round then deleted, the plate's top running in from radius 4 and the boss's side down from z = 1
//! until they meet in the circle of radius 3 at z = 0.
//!
//! The rounded boss is the cylinders, `prism`s of `disc`s, joined, the circle at the boss's foot
//! rolled by `fillet::roll` and the round's piece joined (`sketch::rolled`); the round, the face on
//! a torus, is deleted (`defeature::deleted`).
use crate::sketch::{deleted_where, rolled};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{Surface, V};
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let plate = prism(&disc([0., 0., 0.], UP, 8.), -4., 0.)?;
    let solid = combined(&plate, &prism(&disc([0., 0., 0.], UP, 3.), 0., 8.)?, Op::Union, 0.)?;
    let rounded = rolled(&solid, &[[3., 0., 0.]], 1.)?;
    deleted_where(&rounded, |s| matches!(s, Surface::Torus(..)))
}
