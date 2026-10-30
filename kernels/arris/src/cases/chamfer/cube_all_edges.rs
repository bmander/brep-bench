//! A cube of side 10 with all twelve of its edges chamfered 2 on both faces, each corner cut flat
//! by a triangle.
use arris::math::{Control, Point3};
use arris::ops::{chamfer, primitive_box, OpError};
use arris::topo::{Body, Model};

const SIDE: f64 = 10.0;
const DISTANCE: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, _) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let edges = m.edges(cube)?;
    let (chamfered, _) = chamfer(m, cube, &edges, DISTANCE, &Control::NONE)?;
    Ok(chamfered)
}
