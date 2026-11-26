//! The rod and bar of `swept/spline-bar-screwed-through-rod`, ten times larger: the cylinder of
//! radius 50 about z from z = 0 to 200, less everything a bar passes through as it is screwed about
//! z, rising 60 a turn, from a turn of −90° to one of 1350°; the bar's section in the plane y = 0 the
//! closed uniform cubic B-spline on seven control points, carried from y = −80 to 80.
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::BSpline;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::sync::Arc;

const ROD_RADIUS: f64 = 50.;
const ROD_LENGTH: f64 = 200.;
/// The section's control points, in (x, z), in order round the loop.
const CONTROL: [[f64; 2]; 7] =
    [[35., 0.], [45., -15.], [80., -15.], [120., -10.], [120., 10.], [80., 15.], [45., 15.]];
const DEGREE: usize = 3;
const HALF_LENGTH: f64 = 80.;
const LEAD: f64 = 60.;
const ROLL: [f64; 2] = [-90f64.to_radians(), 1350f64.to_radians()];
/// How closely the bar is meshed for the sweep's membership reads.
const BAR: f64 = 0.1;

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
