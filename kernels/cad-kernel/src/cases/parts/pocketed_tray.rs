//! A tray: block (−36, 0, −24)–(36, 16, 24) pocketed from y = 0, four blind-bored standoffs inside.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const HALF_LENGTH: f64 = 36.;
const DEPTH: f64 = 16.;
const HALF_HEIGHT: f64 = 24.;
const WALL: f64 = 3.;
const OUTER_RADIUS: f64 = 5.;
const INNER_RADIUS: f64 = 2.;
const STANDOFF_FOOT: f64 = 5.;
const AXES: [(f64, f64); 4] = [(26., 14.), (-26., 14.), (-26., -14.), (26., -14.)];

pub fn build() -> Result<Brep, String> {
    let floor = DEPTH - WALL;
    let stock = block([-HALF_LENGTH, 0., -HALF_HEIGHT], [HALF_LENGTH, DEPTH, HALF_HEIGHT])?;
    // the pocket, its cutter reaching 1 past the face at y = 0 it opens through
    let pocket = block(
        [-HALF_LENGTH + WALL, -1., -HALF_HEIGHT + WALL],
        [HALF_LENGTH - WALL, floor, HALF_HEIGHT - WALL],
    )?;
    let mut tray = combined(&stock, &pocket, Op::Cut, 0.)?;
    let along = [0., 1., 0.];
    for (x, z) in AXES {
        // a tube from y = 5 to the floor: a cylinder less its core over the same span, its hole
        // stopping at the floor
        let height = floor - STANDOFF_FOOT;
        let foot = [x, STANDOFF_FOOT, z];
        let post = prism(&disc(foot, along, OUTER_RADIUS), 0., height)?;
        let core = prism(&disc(foot, along, INNER_RADIUS), 0., height)?;
        let tube = combined(&post, &core, Op::Cut, 0.)?;
        // standing on the floor's inside face, joined to it there
        tray = combined(&tray, &tube, Op::Union, 0.)?;
    }
    Ok(tray)
}
