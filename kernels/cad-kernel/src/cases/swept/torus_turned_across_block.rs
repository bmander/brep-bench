//! A ring sector about z (radii 8 to 12, z from −1 to 1, from 0° to 60°) grooved by a torus whose
//! axis passes through (5, 0, 0) along (0, sin 25°, cos 25°), radii 2.5 and 2, turned rigidly about
//! z from −140° to +140°.
use crate::sketch::polygon;
use cad_kernel::build::{disc, revolve};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;

const INNER: f64 = 8.;
const OUTER: f64 = 12.;
const HALF_HEIGHT: f64 = 1.;
const SECTOR_DEGREES: f64 = 60.;
const CENTRE: [f64; 3] = [5., 0., 0.];
const LEAN_DEGREES: f64 = 25.;
const MAJOR: f64 = 2.5;
const MINOR: f64 = 2.;
const ROLL_DEGREES: [f64; 2] = [-140., 140.];
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let (h, r) = (HALF_HEIGHT, [INNER, OUTER]);
    let section = polygon(&[[r[0], 0., -h], [r[1], 0., -h], [r[1], 0., h], [r[0], 0., h]]);
    let block = revolve(&section, [0., 0., 0.], up, SECTOR_DEGREES.to_radians())?;
    let lean = LEAN_DEGREES.to_radians();
    let axis = [0., lean.sin(), lean.cos()];
    // the torus: its tube's circle in the plane through its axis and x (normal x × axis), turned
    let tube = disc([CENTRE[0] + MAJOR, CENTRE[1], CENTRE[2]], [0., -lean.cos(), lean.sin()], MINOR);
    let cutter = Tool::Revolved { name: "cutter".into(), profile: tube, origin: CENTRE, axis };
    // cad-kernel's own solid sweep: a turn about z, right-handed, a radian a radian of roll
    let turn = Motion::Turn { origin: [0., 0., 0.], axis: up, ratio: 1. };
    let roll = ROLL_DEGREES.map(f64::to_radians);
    swept_cut(&block, &cutter, &turn, roll, Some(TOLERANCE)).map_err(|e| e.to_string())
}
