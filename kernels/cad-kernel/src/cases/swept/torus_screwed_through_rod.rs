//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove ground round it
//! by a torus of radii 6 and 2, centred at (11.5, 0, 0) with its axis (0, cos h, −sin h), h =
//! atan(2π·5.5 / 30), screwed about z — rising 30 a turn — from a turn of −108° to +348°.
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: [f64; 3] = [11.5, 0., 0.];
const MAJOR: f64 = 6.;
const MINOR: f64 = 2.;
const REACH: f64 = 5.5; // from the rod's axis to the tube's nearest centre
const LEAD: f64 = 30.;
const ROLL_DEGREES: [f64; 2] = [-108., 348.];
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let h = (TAU * REACH / LEAD).atan();
    let axis = [0., h.cos(), -h.sin()];
    // workaround: no torus primitive
    // the torus: its tube's circle in the plane through its axis and x (normal x × axis), turned
    let tube = disc([CENTRE[0] - MAJOR, CENTRE[1], CENTRE[2]], [0., h.sin(), h.cos()], MINOR);
    let wheel = Tool::Revolved { name: "wheel".into(), profile: tube, origin: CENTRE, axis };
    // cad-kernel's own solid sweep: a turn about z, right-handed, advancing LEAD a turn
    let screw = Motion::Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD };
    let roll = ROLL_DEGREES.map(f64::to_radians);
    swept_cut(&rod, &wheel, &screw, roll, Some(TOLERANCE)).map_err(|e| e.to_string())
}
