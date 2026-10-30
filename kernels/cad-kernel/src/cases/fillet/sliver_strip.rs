//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 running along x
//! rounded to radius 0.9999, leaving a strip of that face 0.0002 tall between the rounds.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const LENGTH: f64 = 20.;
const DEPTH: f64 = 10.;
const THICKNESS: f64 = 2.;
const ROUND: f64 = 0.9999;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece taken off a convex
/// edge, filled in at a concave one.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    // the floor, counter-clockwise from above so it faces up, swept up; its sides named, since a
    // round reads faces of one name as pieces of one face (the caps are `far` and `near`)
    let mut floor = polygon(&[
        [0., 0., 0.],
        [LENGTH, 0., 0.],
        [LENGTH, DEPTH, 0.],
        [0., DEPTH, 0.],
    ]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let plate = prism(&floor, 0., THICKNESS)?;
    // the face's edge at z = 0, then the one at z = 2, each picked by its middle
    let plate = rounded(&plate, [LENGTH / 2., DEPTH, 0.])?;
    rounded(&plate, [LENGTH / 2., DEPTH, THICKNESS])
}
