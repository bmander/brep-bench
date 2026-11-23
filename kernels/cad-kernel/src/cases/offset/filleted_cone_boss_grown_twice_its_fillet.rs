//! The plate of radius 8 on the z axis from z = −2 to z = 0 joined to a boss, the cone frustum from
//! radius 4 at z = 0 to radius 1 at z = 6, the concave edge where the boss meets the plate's top
//! filleted to radius 1.5, grown by 3 with rounded edges: every point within 3 of it.
//!
//! Refused, by the kernel: "`round` (torus) is not swept along its normal: swept past the centre
//! of its torus: no point stands that far off it". `offset::grown` sweeps the fillet's torus
//! outward only as far as its tube's radius and refuses it past that rather than drop it. Grown in
//! two steps (`offset/filleted-post-grown-past-its-fillet`'s way), first by the fillet's radius,
//! which the kernel builds with the fillet gone, the boss's top rim has become a torus as wide as
//! the circle it runs round, and the second step refuses it: "face 7 (torus) is not swept along
//! its normal: its sweep passes the axis of its torus" (`offset::revolved_sweep` cuts a sweep at
//! the axis only for a straight meridian or a whole tube, not a part of a torus); so does every
//! other split tried. (Growing the boss without its fillet builds, but that is not the solid
//! described grown: it drops the face the case is about.)
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
const FILLET: f64 = 1.5;
const GROWTH: f64 = 3.;

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
