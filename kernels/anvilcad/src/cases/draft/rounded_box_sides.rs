//! The box from the origin to (30, 20, 10) with its vertical edges rounded to radius 4, all eight
//! sides drafted 5° about the floor, pulling along +z.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{draft, e, region, rounded_rect, rounded_rect_sides};

pub fn build() -> Result<Solid, String> {
    let outline = rounded_rect((0., 0.), (30., 20.), 4.);
    let b = extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 10.).map_err(e)?;
    draft(&b, &rounded_rect_sides((0., 0.), (30., 20.), 4., 5.), [0., 0., 0.], [0., 0., 1.], 5.)
}
