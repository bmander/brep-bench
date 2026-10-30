//! "A rectangular block 10 long in x, 20 in y and 30 in z, with one corner at the origin and the
//! opposite corner at (10, 20, 30)."
use truck_modeling::*;

const LENGTH: f64 = 10.0; // in x
const WIDTH: f64 = 20.0; // in y
const HEIGHT: f64 = 30.0; // in z

pub fn build() -> crate::Built {
    let corners = [Point3::origin(), Point3::new(LENGTH, WIDTH, HEIGHT)];
    let block: Solid = primitive::cuboid(BoundingBox::from_iter(corners));
    Ok(block.into())
}
