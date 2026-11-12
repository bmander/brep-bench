//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by the flat end of an end
//! mill — the cylinder of radius 1.2 about the x axis from x = 4 to 12 — screwed about z,
//! right-handed and rising 6 a turn, from a turn of −90° to 1350°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the mill whole: its closed class
//! refuses a cylinder about an axis square to the screw's (`Unbuilt::Curved`), and since 0.10.0 the
//! contact's road takes a screw past the axis — the side's contact and each end rim's fan carried
//! over the four turns as sheets, given their meetings and the end poses, trimmed by membership in
//! the sweep, and the result cut from the rod.
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const END: V = [4., 0., 0.];
const RADIUS: f64 = 1.2;
const LENGTH: f64 = 8.;
/// How far the screw rises a whole turn, and its start and stop, in degrees.
const RISE: f64 = 6.;
const FROM: f64 = -90.;
const TO: f64 = 1350.;
/// How closely the mill is meshed for the sweep's membership reads.
const BAR: f64 = 0.002;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0.; 3], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let mill = prism(&disc(END, [1., 0., 0.], RADIUS), 0., LENGTH)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: RISE });
    let size = rod.size().max(mill.size());
    cut_from(&rod, &mill, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
