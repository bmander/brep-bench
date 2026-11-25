//! The plate of radius 8 on the z axis from z = −2 to z = 0 joined to a boss, the cone frustum from
//! radius 4 at z = 0 to radius 1 at z = 6, the concave edge where the boss meets the plate's top
//! filleted to radius 1, grown by 2 with rounded edges: every point within 2 of it.
//!
//! The fillet's torus, swept past its tube's radius, would turn inside out: `offset::grown` stops
//! its sweep at the centre of its tube (a point past it is nearer the far side's faces), and the
//! moved top and cone, running on past it, meet in a sharp concave edge.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const PLATE_RADIUS: f64 = 8.;
const PLATE_DEPTH: f64 = 2.;
const BOSS_FOOT: f64 = 4.;
const BOSS_TOP: f64 = 1.;
const BOSS_HEIGHT: f64 = 6.;
const FILLET: f64 = 1.;
const GROWTH: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let plate = prism(&disc([0., 0., -PLATE_DEPTH], up, PLATE_RADIUS), 0., PLATE_DEPTH)?;
    let boss = cone([0., 0., 0.], up, BOSS_FOOT, BOSS_TOP, BOSS_HEIGHT)?;
    let part = combined(&plate, &boss, Op::Union, 0.)?;
    // the edge is picked by a point on it, the boss's foot on the plate's top; the kernel rolls
    // the ball between the top and the cone round the loop
    let round = roll(&part, [0., BOSS_FOOT, 0.], FILLET, tolerance(part.size()))?;
    // a concave edge's round fills its piece in
    let op = if round.concave { Op::Union } else { Op::Cut };
    let filleted = combined(&part, &round.piece, op, 0.)?;
    grown(&filleted, GROWTH, Join::Round)
}
