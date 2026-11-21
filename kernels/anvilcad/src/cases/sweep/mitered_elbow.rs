//! A 10 × 10 square bar swept from the origin along +x to (40, 0, 0), then 120° to the left on to
//! (20, 20√3, 0), mitered at the corner in the plane x + √3 y = 40.
use anvil_brep::Solid;

use crate::shared::{beyond, frame, rect, region, subtract, union, e};
use anvil_brep::extrude;

pub fn build() -> Result<Solid, String> {
    // anvil-brep's sweep turns no sharp corner: each run is its square swept straight along it,
    // run past the corner, and cut by the miter plane, the two pieces joined there
    let s3 = 3f64.sqrt();
    let square = region(&[rect((-5., -5.), (5., 5.))])?;
    let first = extrude::extrude(&frame([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?, &square, 0., 60.).map_err(e)?;
    let second = extrude::extrude(&frame([20., 20. * s3, 0.], [0.5, -s3 / 2., 0.], [0., 0., 1.])?, &square, 0., 60.).map_err(e)?;
    // the miter: the plane x + √3 y = 40 through the corner, its normal (1, √3, 0)/2
    let a = subtract(&first, &beyond([40., 0., 0.], [0.5, s3 / 2., 0.], 100.)?)?;
    let b = subtract(&second, &beyond([40., 0., 0.], [-0.5, -s3 / 2., 0.], 100.)?)?;
    union(&a, &b)
}
