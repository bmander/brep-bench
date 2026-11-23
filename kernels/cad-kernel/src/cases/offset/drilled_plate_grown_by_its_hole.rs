//! The plate from (−10, −10, 0) to (10, 10, 4) drilled through along z with a hole of radius 3,
//! grown by 3, the hole's radius: the hole fills, and a dimple stays over each of its ends.
//!
//! Grown by 3 in one step the kernel refuses the union of its pieces: "the join of `far` (plane)
//! and face 6 (cylinder) united: a shell of the arrangement is kept on one side of a plane
//! (`near`) and dropped on another: two of its cells joined round an edge" (the hole's wall swept
//! onto its own axis, the tube over its rim a horn torus meeting itself there). A solid grown by a
//! and then by b is the solid grown by a + b (every point within a + b of it), so it is grown by
//! half the hole's radius twice, which the kernel builds (as it does any split of the 3).
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
    grown(&grown(&drilled, GROWTH / 2., Join::Round)?, GROWTH / 2., Join::Round)
}
