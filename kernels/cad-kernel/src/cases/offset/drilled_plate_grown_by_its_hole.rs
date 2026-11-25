//! The plate from (−10, −10, 0) to (10, 10, 4) drilled through along z with a hole of radius 3,
//! grown by 3, the hole's radius: the hole fills, and a dimple stays over each of its ends.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const GROWTH: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-10., -10., 0.], [10., 10., 4.])?;
    // the drill runs 1 past both faces of the plate
    let drill = prism(&disc([0., 0., -1.], [0., 0., 1.], 3.), 0., 6.)?;
    let drilled = combined(&plate, &drill, Op::Cut, 0.)?;
    grown(&drilled, GROWTH, Join::Round)
}
