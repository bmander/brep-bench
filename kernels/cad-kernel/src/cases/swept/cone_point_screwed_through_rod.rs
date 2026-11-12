//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by a pointed cone — its axis
//! the x axis, its point (4, 0, 0), opening at 60° to a flat base at x = 12 — screwed about z,
//! right-handed and rising 6 a turn, from a turn of −90° to 1350°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the cone whole: its closed class
//! refuses a cone about an axis square to the screw's (`Unbuilt::Curved`), and since 0.10.0 the
//! contact's road takes a screw past the axis — the cone's contact, run on to its point, and its
//! base rim's fan carried over the four turns as sheets, given their meetings and the end poses,
//! trimmed by membership in the sweep, and the result cut from the rod.
use cad_kernel::build::{cone, disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const POINT: V = [4., 0., 0.];
const LENGTH: f64 = 8.;
/// Each side of the cone from its axis, in degrees.
const HALF_ANGLE: f64 = 30.;
/// How far the screw rises a whole turn, and its start and stop, in degrees.
const RISE: f64 = 6.;
const FROM: f64 = -90.;
const TO: f64 = 1350.;
/// How closely the cone is meshed for the sweep's membership reads: at 0.002 the kernel writes the
/// same STEP file, byte for byte, and takes over half as long again.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0.; 3], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let base = LENGTH * HALF_ANGLE.to_radians().tan();
    let tool = cone(POINT, [1., 0., 0.], 0., base, LENGTH)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: RISE });
    let size = rod.size().max(tool.size());
    cut_from(&rod, &tool, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the cone's sweep not built: {why:?}"))
}
