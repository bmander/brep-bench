//! "A torus about the z axis, centred on the origin, whose tube's centre circle has radius 10 and
//! whose tube has radius 2. Cut away everything where x < 0."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const RING_RADIUS: f64 = 10.0;
const TUBE_RADIUS: f64 = 2.0;

pub fn build() -> Built {
    let torus = shapes::torus(Point3::origin(), RING_RADIUS, TUBE_RADIUS);
    // everything where x < 0, as far as the torus reaches (12 from the axis, 2 from the xy
    // plane): a block from x = -20 to x = 0
    let mut half_space: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(-20.0, -20.0, -5.0),
        Point3::new(0.0, 20.0, 5.0),
    ]));
    // truck-shapeops cuts by intersecting with the tool turned inside out
    half_space.not();
    let tolerance = shapes::boolean_tolerance(&torus, &half_space);
    let half = truck_shapeops::and(&torus, &half_space, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(half.into())
}
