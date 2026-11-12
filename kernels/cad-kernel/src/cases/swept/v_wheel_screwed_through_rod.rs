//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical V groove ground round
//! it by a wheel with a sharp 60° V rim — the triangle (4, 0, 0), (6.5, 0, 0) ± 2.5·tan 30°·along
//! turned about the axis through (14, 0, 0) along (0, cos h, −sin h), h = atan(2π·4.5 / 30) —
//! screwed about z, rising 30 a turn, from a turn of −144° to +384°.
//!
//! Through the kernel's closed sweep (`swept::closed::cut_from`): it refuses cones about an axis
//! leaning to the screw's (`Unbuilt::Curved`), and a tool it refuses so under a constant twist goes
//! the contact's road (geomsolver #880 Q1b) — each face's contact and each stretch of the sharp
//! rim's fan carried over the roll, the tool at the roll's two ends, every meeting of those given,
//! trimmed by membership, judged, and cut from the rod.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism, revolve};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{add, scale};
use std::f64::consts::TAU;

const UP: [f64; 3] = [0., 0., 1.];
/// How far the screw rises a whole turn.
const RISE: f64 = 30.;
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], UP, 5.), 0., 20.)?;
    let h = (TAU * 4.5 / RISE).atan();
    let along = [0., h.cos(), -h.sin()];
    let half = scale(along, 2.5 * 30f64.to_radians().tan());
    let back = [6.5, 0., 0.];
    let triangle = polygon(&[[4., 0., 0.], add(back, half), add(back, scale(half, -1.))]);
    let wheel = revolve(&triangle, [14., 0., 0.], along, TAU)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, [(-144f64).to_radians(), 384f64.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
