//! A plate (0, 0, 0)–(100, 20, 10) drilled by nine holes of radius 2 at y = 10, x = 10 … 90.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const THICKNESS: f64 = 10.;
const HOLE_RADIUS: f64 = 2.;
const HOLES: usize = 9;
const PITCH: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let mut plate = block([0., 0., 0.], [100., 20., THICKNESS])?;
    for k in 1..=HOLES {
        // each drill runs past both faces of the plate
        let at = [PITCH * k as f64, 10., -1.];
        let drill = prism(&disc(at, [0., 0., 1.], HOLE_RADIUS), 0., THICKNESS + 2.)?;
        plate = combined(&plate, &drill, Op::Cut, 0.)?;
    }
    Ok(plate)
}
