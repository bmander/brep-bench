//! A block from the origin to (10, 4, 6), the two edges of its top running along x rounded to
//! radius 2, half the top's width: the top vanishes into a half-cylinder.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const LENGTH: f64 = 10.;
const WIDTH: f64 = 4.;
const HEIGHT: f64 = 6.;
const ROUND: f64 = 2.;

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
        [LENGTH, WIDTH, 0.],
        [0., WIDTH, 0.],
    ]);
    floor.names = vec![["front", "right", "back", "left"].map(String::from).to_vec()];
    let slab = prism(&floor, 0., HEIGHT)?;
    // the top's edge at y = 0, then the one at y = 4, each picked by its middle
    let slab = rounded(&slab, [LENGTH / 2., 0., HEIGHT])?;
    rounded(&slab, [LENGTH / 2., WIDTH, HEIGHT])
}
