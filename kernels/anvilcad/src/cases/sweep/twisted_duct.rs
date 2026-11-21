//! A duct along y from the square A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10), D (10, 0, 10)
//! at y = 0 to the diamond A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12), D′ (12, 40, 0), each
//! corner running straight to its primed one.
use anvil_brep::Solid;
use anvil_brep::loft::{Section, loft_ruled};

use crate::shared::{e, frame, polygon, v3};

pub fn build() -> Result<Solid, String> {
    // each end in its plane, sketch x along world x and sketch y along world −z
    let square = Section::Loop { frame: frame([0., 0., 0.], [0., 1., 0.], [1., 0., 0.])?, outline: polygon(&[(10., 10.), (-10., 10.), (-10., -10.), (10., -10.)]) };
    let diamond = Section::Loop { frame: frame([0., 40., 0.], [0., 1., 0.], [1., 0., 0.])?, outline: polygon(&[(0., 12.), (-12., 0.), (0., -12.), (12., 0.)]) };
    // the loft starts each loop at its pick: A on the square, A′ on the diamond
    loft_ruled(&square, &diamond, [Some(v3(10., 0., -10.)), Some(v3(0., 40., -12.))]).map_err(e)
}
