//! A plate 12 thick, from z = 0 to z = 12, whose outline is six round lobes and six hollows
//! between them, each running on tangent into the next, with both its rims rounded to radius 1.
//! The lobes are arcs of the circles of radius 8 about the points 40 from the z axis at 0°, 60°,
//! …, 300°; each hollow an arc of the circle of radius 30 touching the two lobes beside it from
//! outside, its centre on the ray halfway between theirs, 38 from each (the farther such point).
//! Each rim, the twelve edges where the top (or the bottom) meets the side, is one tangent chain
//! rounded as one run that never ends.
//!
//! As cad-kernel composes a chain's rounds (`rounds::rounded` rounds only straight edges between
//! planes in one call): each arc's run rolled against the plate (`fillet::roll`), cut off at the
//! section through the seam where its side runs on tangent into the next, and each piece taken
//! off the plate in turn; the pieces of two runs meet at the section they share.
use crate::sketch::named;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const LOBES: usize = 6;
const LOBE_RADIUS: f64 = 8.;
const LOBE_REACH: f64 = 40.;
const HOLLOW_RADIUS: f64 = 30.;
const THICKNESS: f64 = 12.;
const ROUND: f64 = 1.;

/// The turn between two lobes.
const STEP: f64 = TAU / LOBES as f64;

fn ray(a: f64, d: f64) -> V { [d * a.cos(), d * a.sin(), 0.] }

/// How far each hollow's centre stands from the axis: on the ray halfway between two lobes'
/// centres, `LOBE_RADIUS + HOLLOW_RADIUS` from both, the farther of the two such points.
fn hollow_reach() -> f64 {
    let apart = LOBE_RADIUS + HOLLOW_RADIUS;
    let half_chord = LOBE_REACH * (STEP / 2.).sin();
    LOBE_REACH * (STEP / 2.).cos() + (apart * apart - half_chord * half_chord).sqrt()
}

/// The outline in the plane z = 0: each lobe's outer arc and each hollow's inner arc, running
/// between the points where they touch (on the line between their centres).
fn outline() -> Profile {
    let lobe = |k: usize| ray(STEP * k as f64, LOBE_REACH);
    let hollow = |k: usize| ray(STEP * (k as f64 + 0.5), hollow_reach());
    let angle = |from: V, to: V| (to[1] - from[1]).atan2(to[0] - from[0]);
    // an arc counter-clockwise about `c` from the angle `a0` to `a1`
    let arc = |c: V, r: f64, a0: f64, a1: f64| {
        let a1 = if a1 < a0 { a1 + TAU } else { a1 };
        let frame = Frame::new(c, [0., 0., 1.], [1., 0., 0.]);
        ProfileEdge::Arc { frame, r, span: Some([a0, a1]) }
    };
    let mut edges = Vec::new();
    for k in 0..LOBES {
        let (l, h) = (lobe(k), hollow(k));
        // the lobe's outer side, from the hollow before it to the hollow after
        edges.push(arc(l, LOBE_RADIUS, angle(l, hollow((k + LOBES - 1) % LOBES)), angle(l, h)));
        // the hollow's inner side, about its centre from the next lobe round to this one
        edges.push(arc(h, HOLLOW_RADIUS, angle(h, lobe((k + 1) % LOBES)), angle(h, l)));
    }
    Profile { origin: [0., 0., 0.], normal: [0., 0., 1.], loops: vec![edges], names: Vec::new() }
}

pub fn build() -> Result<Brep, String> {
    // (every face named apart: a roll reads faces of one name as pieces of one face)
    let plate = named(prism(&outline(), 0., THICKNESS)?);
    let tol = tolerance(plate.size());
    // every run of both rims rolled against the plate, each picked by its middle: a lobe's
    // outermost point, a hollow's innermost
    let mut rounds = Vec::new();
    for z in [THICKNESS, 0.] {
        for k in 0..LOBES {
            let lobe = ray(STEP * k as f64, LOBE_REACH + LOBE_RADIUS);
            let hollow = ray(STEP * (k as f64 + 0.5), hollow_reach() - HOLLOW_RADIUS);
            for [x, y, _] in [lobe, hollow] { rounds.push(roll(&plate, [x, y, z], ROUND, tol)?); }
        }
    }
    let mut solid = plate;
    for round in rounds {
        let op = if round.concave { Op::Union } else { Op::Cut };
        solid = combined(&solid, &round.piece, op, 0.)?;
    }
    Ok(solid)
}
