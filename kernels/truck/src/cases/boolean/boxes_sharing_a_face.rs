//! "Two cubes of side 10, side by side: the first between the origin and (10, 10, 10), the second
//! between (10, 0, 0) and (20, 10, 10). Join them into one solid. They touch along a whole face,
//! which must disappear from the result."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

pub fn build() -> Built {
    let first: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 10.0, 10.0),
    ]));
    let second: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(10.0, 0.0, 0.0),
        Point3::new(20.0, 10.0, 10.0),
    ]));
    let tolerance = shapes::boolean_tolerance(&first, &second);
    let joined = truck_shapeops::or(&first, &second, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `or`"))?;
    Ok(joined.into())
}
