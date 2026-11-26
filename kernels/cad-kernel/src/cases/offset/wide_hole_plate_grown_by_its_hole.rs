//! The plate from (−30, −30, 0) to (30, 30, 4) drilled through along z with a hole of radius 20,
//! grown by 20, the hole's radius: the hole fills, and a horn torus's dimple, cusped on the axis,
//! stays over each of its ends.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const GROWTH: f64 = 20.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-30., -30., 0.], [30., 30., 4.])?;
    // the drill runs 1 past both faces of the plate
    let drill = prism(&disc([0., 0., -1.], [0., 0., 1.], 20.), 0., 6.)?;
    grown(&combined(&plate, &drill, Op::Cut, 0.)?, GROWTH, Join::Round)
}
