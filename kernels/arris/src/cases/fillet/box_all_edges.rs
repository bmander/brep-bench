//! A cube of side 10 with all twelve of its edges rounded to radius 1.
use arris::math::{Control, Point3};
use arris::ops::{fillet, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;
const ROUND: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let edges = m.edges(cube)?;
    let (rounded, _) = fillet(m, cube, &edges, ROUND, &Control::NONE)?;
    Ok(rounded)
}
