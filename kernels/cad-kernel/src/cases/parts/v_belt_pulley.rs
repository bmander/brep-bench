//! A pulley on z: hub r 12 over z ±14, rim r 28 over z ±9, bore r 6, a V groove round the rim.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::revolve;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const HUB_RADIUS: f64 = 12.;
const HUB_HALF_LENGTH: f64 = 14.;
const RIM_RADIUS: f64 = 28.;
const RIM_HALF_WIDTH: f64 = 9.;
const BORE_RADIUS: f64 = 6.;
// the groove's triangle in the xz plane: its root on the rim's middle, its mouth past the rim
const GROOVE_ROOT: f64 = 22.;
const GROOVE_MOUTH: f64 = 29.;
const GROOVE_HALF_OPENING: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let (axis_at, axis) = ([0., 0., 0.], [0., 0., 1.]);
    // the blank's meridian in the xz plane, the bore's wall its inner side: hub, rim, hub
    let meridian = polygon(&[
        [BORE_RADIUS, 0., -HUB_HALF_LENGTH],
        [HUB_RADIUS, 0., -HUB_HALF_LENGTH],
        [HUB_RADIUS, 0., -RIM_HALF_WIDTH],
        [RIM_RADIUS, 0., -RIM_HALF_WIDTH],
        [RIM_RADIUS, 0., RIM_HALF_WIDTH],
        [HUB_RADIUS, 0., RIM_HALF_WIDTH],
        [HUB_RADIUS, 0., HUB_HALF_LENGTH],
        [BORE_RADIUS, 0., HUB_HALF_LENGTH],
    ]);
    let blank = revolve(&meridian, axis_at, axis, TAU)?;
    // the groove: the triangle turned once, its flanks two cones, reaching 1 past the rim's face
    let groove = revolve(
        &polygon(&[
            [GROOVE_ROOT, 0., 0.],
            [GROOVE_MOUTH, 0., -GROOVE_HALF_OPENING],
            [GROOVE_MOUTH, 0., GROOVE_HALF_OPENING],
        ]),
        axis_at,
        axis,
        TAU,
    )?;
    Ok(combined(&blank, &groove, Op::Cut, 0.)?)
}
