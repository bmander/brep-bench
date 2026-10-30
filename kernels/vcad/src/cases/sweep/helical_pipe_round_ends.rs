//! A pipe of circular section, radius 1, along the helix of radius 10 about the z axis that starts
//! at (10, 0, 0), turns right-handed and rises 5 a turn, for three turns, each end closed by a
//! half ball of radius 1.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::{Helix, SweepOptions};
use vcad_kernel::Solid;

const RADIUS: f64 = 1.0;
const HELIX_RADIUS: f64 = 10.0;
const PITCH: f64 = 5.0;
const TURNS: f64 = 3.0;
/// The section's circle is cut into this many arcs, each into SEGMENTS / ARCS lines.
const ARCS: u32 = 4;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's helix is about the z axis, starts on +x and turns from +x towards +y as it rises
    let path = Helix::new(HELIX_RADIUS, PITCH, PITCH * TURNS, TURNS);
    // the sweep reads the section in sketch coordinates about the path: a circle about the origin
    let section = SketchProfile::circle(Point3::origin(), Vec3::z(), RADIUS, ARCS);
    // vcad's sweep is faceted: the circle cut into SEGMENTS lines, as a primitive's is, and the
    // helix sampled at SEGMENTS places a turn
    let options = SweepOptions {
        path_segments: SEGMENTS * TURNS as u32,
        arc_segments: SEGMENTS / ARCS,
        ..SweepOptions::default()
    };
    let pipe = Solid::sweep(section, &path, options)?;
    // each end is closed by a ball on the helix's end, its inner half inside the pipe; after
    // whole turns the helix ends above its start
    let start = Solid::sphere(RADIUS, SEGMENTS).translate(HELIX_RADIUS, 0.0, 0.0);
    let end = Solid::sphere(RADIUS, SEGMENTS).translate(HELIX_RADIUS, 0.0, PITCH * TURNS);
    Ok(pipe.try_union(&start)?.try_union(&end)?)
}
