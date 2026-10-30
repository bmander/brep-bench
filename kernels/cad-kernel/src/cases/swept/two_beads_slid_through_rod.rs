//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved by a wheel of two overlapping
//! beads on one axis along y — a torus of radii 6 and 2 centred at (11.5, 0, −9) and one of radii
//! 6 and 1.5 centred at (11.5, 1.5, −9) — slid without turning 40 along z.
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: [f64; 3] = [11.5, 0., -9.];
const MAJOR: f64 = 6.;
const TUBES: [(f64, f64); 2] = [(0., 2.), (1.5, 1.5)]; // each bead: along the axis, tube radius
const SLIDE: f64 = 40.;
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // workaround: no torus primitive
    // each bead its tube's circle turned about the wheel's axis; the wheel is their union, which
    // the kernel sweeps member by member (a point is passed by the wheel just when by a bead)
    let beads = TUBES
        .iter()
        .enumerate()
        .map(|(k, &(along, r))| {
            let centre = [CENTRE[0], CENTRE[1] + along, CENTRE[2]];
            Tool::Revolved {
                name: format!("bead {}", k + 1),
                profile: disc([centre[0] - MAJOR, centre[1], centre[2]], up, r),
                origin: centre,
                axis: [0., 1., 0.],
            }
        })
        .collect();
    let slide = Motion::Slide { direction: up, advance: SLIDE };
    swept_cut(&rod, &Tool::Union(beads), &slide, [0., TAU], Some(TOLERANCE)).map_err(|e| e.to_string())
}
