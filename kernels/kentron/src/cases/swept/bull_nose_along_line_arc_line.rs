//! A block, the box from (−10, −10, −10) to (40, 40, 0), grooved by a bull-nose mill run along a
//! line, an arc and a line: upright, a cylinder of radius 2 from z = −1.5 to 5 whose bottom corner
//! is rounded to 0.5, its axis carried from the vertical line through (0, 0) along x to (20, 0),
//! a quarter turn about the vertical line through (20, 10) to (30, 10), and along y to (30, 30).
//! Everything the mill passes through, cut.
//!
//! The mill is its half section revolved about its axis. Each of its three moves is kentron's
//! sweep of a solid (`sweep::solid`): a slide, a turn and a slide, each from where the move
//! before left the mill. Each is cut from the block in turn.
use crate::bench::{p, v};
use crate::shared::parts::xz_plane;
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cuboid;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let block = Cuboid::from_corners(p(-10.0, -10.0, -10.0), p(40.0, 40.0, 0.0)).build(m)?;
    // The mill: its half section, out from its axis (the flat bottom to 1.5, the round's quarter
    // circle about (1.5, −1), the side up to the top), turned whole about it.
    let half = [Start(0.0, -1.5), To(1.5, -1.5), Arc { radius: 0.5, angle: 90.0 }, To(2.0, 5.0), To(0.0, 5.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let mut mill = sweep::revolve(m, half, Axis::Z, TAU)?.shape;
    let moves = [
        RigidMotion::Slide { by: v(20.0, 0.0, 0.0) },
        RigidMotion::Turn { axis: Axis::new(p(20.0, 10.0, 0.0), Direction::Z), from: 0.0, to: FRAC_PI_2 },
        RigidMotion::Slide { by: v(0.0, 20.0, 0.0) },
    ];
    let mut sweeps = Vec::new();
    for motion in &moves {
        sweeps.push(sweep::solid(m, mill, motion)?);
        mill = modify::transform(m, mill, &motion.at(1.0), TransformOptions::default())?;
    }
    cut_each(m, block, &sweeps)
}
