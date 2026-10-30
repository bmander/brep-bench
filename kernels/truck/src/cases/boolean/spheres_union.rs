//! "Two balls of radius 5, one centred on the origin and one at (6, 0, 0). Make their union."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const RADIUS: f64 = 5.0;
const SECOND_CENTRE: f64 = 6.0; // on the x axis

// workaround: no sphere primitive
pub fn build() -> Built {
    let first = shapes::ball(Point3::origin(), RADIUS);
    let second = shapes::ball(Point3::new(SECOND_CENTRE, 0.0, 0.0), RADIUS);
    let tolerance = shapes::boolean_tolerance(&first, &second);
    let union = truck_shapeops::or(&first, &second, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `or`"))?;
    Ok(union.into())
}
