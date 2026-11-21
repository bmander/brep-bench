//! A plate from the origin to (40, 30, 5) with a square boss from (12, 7, 5) to (28, 23, 15): the
//! boss's upright edges rounded to radius 3, then its root, now one tangent chain, to radius 2.
use std::f64::consts::FRAC_1_SQRT_2;

use anvil_brep::Solid;

use crate::shared::{block, fillet, union};

pub fn build() -> Result<Solid, String> {
    let body = union(&block([0., 0., 0.], [40., 30., 5.])?, &block([12., 7., 5.], [28., 23., 15.])?)?;
    let uprights = fillet(&body, &[[12., 7., 10.], [28., 7., 10.], [28., 23., 10.], [12., 23., 10.]], 3.)?;
    // the root: four sides and the four quarter circles of radius 3 round the uprights' axes
    let d = 3. * FRAC_1_SQRT_2;
    let root = [
        [20., 7., 5.], [28., 15., 5.], [20., 23., 5.], [12., 15., 5.],
        [15. - d, 10. - d, 5.], [25. + d, 10. - d, 5.], [25. + d, 20. + d, 5.], [15. - d, 20. + d, 5.],
    ];
    fillet(&uprights, &root, 2.)
}
