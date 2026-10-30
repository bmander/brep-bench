//! "Two blocks 10 high, both standing on the xy plane. The first spans x and y from 0 to 10; the
//! second spans x and y from 5 to 15. Cut the second away from the first."
use crate::{shapes, Built, Refused};
use truck_modeling::*;

const HEIGHT: f64 = 10.0;
const FIRST: (f64, f64) = (0.0, 10.0); // its span in x and in y
const SECOND: (f64, f64) = (5.0, 15.0);

pub fn build() -> Built {
    let block = |(lo, hi): (f64, f64)| -> Solid {
        primitive::cuboid(BoundingBox::from_iter([
            Point3::new(lo, lo, 0.0),
            Point3::new(hi, hi, HEIGHT),
        ]))
    };
    let (first, mut second) = (block(FIRST), block(SECOND));
    // truck-shapeops cuts by intersecting with the tool turned inside out
    second.not();
    let tolerance = shapes::boolean_tolerance(&first, &second);
    let cut = truck_shapeops::and(&first, &second, tolerance)
        .ok_or_else(|| Refused::none_from("truck-shapeops `and`"))?;
    Ok(cut.into())
}
