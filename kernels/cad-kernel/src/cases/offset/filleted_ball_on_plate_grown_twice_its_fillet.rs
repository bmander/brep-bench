//! The plate of radius 8 on the z axis from z = −2 to z = 0 joined to the ball of radius 3 about
//! (0, 0, 1), the concave edge where the ball meets the plate's top filleted to radius 1.5, grown
//! by 3 with rounded edges: every point within 3 of it.
//!
//! The plate and the ball share the z axis, so the joined solid is their half-sections joined and
//! turned about it, and the fillet is the kernel's round of that section's corner: a ball of the
//! fillet's radius rolled between the plate's top and the sphere. (`fillet::roll` between the plane
//! and the sphere in space fails to build this fillet.)
use cad_kernel::build::revolve;
use cad_kernel::corners::round;
use cad_kernel::offset::{grown, Join};
use cad_kernel::planar::{Region, Seg, Step};
use cad_kernel::recipe::region_profile;
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const PLATE_RADIUS: f64 = 8.;
const PLATE_DEPTH: f64 = 2.;
const BALL_CENTRE: f64 = 1.;
const BALL_RADIUS: f64 = 3.;
const FILLET: f64 = 1.5;
const GROWTH: f64 = 3.;

/// The half-section's steps' tags: the plate's bottom, side and top, the ball, the axis, and the
/// fillet between the top and the ball.
const BOTTOM: u32 = 1;
const SIDE: u32 = 2;
const TOP: u32 = 3;
const BALL: u32 = 4;
const AXIS: u32 = 5;
const ROOT: u32 = 6;

pub fn build() -> Result<Brep, String> {
    // where the ball leaves the plate's top, the circle of radius √8 at z = 0, and its angle about
    // the ball's centre in the half-section
    let leaves = (BALL_RADIUS * BALL_RADIUS - BALL_CENTRE * BALL_CENTRE).sqrt();
    let from = (-BALL_CENTRE).atan2(leaves);
    // the half-section in (ρ, z): along the bottom, up the side, in along the top to the ball, up
    // round the ball to the axis and down it
    let steps = [
        (Seg::Line { a: [0., -PLATE_DEPTH], b: [PLATE_RADIUS, -PLATE_DEPTH] }, BOTTOM),
        (Seg::Line { a: [PLATE_RADIUS, -PLATE_DEPTH], b: [PLATE_RADIUS, 0.] }, SIDE),
        (Seg::Line { a: [PLATE_RADIUS, 0.], b: [leaves, 0.] }, TOP),
        (Seg::Arc { c: [0., BALL_CENTRE], r: BALL_RADIUS, a0: from, sweep: FRAC_PI_2 - from },
         BALL),
        (Seg::Line { a: [0., BALL_CENTRE + BALL_RADIUS], b: [0., -PLATE_DEPTH] }, AXIS),
    ];
    let steps = steps.into_iter().map(|(seg, tag)| Step { seg, chart: None, tag }).collect();
    let section = Region::new(vec![steps]);
    // the corner where the ball meets the top filleted in the section (the kernel reads it as
    // concave and fills it), then the whole turned about the axis
    let filleted = round(&section, &[TOP], &[BALL], FILLET, ROOT)?;
    let (origin, axis) = ([0., 0., 0.], [0., 0., 1.]);
    let profile = region_profile(&filleted, origin, [1., 0., 0.], axis, &|_| String::new());
    let part = revolve(&profile, origin, axis, TAU)?;
    grown(&part, GROWTH, Join::Round)
}
