//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −2 to 2
//! and polar angle from −30° to 30°, with a helical groove milled into it by a ball-end mill: about
//! the vertical line through (10, 0), the lower half of the ball of radius 1 about (10, 0, 0) under
//! a cylinder of radius 1 up to z = 3, carried along the screw about the z axis, right-handed,
//! rising 3 a turn, from a turn of −60° to +60°. Everything the mill passes through, cut.
//!
//! The mill is its half section revolved about its axis; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one screw), cut from the block.
use crate::bench::p;
use crate::shared::parts::xz_plane;
use crate::shared::primitives::rectangle_xz;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_3, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Heading, Start, To};
    let section = rectangle_xz(m, (8.0, -2.0), (12.0, 2.0))?;
    let section = modify::transform(m, section, &Transform::rotation(Axis::Z, -FRAC_PI_3 / 2.0), TransformOptions::default())?;
    let block = sweep::revolve(m, section, Axis::Z, FRAC_PI_3)?.shape;
    // The mill: its half section, out from its axis (the ball's quarter circle from its lowest
    // point up to its equator, the side up to the top), turned whole about it.
    let half = [Start(10.0, -1.0), Heading(0.0), Arc { radius: 1.0, angle: 90.0 }, To(11.0, 3.0), To(10.0, 3.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let mill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 3.0, from: (-60f64).to_radians(), to: 60f64.to_radians() };
    let swept = sweep::solid(m, mill, &screw)?;
    kentron::boolean::difference(m, block, swept)
}
