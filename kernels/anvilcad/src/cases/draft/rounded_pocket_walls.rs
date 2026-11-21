//! The block from the origin to (40, 30, 15) with a pocket into its top, the box from (8, 8, 5) to
//! (32, 22, 15) with its vertical edges rounded to radius 4; the pocket's eight walls drafted 5°
//! about its floor, pulling along +z.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{block, draft, e, region, rounded_rect, rounded_rect_sides, subtract};

pub fn build() -> Result<Solid, String> {
    let outline = rounded_rect((8., 8.), (32., 22.), 4.);
    let pocket = extrude::extrude(&Frame::world(), &region(&[outline])?, 5., 16.).map_err(e)?;
    let pocketed = subtract(&block([0., 0., 0.], [40., 30., 15.])?, &pocket)?;
    draft(&pocketed, &rounded_rect_sides((8., 8.), (32., 22.), 4., 10.), [0., 0., 5.], [0., 0., 1.], 5.)
}
