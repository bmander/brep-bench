//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove ground round it
//! by a torus of radii 6 and 2, centred at (11.5, 0, 0) with its axis (0, cos h, −sin h), h =
//! atan(2π·5.5 / 30), screwed about z — rising 30 a turn — from a turn of −108° to +348°.
//!
//! Through the kernel's closed sweep (`swept::closed::cut_from`): it refuses a torus by what it is
//! (`Unbuilt::Curved`), and a tool it refuses so under a constant twist goes the contact's road
//! (geomsolver #880 Q1b) — each face's contact carried over the roll, the tool at the roll's two
//! ends, every meeting of those given, trimmed by membership, judged, and cut from the rod.
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const UP: [f64; 3] = [0., 0., 1.];
/// How far the screw rises a whole turn.
const RISE: f64 = 30.;
/// How closely the torus is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], UP, 5.), 0., 20.)?;
    let h = (TAU * 5.5 / RISE).atan();
    let wheel = torus([11.5, 0., 0.], [0., h.cos(), -h.sin()], 6., 2.)?;
    // the screw about z: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: UP, ratio: 1., advance: RISE });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, [(-108f64).to_radians(), 348f64.to_radians()], BAR, tolerance(size))
        .map_err(|why| format!("the torus's sweep not built: {why:?}"))
}
