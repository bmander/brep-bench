//! The 20 × 14 rectangle with its corners rounded to radius 2, extruded 10 up z, shelled inward
//! 1. thick, open at its top: a tray.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, region, rounded_rect, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let outline = rounded_rect((0., 0.), (20., 14.), 2.);
    let plate = extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 10.).map_err(e)?;
    shell_open(&plate, &[[10., 7., 10.]], 1.)
}
