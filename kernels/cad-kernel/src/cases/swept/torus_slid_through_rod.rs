//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved along its length by a torus
//! (axis along y, radii 6 and 2) slid without turning along z from centre (11.5, 0, −9) to
//! (11.5, 0, 29).
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: [f64; 3] = [11.5, 0., -9.];
const MAJOR: f64 = 6.;
const MINOR: f64 = 2.;
const SLIDE: f64 = 38.; // from z = −9 to z = 29
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // workaround: no torus primitive
    // the torus: its tube's circle, in the plane through its axis (y) and x, turned about the axis
    let tube = disc([CENTRE[0] - MAJOR, CENTRE[1], CENTRE[2]], up, MINOR);
    let wheel = Tool::Revolved { name: "wheel".into(), profile: tube, origin: CENTRE, axis: [0., 1., 0.] };
    // cad-kernel's own solid sweep: a slide moves `advance` over a whole turn of the roll
    let slide = Motion::Slide { direction: up, advance: SLIDE };
    swept_cut(&rod, &wheel, &slide, [0., TAU], Some(TOLERANCE)).map_err(|e| e.to_string())
}
