//! An open tray on its back, opening toward −y: the block from (−36, 0, −24) to (36, 16, 24) less
//! the pocket, the box from (−33, 0, −21) to (33, 13, 21); then four standoffs joined to it on the
//! floor's inside face y = 13, each a tube along y from y = 5 to y = 13, outer radius 5 and inner
//! radius 2, on the lines through (x, z) = (±26, ±14), its hole blind, stopping at the floor.
use crate::solids::fillet::{subtract, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-36.0, 0.0, -24.0), 72.0, 16.0, 48.0)?;
    let pocket = make_box_brep(p(-33.0, 0.0, -21.0), 66.0, 13.0, 42.0)?;
    let mut tray = subtract(&block, &pocket)?;
    let along = p(0.0, 1.0, 0.0);
    for (x, z) in [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)] {
        let outer = make_cylinder_brep(p(x, 5.0, z), along, 5.0, 8.0)?;
        // The tube's own hole, cut by a cylinder past both of its ends; the tube stands on the
        // floor, which closes the hole there.
        let inner = make_cylinder_brep(p(x, 4.0, z), along, 2.0, 10.0)?;
        tray = union(&tray, &subtract(&outer, &inner)?)?;
    }
    Ok(tray)
}
