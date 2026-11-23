//! A bar with an eye across its middle: the box from (−40, −8, −5) to (40, 8, 5) joined to the
//! cylinder of radius 14 about the y axis from y = −8 to 8, flush with the bar's sides. The four
//! straight concave edges where the eye's side meets the bar's top and bottom, at x = ±√171,
//! z = ±5, rounded to radius 3, filling; each round runs the bar's whole width and ends flush with
//! its sides.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;

const LENGTH: f64 = 40.;
const WIDTH: f64 = 8.;
const THICKNESS: f64 = 5.;
const EYE: f64 = 14.;
const ROUND: f64 = 3.;

/// `part` with the edge through `at` rounded to `ROUND`: the kernel's piece filled in at a concave
/// edge, taken off a convex one. Every face is named apart first: a roll reads faces of one name
/// as pieces of one face, and the faces the Booleans leave share theirs.
fn rounded(part: &Brep, at: V) -> Result<Brep, String> {
    let part = &named(part.clone());
    let round = roll(part, at, ROUND, tolerance(part.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(part, &round.piece, op, 0.)?)
}

pub fn build() -> Result<Brep, String> {
    let bar = block([-LENGTH, -WIDTH, -THICKNESS], [LENGTH, WIDTH, THICKNESS])?;
    let eye = prism(&disc([0., -WIDTH, 0.], [0., 1., 0.], EYE), 0., 2. * WIDTH)?;
    let part = combined(&bar, &eye, Op::Union, 0.)?;
    // the four roots, each picked at its middle (y = 0), where the eye's side crosses the bar's
    // top or bottom
    let (x, t) = ((EYE * EYE - THICKNESS * THICKNESS).sqrt(), THICKNESS);
    let roots: [V; 4] = [[x, 0., t], [-x, 0., t], [x, 0., -t], [-x, 0., -t]];
    roots.into_iter().try_fold(part, |b, at| rounded(&b, at))
}
