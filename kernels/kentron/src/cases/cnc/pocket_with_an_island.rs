//! The plate from (0, 0, −20) to (160, 110, 0), machined by a 2.5-axis program of three upright
//! tools, every move between cuts at z = 5:
//!
//! - a pocket, the rectangle from (20, 20) to (140, 90) with corners rounded to 12, round an island,
//!   the disc of radius 15 about (60, 55), cleared at z = −4, −8 and −12 by a flat end mill of
//!   radius 5, 30 long: raster passes along y = 26, 30, … 82 across the region its tip keeps to
//!   (inside the wall path, the rectangle from (25, 25) to (135, 85) with corners rounded to 7, and
//!   outside the island path, the circle of radius 20 about (60, 55)), then once round the wall
//!   path and once round the island path;
//! - a bore in the island's top by a flat end mill of radius 3: plunged at (60, 55) to z = −3, out
//!   to (66, 55), a whole circle of radius 6 about (60, 55), back, and again at z = −6;
//! - four holes pecked at (8, 8), (152, 8), (152, 102) and (8, 102) by a drill of radius 4, 30
//!   long, with a 90° point, its tip down to z = −14 at the deepest.
//!
//! The plate less everything the tools pass through. Each move that cuts is kentron's sweep of a
//! solid of its tool (`shared::cnc`): a slide for each raster pass, each line of a path, the bore's
//! move out and each drill's plunge, a turn about its centre for each arc (a whole circle in two
//! halves), all cut from the plate at once. The moves at z = 5 pass over the plate; a plunge onto
//! a pass's start, onto a path or back up from it passes through nothing the mill at that depth,
//! 30 tall, does not; the bore's move back runs along its move out; and the drill's shallower
//! pecks pass through nothing its deepest does not: so those are not swept.
use crate::bench::p;
use crate::shared::cnc::{along, cut, line, swept, upright};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sketch::Step::{self, Arc, Forward, Heading, Start, To};
use kentron::topology::{Model, Shape};

const DEPTHS: [f64; 3] = [-4.0, -8.0, -12.0];
const MILL: f64 = 5.0;
const LENGTH: f64 = 30.0;

/// The wall path from the middle of its bottom side, (80, 25), anticlockwise.
const WALL: [Step; 10] = [
    Start(80.0, 25.0),
    Forward(48.0), Arc { radius: 7.0, angle: 90.0 },
    Forward(46.0), Arc { radius: 7.0, angle: 90.0 },
    Forward(96.0), Arc { radius: 7.0, angle: 90.0 },
    Forward(46.0), Arc { radius: 7.0, angle: 90.0 },
    Forward(48.0),
];

/// The island path from (80, 55), clockwise.
const ISLAND: [Step; 3] = [Start(80.0, 55.0), Heading(-90.0), Arc { radius: -20.0, angle: 360.0 }];

/// The intervals of the line at height `y` the mill's tip keeps to: within the wall path, its
/// corners' arcs about (32, 32) … (128, 78), and outside the island path.
fn raster(y: f64) -> Vec<(f64, f64)> {
    let below = (32.0 - y).max(y - 78.0).max(0.0);
    let inset = 7.0 - (49.0 - below * below).sqrt();
    let (left, right) = (25.0 + inset, 135.0 - inset);
    let across = 400.0 - (y - 55.0) * (y - 55.0);
    if across > 0.0 {
        let half = across.sqrt();
        vec![(left, 60.0 - half), (60.0 + half, right)]
    } else {
        vec![(left, right)]
    }
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -20.0), p(160.0, 110.0, 0.0)).build(m)?;
    // The pocket: at each depth the raster passes, the wall path and the island path.
    let mut pocket = Vec::new();
    for z in DEPTHS {
        for k in 0..15 {
            let y = 26.0 + 4.0 * k as f64;
            for (from, to) in raster(y) {
                pocket.push(line(p(from, y, z), p(to, y, z)));
            }
        }
        pocket.extend(along(&WALL, z));
        pocket.extend(along(&ISLAND, z));
    }
    let mut cuts = swept(m, |m| Cylinder::new(MILL, LENGTH).build(m), &pocket)?;
    // The bore: at each depth, the mill out from the centre and once round the circle of 6.
    let circle = [Start(66.0, 55.0), Heading(90.0), Arc { radius: 6.0, angle: 360.0 }];
    let mut bore = Vec::new();
    for z in [-3.0, -6.0] {
        bore.push(line(p(60.0, 55.0, z), p(66.0, 55.0, z)));
        bore.extend(along(&circle, z));
    }
    cuts.extend(swept(m, |m| Cylinder::new(3.0, LENGTH).build(m), &bore)?);
    // The holes: the drill (its point a cone of half-angle 45° up to its side, 4 above its tip)
    // plunged from z = 5 to its deepest, z = −14.
    let holes: Vec<_> = [(8.0, 8.0), (152.0, 8.0), (152.0, 102.0), (8.0, 102.0)].iter().map(|&(x, y)| line(p(x, y, 5.0), p(x, y, -14.0))).collect();
    let drill = [Start(0.0, 0.0), To(4.0, 4.0), To(4.0, LENGTH), To(0.0, LENGTH)];
    cuts.extend(swept(m, |m| upright(m, &drill), &holes)?);
    cut(m, plate, &cuts)
}
