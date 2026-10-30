//! A plate (−20, −20, 0)–(20, 20, 4) joined to a boss of radius 5 on z from z = 4 to 8, the boss's
//! rim and root both rounded to radius 2, which takes its whole side.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const HALF_SIDE: f64 = 20.;
const THICKNESS: f64 = 4.;
const BOSS_RADIUS: f64 = 5.;
const BOSS_HEIGHT: f64 = 4.;
const ROUND: f64 = 2.;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece taken off a convex
/// edge, filled in at a concave one.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    let plate = block([-HALF_SIDE, -HALF_SIDE, 0.], [HALF_SIDE, HALF_SIDE, THICKNESS])?;
    // the boss: its disc on the plate's top swept up, its side named `boss` (the plate's top and
    // the boss's are both `near`)
    let mut section = disc([0., 0., THICKNESS], [0., 0., 1.], BOSS_RADIUS);
    section.names = vec![vec!["boss".into()]];
    let boss = prism(&section, 0., BOSS_HEIGHT)?;
    let part = combined(&plate, &boss, Op::Union, 0.)?;
    // the rim of the boss's top (convex), then its root on the plate (concave), each picked by a
    // point on it; the second round's band reaches the first's edge on the boss's side
    let part = rounded(&part, [0., BOSS_RADIUS, THICKNESS + BOSS_HEIGHT])?;
    rounded(&part, [0., BOSS_RADIUS, THICKNESS])
}
