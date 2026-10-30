//! A wheel, the disc of radius 40 about the vertical line through (−50, 0) from z = −8 to −2, cut by
//! a plus (four arms 30 long and 8 wide about the origin, z from −10 to 0) turning about z by t
//! while the wheel turns by −2t/3, t from 0° to 540°.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;

const WHEEL_CENTRE: [f64; 3] = [-50., 0., 0.];
const WHEEL_RADIUS: f64 = 40.;
const WHEEL_SPAN: [f64; 2] = [-8., -2.];
const ARM: f64 = 30.;
const HALF_WIDTH: f64 = 4.;
const TOOL_SPAN: [f64; 2] = [-10., 0.];
const WHEEL_RATIO: f64 = -2. / 3.;
const ROLL_DEGREES: f64 = 540.;
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let base = [WHEEL_CENTRE[0], WHEEL_CENTRE[1], WHEEL_SPAN[0]];
    let wheel = prism(&disc(base, up, WHEEL_RADIUS), 0., WHEEL_SPAN[1] - WHEEL_SPAN[0])?;
    let (a, w) = (ARM, HALF_WIDTH);
    let plus = polygon(&[[w, -w, 0.], [a, -w, 0.], [a, w, 0.], [w, w, 0.], [w, a, 0.], [-w, a, 0.],
        [-w, w, 0.], [-a, w, 0.], [-a, -w, 0.], [-w, -w, 0.], [-w, -a, 0.], [w, -a, 0.]]);
    let tool = Tool::Prism { name: "plus".into(), profile: plus, from: TOOL_SPAN[0], to: TOOL_SPAN[1],
        draft: None };
    // the plus turning about z, seen from the wheel turning two thirds as fast the other way
    let mesh = Motion::Turn { origin: [0., 0., 0.], axis: up, ratio: 1. }
        .seen_from(Motion::Turn { origin: WHEEL_CENTRE, axis: up, ratio: WHEEL_RATIO });
    swept_cut(&wheel, &tool, &mesh, [0., ROLL_DEGREES.to_radians()], Some(TOLERANCE))
        .map_err(|e| e.to_string())
}
