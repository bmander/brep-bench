//! The region over the square from (0, 0) to (10, 10) up to the bilinear Coons patch of the skew
//! quadrilateral (0, 0, 5), (10, 0, 5), (10, 10, 15), (0, 10, 5): the saddle z = 5 + xy/10.
use anvil_brep::Solid;
use anvil_brep::loft::{Section, loft_ruled};

use crate::shared::{e, frame, polygon, v3};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no surface fill. The bilinear patch of four lines is also the ruled surface
    // between two opposite edges, so the solid is the ruled loft between its faces x = 0 and
    // x = 10: the rectangle 5 high and the trapezoid rising from 5 to 15, each drawn as (y, z)
    let side = |x: f64| frame([x, 0., 0.], [1., 0., 0.], [0., 1., 0.]);
    let near = Section::Loop { frame: side(0.)?, outline: polygon(&[(0., 0.), (10., 0.), (10., 5.), (0., 5.)]) };
    let far = Section::Loop { frame: side(10.)?, outline: polygon(&[(0., 0.), (10., 0.), (10., 15.), (0., 5.)]) };
    loft_ruled(&near, &far, [Some(v3(0., 0., 0.)), Some(v3(10., 0., 0.))]).map_err(e)
}
