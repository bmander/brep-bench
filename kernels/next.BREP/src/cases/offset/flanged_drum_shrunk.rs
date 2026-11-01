//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its side
//! from radius 8 to 11 and z = 2.2 to 3.8, shrunk by 1: every point at least 1 from its outside.
//! The flange is gone but for a low ridge where its inside edges become concave pieces of tori.
//!
//! The flange is a ring, its section revolved a full turn, joined to the drum.
use crate::solids::offset::{parallel, union};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 8.0, 6.0)?;
    let section =
        polygon(&[p(8.0, 0.0, 2.2), p(11.0, 0.0, 2.2), p(11.0, 0.0, 3.8), p(8.0, 0.0, 3.8)])?;
    let flange = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), z, TAU)?;
    let flanged = union(&drum, &flange)?;
    Ok(parallel(&flanged, -1.0)?)
}
