//! The rod and groove of `swept/torus-screwed-through-rod` turned rigidly so that the rod's axis is
//! w = (1, 2, 2)/3: the cylinder of radius 5 about w from the origin to 20w, less everything a wheel
//! passes through, the torus of radii 6 and 2 about 11.5u set along the helix the screw carries 5.5u
//! along, screwed about w, rising 30 a turn, from a turn of −108° to one of 348°.
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{scale, sub};
use std::f64::consts::TAU;

/// The turned frame: u where the case before it had x, v where it had y, w where it had z.
const U: V = [2. / 3., -2. / 3., 1. / 3.];
const V_: V = [2. / 3., 1. / 3., -2. / 3.];
const W: V = [1. / 3., 2. / 3., 2. / 3.];
/// How far the screw rises a whole turn.
const RISE: f64 = 30.;
/// How far the screw turns from where the wheel is drawn, in degrees: down to, and up to.
const TURNS: [f64; 2] = [-108., 348.];
/// How closely the wheel is meshed for the sweep's membership reads.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let rod = prism(&disc([0.; 3], W, 5.), 0., 20.)?;
    // the wheel set along the helix the screw carries 5.5u along: its axis cos h·v − sin h·w
    let h = (TAU * 5.5 / RISE).atan();
    let axis = sub(scale(V_, h.cos()), scale(W, h.sin()));
    let wheel = torus(scale(U, 11.5), axis, 6., 2.)?;
    // the screw about w: a radian a radian of the roll, rising `RISE` a turn
    let path = Trajectory::Twist(Screw { origin: [0.; 3], axis: W, ratio: 1., advance: RISE });
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, TURNS.map(f64::to_radians), BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
