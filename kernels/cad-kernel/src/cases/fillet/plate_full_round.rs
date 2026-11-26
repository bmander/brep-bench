//! The plate from the origin to (40, 30, 6), all twelve of its edges rounded to radius 3, half its
//! thickness, in one operation: the four side faces vanish, the top's and the bottom's rounds
//! meeting tangentially along each side's middle, and each corner is a quarter of a ball.
//!
//! Rounded in one call (`rounds::rounded`, every edge) the kernel refuses: "a round of 3 along
//! [0, 0, 0]–[0, 0, 6] is larger than the" faces beside it allow, the round as wide as the side it
//! eats. The plate rounded so is the union of every ball of radius 3 inside it, the 34 × 24
//! rectangle at mid-height grown by 3, which is the ball about one corner of that rectangle slid
//! along its one side and then the other: so the ball is slid along x into a capsule, and the
//! capsule along y, each with the kernel's sweep of a solid.
use cad_kernel::build::sphere;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::slid;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{norm, unit};
use std::f64::consts::TAU;

const PLATE: V = [40., 30., 6.];
const ROUND: f64 = 3.;
/// How closely each tool is meshed for the sweep's membership reads: a thousandth of the round.
const BAR: f64 = 0.003;

/// `tool` slid without turning along `by`: a screw that does not turn, advancing its whole length
/// over a roll of τ.
fn slide(tool: &Brep, by: V) -> Result<Brep, String> {
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: unit(by), ratio: 0., advance: norm(by) });
    slid(tool, &path, [0., TAU], BAR, tolerance(tool.size() + norm(by)))
        .map_err(|why| format!("the slide along {by:?} not built: {why:?}"))
}

pub fn build() -> Result<Brep, String> {
    // the ball about the rectangle's corner (3, 3, 3), then its two sides' lengths
    let ball = sphere([ROUND; 3], [0., 0., 1.], ROUND)?;
    let capsule = slide(&ball, [PLATE[0] - 2. * ROUND, 0., 0.])?;
    slide(&capsule, [0., PLATE[1] - 2. * ROUND, 0.])
}
