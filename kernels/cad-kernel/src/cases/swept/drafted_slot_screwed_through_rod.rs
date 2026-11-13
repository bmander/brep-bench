//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by a drafted bar screwed
//! about z, right-handed and rising 6 a turn, from a turn of −90° to 1350°. The bar's section in
//! the plane x = 12 is the slot of the half discs of radius 1 about (12, ±1.5, 0) and the rectangle
//! between them; it is carried along −x to x = 4, drafted 3° inward, so its sides are two planes
//! and two half cones.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the bar whole: under a screw its
//! closed class takes a cone only about an axis along the screw's and refuses this bar's half
//! cones, which stand about axes square to it (`Unbuilt::Curved`), so the bar goes the contact's
//! road — its faces' contacts and its sharp edges' fans carried over the roll as sheets, given
//! their meetings and the end poses, trimmed by membership in the sweep, and the result cut from
//! the rod. Since 0.12.0 it is built: membership's mesh of the drafted cones is held to its sag
//! as measured over every triangle, where a point a hair off it was proved outside the sweep.
use cad_kernel::build::{disc, drafted, prism, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::{FRAC_PI_2, PI};

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// The section's plane, its half discs' centres' offset along y and their radius.
const FACE: f64 = 12.;
const OFFSET: f64 = 1.5;
const RADIUS: f64 = 1.;
/// How far the bar runs along −x, and its draft, in degrees.
const LENGTH: f64 = 8.;
const DRAFT: f64 = 3.;
/// How far the screw rises a whole turn, and its start and stop, in degrees.
const RISE: f64 = 6.;
const FROM: f64 = -90.;
const TO: f64 = 1350.;
/// How closely the bar is meshed for the sweep's membership reads.
const BAR: f64 = 0.002;

/// The slot in the plane x = 12, facing −x: its two half discs' arcs (each turning about −x, from
/// its top across its far side to its bottom) and the two lines between them.
fn slot() -> Profile {
    let normal = [-1., 0., 0.];
    let arc = |y: f64, from: f64| ProfileEdge::Arc {
        frame: Frame::new([FACE, y, 0.], normal, [0., 1., 0.]),
        r: RADIUS,
        span: Some([from, from + PI]),
    };
    let edges = vec![
        arc(OFFSET, -FRAC_PI_2),
        ProfileEdge::Line { a: [FACE, OFFSET, -RADIUS], b: [FACE, -OFFSET, -RADIUS] },
        arc(-OFFSET, FRAC_PI_2),
        ProfileEdge::Line { a: [FACE, -OFFSET, RADIUS], b: [FACE, OFFSET, RADIUS] },
    ];
    Profile { origin: [FACE, 0., 0.], normal, loops: vec![edges], names: Vec::new() }
}

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0.; 3], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // the kernel's drafted prism: the slot carried along its normal, each side tapered into the
    // material (a line's into a plane, a half circle's into a half cone)
    let bar = drafted(&slot(), 0., LENGTH, DRAFT.to_radians())?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: up, ratio: 1., advance: RISE });
    let size = rod.size().max(bar.size());
    cut_from(&rod, &bar, &path, [FROM.to_radians(), TO.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
