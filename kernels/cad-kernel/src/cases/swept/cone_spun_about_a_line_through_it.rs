//! The solid a cone fills as it turns rigidly about the z axis, right-handed, by 150°: its axis the
//! line through the origin along w = (cos 30°, 0, sin 30°), its point at −2w, opening at 50° to a
//! flat base at 5w of radius 7 tan 25°. The z axis crosses it aslant, through the origin.
//!
//! The kernel's closed sweep (`swept::closed::closed`) takes the cone whole: under a turn its
//! closed class takes a cone about a leaning axis where its contact is a curve over the face clear
//! of the turn's axis, and refuses one the turn's axis crosses within the face (`Unbuilt::Curved`);
//! since 0.10.0 such a tool goes the contact's road — each sheet on the surface of revolution of
//! its section's meridian, cut where the meridian folds back and where it crosses the axis, and
//! trimmed by membership in the sweep.
use cad_kernel::build::cone;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::scale;

const UP: V = [0., 0., 1.];
/// How far the cone's axis rises from x toward z, in degrees.
const RISE: f64 = 30.;
/// Where its point and its base stand along w.
const POINT: f64 = -2.;
const BASE: f64 = 5.;
/// Each side of the cone from its axis, in degrees.
const HALF_ANGLE: f64 = 25.;
/// The turn, in degrees.
const TURN: f64 = 150.;
/// How closely the cone is meshed for the sweep's membership reads.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let (s, c) = RISE.to_radians().sin_cos();
    let w = [c, 0., s];
    let height = BASE - POINT;
    let tool = cone(scale(w, POINT), w, 0., height * HALF_ANGLE.to_radians().tan(), height)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid: a cone about an axis leaning to the turn's touches along a
    // curve on each ring, turned into a surface of revolution — here crossing the turn's axis
    closed(&tool, &path, [0., TURN.to_radians()], BAR, tolerance(tool.size()))
        .map_err(|why| format!("the cone's sweep not built: {why:?}"))
}
