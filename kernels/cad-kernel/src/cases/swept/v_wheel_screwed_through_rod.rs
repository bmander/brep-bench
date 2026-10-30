//! A rod, the cylinder of radius 5 about z from z = 0 to 20, threaded by a wheel with a sharp 60°
//! V rim: the triangle with its tip at (4, 0, 0) and its other corners 2.5 further out, ±2.5·tan 30°
//! along the wheel's axis, turned about that axis, which passes through (14, 0, 0) along
//! (0, cos h, −sin h), h = atan(2π·4.5 / 30); screwed about z — rising 30 a turn — from a turn of
//! −144° to +384°.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, scale};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const AXIS_THROUGH: [f64; 3] = [14., 0., 0.];
const TIP: [f64; 3] = [4., 0., 0.];
const DEPTH: f64 = 2.5; // from the tip to the bore, outward
const FLANK_DEGREES: f64 = 30.; // each flank from the wheel's mid-plane
const HELIX_RADIUS: f64 = 4.5; // where the wheel is set along the helix
const LEAD: f64 = 30.;
const ROLL_DEGREES: [f64; 2] = [-144., 384.];
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let h = (TAU * HELIX_RADIUS / LEAD).atan();
    let along = [0., h.cos(), -h.sin()];
    let half = DEPTH * FLANK_DEGREES.to_radians().tan();
    let back = add(TIP, [DEPTH, 0., 0.]);
    let v = polygon(&[TIP, add(back, scale(along, -half)), add(back, scale(along, half))]);
    let wheel = Tool::Revolved { name: "wheel".into(), profile: v, origin: AXIS_THROUGH, axis: along };
    // cad-kernel's own solid sweep: a turn about z, right-handed, advancing LEAD a turn
    let screw = Motion::Screw { origin: [0., 0., 0.], axis: up, ratio: 1., advance: LEAD };
    let roll = ROLL_DEGREES.map(f64::to_radians);
    swept_cut(&rod, &wheel, &screw, roll, Some(TOLERANCE)).map_err(|e| e.to_string())
}
