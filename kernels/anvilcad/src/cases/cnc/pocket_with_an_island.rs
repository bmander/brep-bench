//! A plate from the origin to (160, 110, −20) with a pocket 12 deep round a round island, cleared
//! by raster passes and walls by a flat end mill of radius 5; a bore of radius 9, 6 deep, in the
//! island; and four holes drilled 10 deep with a 90° point at its corners.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{at, block, circle, cone, cut_all, cylinder, e, region, rounded_rect, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. What the mills sweep follows from the program by
    // elementary geometry: the pocket's passes clear the rounded rectangle less the island,
    // exactly, at each depth down to z = −12; the bore's laps, a mill of radius 3 round a circle
    // of radius 6, clear the disk of radius 9 down to z = −6; and each drill, plunged straight,
    // leaves its own shape: a cylinder of radius 4 down to z = −10 and its point, a cone, to −14
    let pocket = region(&[rounded_rect((20., 20.), (140., 90.), 12.), circle((60., 55.), 15.)])?;
    let pocket = extrude::extrude(&Frame::world(), &pocket, -12., 1.).map_err(e)?;
    let bore = cylinder(&at([60., 55., 0.]), 9., -6., 1.);
    let hole = |x: f64, y: f64| -> Result<Solid, String> {
        union(&cylinder(&at([x, y, 0.]), 4., -10., 1.)?, &cone(&at([x, y, -14.]), 0., 4., 4.)?)
    };
    let cuts = [Ok(pocket), bore, hole(8., 8.), hole(152., 8.), hole(152., 102.), hole(8., 102.)];
    cut_all(block([0., 0., -20.], [160., 110., 0.])?, cuts)
}
