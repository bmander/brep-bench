//! An L-shaped outline in the xy plane extruded 8 along +z.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, polygon, region};

pub fn build() -> Result<Solid, String> {
    let outline = polygon(&[(0., 0.), (20., 0.), (20., 5.), (5., 5.), (5., 15.), (0., 15.)]);
    extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 8.).map_err(e)
}
