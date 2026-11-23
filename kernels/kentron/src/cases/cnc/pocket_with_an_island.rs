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
//! The plate less everything the tools pass through. kentron has no sweep of a solid, so each
//! move's sweep is built as elementary geometry gives it (`shared::cnc`): a level slide of a flat
//! mill is its stadium's prism, a lap round a path the band within its radius of the path
//! extruded, and a slide of a tool along its own axis the tool lengthened. A plunge onto a pass's start, onto a path or
//! back up from it passes through nothing that pass or the column the tool stands in at that
//! depth does not, and the drill's pecks nothing its deepest does, so those are not cut again.
use crate::bench::{frame, p};
use crate::shared::cnc::{flat_milled_round, level_path, slid_flat_mill};
use crate::shared::swept::cut_each;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cuboid, Cylinder};
use kentron::sketch::Step::{self, Arc, Forward, Heading, Start};
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
    let mut cuts = Vec::new();
    for z in DEPTHS {
        for k in 0..15 {
            let y = 26.0 + 4.0 * k as f64;
            for (from, to) in raster(y) {
                cuts.push(slid_flat_mill(m, (from, y), (to, y), z, MILL, LENGTH)?);
            }
        }
        let wall = level_path(m, z, &WALL)?;
        cuts.push(flat_milled_round(m, wall, MILL, LENGTH)?);
        let island = level_path(m, z, &ISLAND)?;
        cuts.push(flat_milled_round(m, island, MILL, LENGTH)?);
    }
    // The bore: at each depth, the mill out from the centre and once round the circle of 6.
    let circle = [Start(66.0, 55.0), Heading(90.0), Arc { radius: 6.0, angle: 360.0 }];
    for z in [-3.0, -6.0] {
        cuts.push(slid_flat_mill(m, (60.0, 55.0), (66.0, 55.0), z, 3.0, LENGTH)?);
        let path = level_path(m, z, &circle)?;
        cuts.push(flat_milled_round(m, path, 3.0, LENGTH)?);
    }
    // The holes: the drill at its deepest, its point from z = −14 to −10, its side up past the
    // plate.
    for (x, y) in [(8.0, 8.0), (152.0, 8.0), (152.0, 102.0), (8.0, 102.0)] {
        cuts.push(Cone::new(0.0, 4.0, 4.0).with_frame(frame(p(x, y, -14.0), Direction::Z)).build(m)?);
        cuts.push(Cylinder::new(4.0, 26.0).with_frame(frame(p(x, y, -10.0), Direction::Z)).build(m)?);
    }
    cut_each(m, plate, &cuts)
}
