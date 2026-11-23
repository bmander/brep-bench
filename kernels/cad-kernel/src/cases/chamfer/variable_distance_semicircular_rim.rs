//! The half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 on the side x ≥ 0, the
//! half circle where its curved side meets its top chamfered by the same distance d on both faces,
//! d growing linearly with the angle φ round the z axis: d = 3/2 + φ/π, 1 at φ = −90°, 2 at 90°.
//! In the half plane at φ the chamfer's section is the line from the point d in across the top to
//! the point d down the side; the chamfer meets the top along a spiral of Archimedes and the side
//! along a helix, and is the ruled surface between them.
//!
//! cad-kernel's chamfer, `fillet::flat_along(&b, at, legs, ..)`, takes two legs constant along the
//! whole edge; nothing takes a distance that changes along it. But the chamfer's surface is the set
//! ρ + z = 11 − d(φ), and turning by δ about the axis while stepping −δ/π along it carries that set
//! onto itself: it is a line of a meridian screwed. So what the chamfer takes away is the triangle
//! that line cuts off the rim's corner in one meridian, run on past the cylinder's side and top,
//! screwed about the z axis by `build::screwed` (exact, its sheets the screw's ruled surfaces) and
//! cut from the half cylinder.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism, screwed};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, PI};

const RADIUS: f64 = 5.;
const HEIGHT: f64 = 6.;
/// The distance at φ = 0 and its rate round the rim: d = 3/2 + φ/π.
const D_MID: f64 = 1.5;
const D_RATE: f64 = 1. / PI;
/// How far the screwed triangle reaches past the side and over the top, and the angle it runs on
/// past each end of the rim, into x < 0, outside the half cylinder.
const REACH: f64 = 2.;
const PAST: f64 = PI / 10.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0.; 3], [0., 0., 1.], RADIUS), 0., HEIGHT)?;
    let half = kept(&cylinder, [0.; 3], [1., 0., 0.], 0.)?;
    // the screw's start and its turn, from past the rim's start at φ = −90° to past its end at 90°
    let (from, turn) = (-FRAC_PI_2 - PAST, PI + 2. * PAST);
    let drop = D_RATE * turn;
    // in the meridian at `from`, the chamfer's line ρ + z = 11 − d, and the triangle it cuts off
    // the corner of the side ρ = 7 and the level z = 6 + 2 + drop, so that screwed down by `drop`
    // it still stands 2 over the top
    let c = RADIUS + HEIGHT - (D_MID + D_RATE * from);
    let (out, top) = (RADIUS + REACH, HEIGHT + REACH + drop);
    let at = |rho: f64, z: f64| [rho * from.cos(), rho * from.sin(), z];
    let section = polygon(&[at(c - top, top), at(out, c - out), at(out, top)]);
    // turned through `turn` about the z axis while stepping down by d's growth over it
    let tool = screwed(&section, [0.; 3], [0., 0., 1.], turn, -drop)?;
    Ok(combined(&half, &tool, Op::Cut, 0.)?)
}
