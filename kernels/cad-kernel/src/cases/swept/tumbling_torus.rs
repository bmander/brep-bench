//! The solid a torus (centre (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0), seen from a frame turning by t about the line through
//! (0, 0.5, 0.5) along x, for t from −75° to 75°.
use cad_kernel::build::disc;
use cad_kernel::sweep::cut::{swept_solid, Motion, Tool};
use cad_kernel::topo::Brep;

const CENTRE: [f64; 3] = [3., 0., 2.];
const MAJOR: f64 = 1.;
const MINOR: f64 = 0.5;
const SPIN_THROUGH: [f64; 3] = [2., 0., 0.];
const SPIN_RATIO: f64 = 0.25;
const OBSERVER_THROUGH: [f64; 3] = [0., 0.5, 0.5];
const ROLL_DEGREES: f64 = 75.;
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let tube = disc([CENTRE[0] + MAJOR, CENTRE[1], CENTRE[2]], [0., -1., 0.], MINOR);
    let torus = Tool::Revolved { name: "torus".into(), profile: tube, origin: CENTRE, axis: [0., 0., 1.] };
    let tumble = Motion::Turn { origin: SPIN_THROUGH, axis: [0., 0., 1.], ratio: SPIN_RATIO }
        .seen_from(Motion::Turn { origin: OBSERVER_THROUGH, axis: [1., 0., 0.], ratio: 1. });
    let roll = [-ROLL_DEGREES.to_radians(), ROLL_DEGREES.to_radians()];
    swept_solid(&torus, &tumble, roll, Some(TOLERANCE)).map_err(|e| e.to_string())
}
