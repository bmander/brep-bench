//! The drilled plate of `offset/drilled-plate-grown-by-its-hole`, ten times larger: the plate from
//! (−100, −100, 0) to (100, 100, 40) drilled through along z with a hole of radius 30, grown by 30,
//! the hole's radius: the hole fills, and a horn torus's dimple, cusped on the axis, stays over
//! each of its ends.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const GROWTH: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-100., -100., 0.], [100., 100., 40.])?;
    // the drill runs 10 past both faces of the plate
    let drill = prism(&disc([0., 0., -10.], [0., 0., 1.], 30.), 0., 60.)?;
    grown(&combined(&plate, &drill, Op::Cut, 0.)?, GROWTH, Join::Round)
}
