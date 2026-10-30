//! A gear blank, the disc of radius 22 about y from y = 0 to 6, with one tooth space cut by a rack
//! tooth — the trapezoid with corners (18, ±0.842856) and (24, ±3.026666) in (x, z), from y = −2 to
//! 8 — whose pitch line x = 20 rolls without slipping on the pitch circle of radius 20, from 60° on
//! one side of the tooth's middle to 60° on the other.
use crate::sketch::polygon;
use cad_kernel::build::{disc, prism};
use cad_kernel::sweep::cut::{swept_cut, Motion, Tool};
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;

const BLANK_RADIUS: f64 = 22.;
const BLANK_WIDTH: f64 = 6.;
const PITCH_RADIUS: f64 = 20.;
const TOOTH: [[f64; 2]; 4] = [[18., -0.842856], [18., 0.842856], [24., 3.026666], [24., -3.026666]];
const TOOTH_SPAN: [f64; 2] = [-2., 8.]; // along y
const ROLL_DEGREES: f64 = 60.;
/// How closely the sweep's sheet is held to its surface, in the kernel's millimetres.
const TOLERANCE: f64 = 1e-4;

pub fn build() -> Result<Brep, String> {
    let along = [0., 1., 0.];
    let blank = prism(&disc([0., 0., 0.], along, BLANK_RADIUS), 0., BLANK_WIDTH)?;
    // the tooth's section in the plane y = 0, its normal along +y, so the prism runs along y
    let mut section = polygon(&TOOTH.map(|[x, z]| [x, 0., z]));
    if section.normal[1] < 0. {
        section = polygon(&TOOTH.map(|[x, z]| [x, 0., z]).into_iter().rev().collect::<Vec<_>>());
    }
    let tooth = Tool::Prism { name: "tooth".into(), profile: section, from: TOOTH_SPAN[0],
        to: TOOTH_SPAN[1], draft: None };
    // the generating motion: the rack slides along its pitch line while the blank turns under it,
    // the pitch line rolling on the pitch circle — the pitch point (20, 0, 0) turned about −y moves
    // along +z at 20 a radian, as the rack does — read as the rack seen from the blank
    let rack = Motion::Slide { direction: [0., 0., 1.], advance: TAU * PITCH_RADIUS }
        .seen_from(Motion::Turn { origin: [0., 0., 0.], axis: [0., -1., 0.], ratio: 1. });
    let roll = [-ROLL_DEGREES.to_radians(), ROLL_DEGREES.to_radians()];
    swept_cut(&blank, &tooth, &rack, roll, Some(TOLERANCE)).map_err(|e| e.to_string())
}
