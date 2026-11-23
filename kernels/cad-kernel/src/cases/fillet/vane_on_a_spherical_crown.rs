//! A disc with a crowned top and a thin vane standing on it, the loop where the vane meets the
//! crown rounded to radius 2. The disc is turned about the z axis: flat beneath at z = 0, radius
//! 100, its rim the cylinder up to z = 3 and its top the sphere of radius 800 through the rim's top
//! circle, centred on the axis below at (0, 0, 3 − 100√63). The vane is the obround prism of every
//! point within 2 of the segment from (22, 0) to (88, 0), from z = 1 (inside the disc) to z = 31,
//! united with the disc. The foot — two straight runs along the vane's flat sides and two half
//! circles round its ends, one tangent chain — is rounded all the way round by the ball of radius
//! 2 rolling between the vane and the crown; every other edge stays sharp.
//!
//! Rolled along the foot as it is, the ball is refused at each seam where a flat side runs on into
//! a round end ("its contact crosses their seam in another section than its edge does … no one
//! section joins the run to the piece going on"): the crown leans the sections, as the case says.
//! So the round is built as cad-kernel's lowering composes a chain beside such a seam, each run's
//! piece kept on its side of the seam's normal plane — the plane through the end's axis square to
//! the vane, where the nearest point of the vane passes from the flat side to the half circle —
//! and the pieces glued there. Each run is rolled (`fillet::roll`) against the disc and the
//! surface it lies in alone: a flat side's against the vane's block between the end planes x = 22
//! and x = 88, the ball running round a circle (a torus's stretch) cut off at those planes; an
//! end's all the way round the whole post of that end's cylinder, its centre traced, kept beyond
//! the plane. The four pieces are filled into the disc and the vane.
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc as disc_profile, kept, prism, revolve, Profile, ProfileEdge};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, Frame};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI, TAU};

const DISC_RADIUS: f64 = 100.;
const RIM_HEIGHT: f64 = 3.;
const CROWN_RADIUS: f64 = 800.;
/// The vane's plan: within `HALF_WIDTH` of the segment from `(START, 0)` to `(END, 0)`.
const START: f64 = 22.;
const END: f64 = 88.;
const HALF_WIDTH: f64 = 2.;
const FOOT: f64 = 1.;
const TOP: f64 = 31.;
const ROUND: f64 = 2.;

/// The disc: its half section in the plane y = 0 — the floor, the rim, the crown's arc about its
/// centre on the axis, the axis — turned once about the z axis.
fn disc() -> Result<Brep, String> {
    let at = |x: f64, z: f64| [x, 0., z];
    let below = RIM_HEIGHT - (CROWN_RADIUS * CROWN_RADIUS - DISC_RADIUS * DISC_RADIUS).sqrt();
    let crown = Frame::new(at(0., below), [0., -1., 0.], [1., 0., 0.]);
    let from = (RIM_HEIGHT - below).atan2(DISC_RADIUS);
    let peak = below + CROWN_RADIUS;
    let edges = vec![
        ProfileEdge::Line { a: at(0., 0.), b: at(DISC_RADIUS, 0.) },
        ProfileEdge::Line { a: at(DISC_RADIUS, 0.), b: at(DISC_RADIUS, RIM_HEIGHT) },
        ProfileEdge::Arc { frame: crown, r: CROWN_RADIUS, span: Some([from, FRAC_PI_2]) },
        ProfileEdge::Line { a: at(0., peak), b: at(0., 0.) },
    ];
    let section = Profile { origin: at(0., 0.), normal: [0., -1., 0.],
                            loops: vec![edges], names: Vec::new() };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}

/// The vane: its obround plan in the plane z = `FOOT`, swept up to `TOP`.
fn vane() -> Result<Brep, String> {
    let at = |x: f64, y: f64| [x, y, FOOT];
    let end = |x: f64, from: f64| ProfileEdge::Arc {
        frame: Frame::new(at(x, 0.), [0., 0., 1.], [1., 0., 0.]),
        r: HALF_WIDTH,
        span: Some([from, from + PI]),
    };
    let edges = vec![
        ProfileEdge::Line { a: at(START, -HALF_WIDTH), b: at(END, -HALF_WIDTH) },
        end(END, -FRAC_PI_2),
        ProfileEdge::Line { a: at(END, HALF_WIDTH), b: at(START, HALF_WIDTH) },
        end(START, FRAC_PI_2),
    ];
    let plan = Profile { origin: at(0., 0.), normal: [0., 0., 1.],
                         loops: vec![edges], names: Vec::new() };
    prism(&plan, 0., TOP - FOOT)
}

pub fn build() -> Result<Brep, String> {
    let disc = disc()?;
    let part = combined(&disc, &vane()?, Op::Union, 0.)?;
    // the crown's height over a point of the plan
    let below = RIM_HEIGHT - (CROWN_RADIUS * CROWN_RADIUS - DISC_RADIUS * DISC_RADIUS).sqrt();
    let crown = |x: f64, y: f64| below + (CROWN_RADIUS * CROWN_RADIUS - x * x - y * y).sqrt();
    // a run rolled against the disc with `standing` on it, picked by the point of its foot over
    // (x, y) (every face named apart: a roll reads faces of one name as pieces of one face)
    let rolled = |standing: Brep, x: f64, y: f64| -> Result<Brep, String> {
        let aux = named(combined(&disc, &standing, Op::Union, 0.)?);
        Ok(roll(&aux, [x, y, crown(x, y)], ROUND, tolerance(aux.size()))?.piece)
    };
    let mut pieces = Vec::new();
    // the flat sides: against the block between the ends' planes, picked at their middles
    let middle = (START + END) / 2.;
    for y in [-HALF_WIDTH, HALF_WIDTH] {
        let block = block([START, -HALF_WIDTH, FOOT], [END, HALF_WIDTH, TOP])?;
        pieces.push(rolled(block, middle, y)?);
    }
    // the ends: against the whole post, picked at their tips, kept beyond the plane of the axis
    for (x, out) in [(END, 1.), (START, -1.)] {
        let post = prism(&disc_profile([x, 0., FOOT], [0., 0., 1.], HALF_WIDTH), 0., TOP - FOOT)?;
        let ring = rolled(post, x + out * HALF_WIDTH, 0.)?;
        pieces.push(kept(&ring, [x, 0., 0.], [out, 0., 0.], 0.)?);
    }
    // the foot's round is concave: each piece filled in
    let mut solid = part;
    for piece in pieces { solid = combined(&solid, &piece, Op::Union, 0.)?; }
    Ok(solid)
}
