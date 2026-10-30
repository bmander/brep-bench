//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut round it by a ball of radius 1 whose centre runs along the helix of radius 5 about the
//! z axis from (5, 0, −2), turning right-handed and rising 4 a turn, for six turns, to (5, 0, 22).
//!
//! vcad has no sweep of a solid (`Solid::sweep` moves a sketch profile), so the ball's sweep is
//! built from what it is by elementary geometry: every point within 1 of the helix. That is the
//! tube the ball's great disc square to the helix fills as it runs, vcad's sweep of the disc along
//! its helix (its turns 4 apart, more than the ball's width 2, so it never meets itself), with a
//! ball at each end. The ball where it starts lies wholly below z = −1 and where it ends wholly
//! above z = 21, and so do the tube's end discs, so the end balls are clear of the rod and what the
//! sweep takes from it is the tube's alone: the tube is the tool cut here.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::{Helix, SweepOptions};
use vcad_kernel::Solid;

const ROD_RADIUS: f64 = 5.0;
const ROD_LENGTH: f64 = 20.0;
const BALL_RADIUS: f64 = 1.0;
const HELIX_RADIUS: f64 = 5.0;
const START_Z: f64 = -2.0;
const PITCH: f64 = 4.0;
const TURNS: f64 = 6.0;
/// The disc's circle is cut into this many arcs, each into SEGMENTS / ARCS lines.
const ARCS: u32 = 4;

// workaround: no sweep of a solid
pub fn build() -> Result<Solid, crate::Refusal> {
    let rod = Solid::cylinder(ROD_RADIUS, ROD_LENGTH, SEGMENTS);
    // vcad's helix is about the z axis through its centre, starts on +x and turns from +x towards
    // +y as it rises
    let path = Helix::new(HELIX_RADIUS, PITCH, PITCH * TURNS, TURNS)
        .with_center(Point3::new(0.0, 0.0, START_Z));
    // the sweep reads the disc in sketch coordinates about the path: a circle about the origin.
    // vcad's sweep is faceted: the circle cut into SEGMENTS lines, as a primitive's is, and the
    // helix sampled at SEGMENTS places a turn
    let disc = SketchProfile::circle(Point3::origin(), Vec3::z(), BALL_RADIUS, ARCS);
    let options = SweepOptions {
        path_segments: SEGMENTS * TURNS as u32,
        arc_segments: SEGMENTS / ARCS,
        ..SweepOptions::default()
    };
    let tube = Solid::sweep(disc, &path, options)?;
    Ok(rod.try_difference(&tube)?)
}
