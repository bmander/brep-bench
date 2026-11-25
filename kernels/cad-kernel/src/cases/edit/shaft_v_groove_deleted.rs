//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it, everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at
//! radius 3 at z = 10, running out to the shaft at z = 8 and z = 12; both cones then deleted
//! together, the shaft's cylinder below and above the groove, one surface, running on across it
//! until it is one face again.
//!
//! The grooved shaft is a `prism` of a `disc` cut by the ring the groove takes, its meridian
//! triangle, from radius 3 at z = 10 out past the shaft to radius 6 at z = 7 and 13, turned by
//! `build::revolve`; the groove's two cones are picked by a point on each and deleted together
//! (`defeature::deleted`).
use crate::sketch::{deleted_through, polygon};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let meridian = polygon(&[[3., 0., 10.], [6., 0., 7.], [6., 0., 13.]]);
    let ring = revolve(&meridian, [0.; 3], [0., 0., 1.], TAU)?;
    let grooved = combined(&shaft, &ring, Op::Cut, 0.)?;
    deleted_through(&grooved, &[[4., 0., 9.], [4., 0., 11.]])
}
