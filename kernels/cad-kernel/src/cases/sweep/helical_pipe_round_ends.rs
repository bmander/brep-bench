//! A pipe of radius 1 along 3 turns of the helix of radius 10 and pitch 5 about z; round ends.
use cad_kernel::build::canal;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const HELIX_RADIUS: f64 = 10.;
const PITCH: f64 = 5.;
const TURNS: f64 = 3.;
const RADIUS: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    // the ball about the helix's start carried by the screw about z — three turns, climbing a
    // pitch each — is the pipe with its rounded ends: cad-kernel's canal
    let start = [HELIX_RADIUS, 0., 0.];
    canal(start, RADIUS, [0., 0., 0.], [0., 0., 1.], TURNS * TAU, TURNS * PITCH, "pipe")
}
