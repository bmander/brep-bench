//! The solid of `swept/cone-spun-about-a-line-through-it`, ten times larger: a cone, its axis
//! through the origin along (cos 30°, 0, sin 30°), its point at −20 and its base at 50 along that
//! axis, opening at 25° each side, turned 150° about the z axis, which crosses it.
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
const POINT: f64 = -20.;
const BASE: f64 = 50.;
/// Each side of the cone from its axis, in degrees.
const HALF_ANGLE: f64 = 25.;
/// The turn, in degrees.
const TURN: f64 = 150.;
/// How closely the cone is meshed for the sweep's membership reads.
const BAR: f64 = 0.02;

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
