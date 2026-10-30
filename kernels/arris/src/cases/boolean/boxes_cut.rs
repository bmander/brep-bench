//! A block 10 high over x, y in 0..10 with a second, over 5..15, cut away from it.
use arris::math::{Control, Point3};
use arris::ops::{cut, primitive_box, OpError};
use arris::topo::{Body, Model};

const HEIGHT: f64 = 10.0;
const FIRST: (f64, f64) = (0.0, 10.0);
const SECOND: (f64, f64) = (5.0, 15.0);

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let block = |m: &mut Model, (lo, hi): (f64, f64)| {
        primitive_box(m, Point3::new(lo, lo, 0.0), Point3::new(hi, hi, HEIGHT), &Control::NONE)
    };
    let (first, _) = block(m, FIRST)?;
    let (second, _) = block(m, SECOND)?;
    let (notched, _) = cut(m, first, second, &Control::NONE)?;
    Ok(notched)
}
