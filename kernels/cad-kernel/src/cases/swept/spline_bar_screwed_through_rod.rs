//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by a bar whose section, in
//! the plane y = 0, is the closed uniform periodic cubic B-spline on the control points
//! (3.5, 0), (4.5, −1.5), (8, −1.5), (12, −1), (12, 1), (8, 1.5), (4.5, 1.5) in (x, z), carried
//! from y = −8 to 8; screwed about z — right-handed, rising 6 a turn — from a turn of −90° to
//! 1350°.
//!
//! The kernel's closed sweep (`swept::closed::cut_from`) takes the bar whole: under a screw its
//! closed class takes planes, balls, and cylinders and cones about axes along the screw's, and
//! refuses the spline's surface of extrusion (`Unbuilt::Curved`); since 0.12.0 such a tool goes
//! the contact's road — the contact of its free-form side, a face closed across its own seam, cut
//! where its burial in the tool changes, and its end rims' fans carried over the roll as sheets,
//! given their meetings and the end poses, trimmed by membership in the sweep, and the result cut
//! from the rod.
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::sync::Arc;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// The section's control points, in (x, z), in order round the loop.
const CONTROL: [[f64; 2]; 7] =
    [[3.5, 0.], [4.5, -1.5], [8., -1.5], [12., -1.], [12., 1.], [8., 1.5], [4.5, 1.5]];
const DEGREE: usize = 3;
const HALF_LENGTH: f64 = 8.;
const LEAD: f64 = 6.;
const ROLL: [f64; 2] = [-90f64.to_radians(), 1350f64.to_radians()];
/// How closely the bar is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // the periodic curve written the way cad-kernel holds a B-spline: its poles with the first
    // three repeated after the last, on uniform knots, so its domain [3, 10] runs once round
    let poles: Vec<_> = (0..CONTROL.len() + DEGREE)
        .map(|k| { let [x, z] = CONTROL[k % CONTROL.len()]; [x, 0., z] })
        .collect();
    let knots = (0..poles.len() + DEGREE + 1).map(|k| k as f64).collect();
    let curve = BSpline::new(DEGREE, knots, poles)?;
    let section = Profile { origin: [0., 0., 0.], normal: [0., 1., 0.],
                            loops: vec![vec![ProfileEdge::Spline(Arc::new(curve))]],
                            names: Vec::new() };
    let bar = prism(&section, -HALF_LENGTH, HALF_LENGTH)?;
    // the screw about z: a radian a radian of the roll, rising the lead a turn
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD,
    });
    let size = rod.size().max(bar.size());
    cut_from(&rod, &bar, &path, ROLL, BAR, tolerance(size))
        .map_err(|why| format!("the bar's sweep not built: {why:?}"))
}
