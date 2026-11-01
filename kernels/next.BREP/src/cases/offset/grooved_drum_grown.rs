//! A drum, the cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove round its
//! side from z = 4.2 to 5.8 cut to radius 7, grown by 1 to its parallel body: the groove filled
//! but for a shallow notch where its outer rims become pieces of tori.
//!
//! The groove is cut by a ring, its section revolved a full turn, reaching past the drum's side.
use crate::solids::offset::{parallel, subtract};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, revolve_profile_brep, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 10.0, 10.0)?;
    let section =
        polygon(&[p(7.0, 0.0, 4.2), p(12.0, 0.0, 4.2), p(12.0, 0.0, 5.8), p(7.0, 0.0, 5.8)])?;
    let groove = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), z, TAU)?;
    let grooved = subtract(&drum, &groove)?;
    Ok(parallel(&grooved, 1.0)?)
}
