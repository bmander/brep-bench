//! The half of the ball of radius 5 about the origin above the plane z = 0, its rim rounded to
//! radius 1, shrunk by 3/2: every point of it at least 3/2 inside it.
//!
//! The hemisphere is its half-section turned about the z axis, and the round is the kernel's round
//! of that section's corner, a torus exactly. (`fillet::roll` between the plane and the sphere in
//! space fits the round as a B-spline canal, which the offset cannot sweep.)
use cad_kernel::build::revolve;
use cad_kernel::corners::round;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::planar::{Region, Seg, Step};
use cad_kernel::recipe::region_profile;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const RADIUS: f64 = 5.;
const ROUND: f64 = 1.;
const SHRINK: f64 = 1.5;

/// The half-section's steps' tags: the flat, the ball, the axis, and the round between the first
/// two.
const FLAT: u32 = 1;
const BALL: u32 = 2;
const AXIS: u32 = 3;
const RIM: u32 = 4;

pub fn build() -> Result<Brep, String> {
    // the half-section in (ρ, z): out along the flat, up round the ball, down the axis
    let steps = [
        (Seg::Line { a: [0., 0.], b: [RADIUS, 0.] }, FLAT),
        (Seg::Arc { c: [0., 0.], r: RADIUS, a0: 0., sweep: FRAC_PI_2 }, BALL),
        (Seg::Line { a: [0., RADIUS], b: [0., 0.] }, AXIS),
    ];
    let steps = steps.into_iter().map(|(seg, tag)| Step { seg, chart: None, tag }).collect();
    let section = Region::new(vec![steps]);
    // the rim, where the flat meets the ball, rounded in the section (the kernel reads it as
    // convex and takes it off), then the whole turned about the axis
    let rounded = round(&section, &[FLAT], &[BALL], ROUND, RIM)?;
    let (origin, axis) = ([0., 0., 0.], [0., 0., 1.]);
    let profile = region_profile(&rounded, origin, [1., 0., 0.], axis, &|_| String::new());
    let hemisphere = revolve(&profile, origin, axis, TAU)?;
    shrunk(&hemisphere, SHRINK, Join::Round)
}
