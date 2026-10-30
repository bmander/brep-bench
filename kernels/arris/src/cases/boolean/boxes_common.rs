//! The part common to two blocks 10 high, spanning x and y from 0 to 10 and 5 to 15.
use arris::math::{Control, Point3};
use arris::ops::{common, primitive_box, OpError};
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
    let (shared, _) = common(m, first, second, &Control::NONE)?;
    Ok(shared)
}
