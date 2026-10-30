//! A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin.
use arris::math::{Control, Point3};
use arris::ops::{primitive_box, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 10.0;
const WIDTH: f64 = 20.0;
const HEIGHT: f64 = 30.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, _) =
        primitive_box(m, Point3::origin(), Point3::new(LENGTH, WIDTH, HEIGHT), &Control::NONE)?;
    Ok(block)
}
