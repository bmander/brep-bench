//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 from its outside.
use arris::math::{Control, Point3};
use arris::ops::{offset_faces, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIZE: (f64, f64, f64) = (10.0, 8.0, 6.0);
const SHRINKAGE: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIZE.0, SIZE.1, SIZE.2), &Control::NONE)?;
    // every face moved in along its normal; a box's edges are all convex, so shrunk they stay
    // sharp, the moved faces meeting, as `offset_faces` joins them
    let faces = m.faces(block)?;
    let (shrunk, _) = offset_faces(m, block, &faces, -SHRINKAGE, &Control::NONE)?;
    Ok(shrunk)
}
