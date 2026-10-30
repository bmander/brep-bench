//! A flange on z: plate r 32 to z = 6, hub r 15 to z = 16, bore r 8, six bolt holes r 3 on r 24.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const PLATE_RADIUS: f64 = 32.;
const PLATE_THICKNESS: f64 = 6.;
const HUB_RADIUS: f64 = 15.;
const HUB_TOP: f64 = 16.;
const BORE_RADIUS: f64 = 8.;
const BOLT_CIRCLE: f64 = 24.;
const BOLT_RADIUS: f64 = 3.;
const BOLTS: usize = 6;

pub fn build() -> Result<Brep, String> {
    // a turned part: its meridian in the xz plane — the bore's wall, the plate's foot and rim, the
    // hub's shoulder and side, the hub's top — turned once about z
    let meridian = polygon(&[
        [BORE_RADIUS, 0., 0.],
        [PLATE_RADIUS, 0., 0.],
        [PLATE_RADIUS, 0., PLATE_THICKNESS],
        [HUB_RADIUS, 0., PLATE_THICKNESS],
        [HUB_RADIUS, 0., HUB_TOP],
        [BORE_RADIUS, 0., HUB_TOP],
    ]);
    let mut flange = revolve(&meridian, [0., 0., 0.], [0., 0., 1.], TAU)?;
    for k in 0..BOLTS {
        // one hole every 60° from +x, each drill running past both faces of the plate
        let angle = TAU * k as f64 / BOLTS as f64;
        let at = [BOLT_CIRCLE * angle.cos(), BOLT_CIRCLE * angle.sin(), -1.];
        let drill = prism(&disc(at, [0., 0., 1.], BOLT_RADIUS), 0., PLATE_THICKNESS + 2.)?;
        flange = combined(&flange, &drill, Op::Cut, 0.)?;
    }
    Ok(flange)
}
