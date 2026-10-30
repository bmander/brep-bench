//! A disc of radius 20 on z from z = −5 to 0, six holes of radius 2 on a circle of 12, cut in turn.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::Rigid;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const RADIUS: f64 = 20.;
const THICKNESS: f64 = 5.;
const HOLE_RADIUS: f64 = 2.;
const HOLE_CIRCLE: f64 = 12.;
const HOLES: usize = 6;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let mut plate = prism(&disc([0., 0., -THICKNESS], up, RADIUS), 0., THICKNESS)?;
    // the first drill at (12, 0), running past both faces, from z = −6 to z = 1
    let start = [HOLE_CIRCLE, 0., -THICKNESS - 1.];
    let first = prism(&disc(start, up, HOLE_RADIUS), 0., THICKNESS + 2.)?;
    for k in 0..HOLES {
        // each the first turned about z by k sixths of a turn, each its own cut
        let turn = Rigid::turn([0., 0., 0.], up, TAU * k as f64 / HOLES as f64);
        plate = combined(&plate, &first.moved(&turn), Op::Cut, 0.)?;
    }
    Ok(plate)
}
