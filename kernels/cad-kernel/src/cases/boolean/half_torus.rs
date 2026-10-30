//! A torus about z of radii 10 and 2, with everything where x < 0 cut away.
use cad_kernel::build::{disc, kept, revolve};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const MAJOR: f64 = 10.;
const MINOR: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let torus = revolve(&disc([MAJOR, 0., 0.], [0., 1., 0.], MINOR), [0., 0., 0.], [0., 0., 1.], TAU)?;
    // what lies on the +x side of the plane x = 0: `kept` takes it in common with a block standing
    // on that plane and reaching past the torus everywhere else
    kept(&torus, [0., 0., 0.], [1., 0., 0.], 0.)
}
