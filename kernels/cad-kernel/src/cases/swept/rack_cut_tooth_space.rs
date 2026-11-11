//! A gear blank, the disc of radius 22 about y from y = 0 to 6, with one tooth space cut by a rack
//! tooth — the prism on the trapezoid (18, ±0.842856), (24, ±3.026666) in (x, z), from y = −2 to
//! 8 — whose pitch line x = 20 rolls without slipping on the pitch circle of radius 20, from 60°
//! on one side of (20, 0, 0) to 60° on the other.
//!
//! The kernel's one sweep operation builds the tooth's swept solid alone, over the case's whole
//! roll, and the ordinary Boolean cuts it from the blank. Seen from the blank the rack rolls in the
//! plane y = 0 and the tooth is a prism square to it, so `swept::closed::closed` builds the region
//! the trapezoid sweeps in that plane — its flanks' envelopes (involutes of the base circle), its tip
//! corners' trochoids and the trapezoid at each end pose, each stretch kept where it stands outside
//! the trapezoid at every pose — fitted and carried through the tooth's length (#880 Q2), and judges
//! it by the sweep's definition before returning it.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::{PI, TAU};

/// How closely the tooth is meshed for the sweep's membership reads (its faces are flat).
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let blank = prism(&disc([0., 0., 0.], [0., 1., 0.], 22.), 0., 6.)?;
    // the trapezoid in the plane y = −2, walked so that it faces +y, carried 10 along y
    let (tip, root) = (0.842856, 3.026666);
    let section = polygon(&[[18., -2., -tip], [18., -2., tip], [24., -2., root], [24., -2., -root]]);
    let tooth = prism(&section, 0., 10.)?;
    // the blank turns about y while the rack slides along the pitch line, 2π·20 a turn the way the
    // blank's rim moves at the pitch point: the tooth seen from the blank
    let rack = Trajectory::Twist(Screw { origin: [0.; 3], axis: [0., 0., 1.], ratio: 0., advance: -TAU * 20. });
    let blank_turn = Trajectory::Twist(Screw { origin: [0.; 3], axis: [0., 1., 0.], ratio: 1., advance: 0. });
    let path = Trajectory::Relative { source: Box::new(rack), observer: Box::new(blank_turn) };
    let swept = closed(&tooth, &path, [-PI / 3., PI / 3.], BAR, tolerance(tooth.size()))
        .map_err(|why| format!("the tooth's sweep not built: {why:?}"))?;
    Ok(combined(&blank, &swept, Op::Cut, 0.)?)
}
