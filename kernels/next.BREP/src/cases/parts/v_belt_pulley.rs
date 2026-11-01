//! A pulley turned about the z axis: a hub, the cylinder of radius 12 from z = −14 to z = 14, and
//! a rim, the disc of radius 28 from z = −9 to z = 9, joined; a bore of radius 6 through on the
//! axis; and a V belt groove, the triangle (22, 0), (29, −5), (29, 5) of the xz plane turned a
//! full turn about z, taken away.
use crate::solids::fillet::{subtract, union};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let hub = make_cylinder_brep(p(0.0, 0.0, -14.0), up, 12.0, 28.0)?;
    let rim = make_cylinder_brep(p(0.0, 0.0, -9.0), up, 28.0, 18.0)?;
    // The bore's cylinder runs past both ends.
    let bore = make_cylinder_brep(p(0.0, 0.0, -15.0), up, 6.0, 30.0)?;
    let pulley = subtract(&union(&hub, &rim)?, &bore)?;
    let triangle = polygon(&[p(22.0, 0.0, 0.0), p(29.0, 0.0, -5.0), p(29.0, 0.0, 5.0)])?;
    let groove = revolve_profile_brep(&triangle, p(0.0, 0.0, 0.0), up, TAU)?;
    Ok(subtract(&pulley, &groove)?)
}
