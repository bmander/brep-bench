//! "Two cubes of side 10, joined into one solid. The first lies between the origin and
//! (10, 10, 10). The second lies between (5, 0, 10) and (15, 10, 20): it stands on the first, and
//! its bottom covers half of the first's top."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

pub fn build() -> Built {
    let first: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(10.0, 10.0, 10.0),
    ]));
    let second: Solid = primitive::cuboid(BoundingBox::from_iter([
        Point3::new(5.0, 0.0, 10.0),
        Point3::new(15.0, 10.0, 20.0),
    ]));
    let tolerance = shapes::boolean_tolerance(&first, &second);
    let joined = truck_shapeops::or(&first, &second, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `or`"))?;
    Ok(joined.into())
}
