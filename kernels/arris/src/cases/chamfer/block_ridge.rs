//! A block from the origin to (10, 4, 6), the two edges of its top running along x chamfered 2 on
//! both faces, half the top's width, so the top becomes a ridge.
use arris::math::{Control, Point3};
use arris::ops::{chamfer, primitive_box, OpError};
use arris::topo::provenance::{BoxPart, Coord, Role, Side};
use arris::topo::{Body, Edge, Model};

const LENGTH: f64 = 10.0;
const WIDTH: f64 = 4.0;
const HEIGHT: f64 = 6.0;
const DISTANCE: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, made) =
        primitive_box(m, Point3::origin(), Point3::new(LENGTH, WIDTH, HEIGHT), &Control::NONE)?;
    // the top's two edges along x, at y = 0 and y = 4, named by the box's provenance
    let edges: Vec<Edge> = [Side::Min, Side::Max]
        .into_iter()
        .flat_map(|y| {
            let along = Role::Box(BoxPart::Edge { along: Coord::X, sides: [y, Side::Max] });
            made.generated_from(along).to_vec()
        })
        .filter_map(|s| Edge::try_from(s).ok())
        .collect();
    let (ridged, _) = chamfer(m, block, &edges, DISTANCE, &Control::NONE)?;
    Ok(ridged)
}
