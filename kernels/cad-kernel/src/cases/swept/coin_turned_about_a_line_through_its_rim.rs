//! The solid a disc fills — the cylinder of radius 2 about the line through (2, 0, 0) along y, from
//! y = 0 to 0.5, its faces flat — as it turns rigidly about the z axis, right-handed, by 90°. Its
//! rim in the plane y = 0 passes through the origin, on the axis.
//!
//! The kernel's closed sweep (`swept::closed::closed`) takes the disc whole: its closed class
//! refuses a sharp circle through the turn's axis (`Unbuilt::Curved`), and since 0.10.0 such a tool
//! goes the contact's road under a turn — the rim through the axis, in its side's contact all
//! round, cut where it touches the axis, its far half (which moves into the disc) buried and the
//! rest turned into one horn torus closing at the axis; the other rim's fan on a surface of
//! revolution, and the side's top and bottom lines turned into sectors of planes.
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::closed;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const UP: V = [0., 0., 1.];
const CENTRE: V = [2., 0., 0.];
const ALONG: V = [0., 1., 0.];
const RADIUS: f64 = 2.;
const THICKNESS: f64 = 0.5;
/// The turn, in degrees.
const TURN: f64 = 90.;
/// How closely the disc is meshed for the sweep's membership reads: a thousandth of its radius.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let coin = prism(&disc(CENTRE, ALONG, RADIUS), 0., THICKNESS)?;
    // the turn about z: a screw that does not advance, a radian a radian of the roll
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: 0. });
    // the kernel's closed swept solid: the side, a cylinder about an axis square to the turn's,
    // touching along a curve turned into a surface of revolution, and each rim a sharp circle
    // leaning to the axis turned into its ring
    closed(&coin, &path, [0., TURN.to_radians()], BAR, tolerance(coin.size()))
        .map_err(|why| format!("the disc's sweep not built: {why:?}"))
}
