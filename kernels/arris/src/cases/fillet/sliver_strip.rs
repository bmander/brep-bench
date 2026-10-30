//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 running along x
//! rounded to 0.9999, leaving a strip of that face 0.0002 tall between them.
use arris::math::{Control, Point3};
use arris::ops::{fillet, primitive_box, OpError};
use arris::topo::provenance::{BoxPart, Coord, Role, Side};
use arris::topo::{Body, Edge, Model};

const LENGTH: f64 = 20.0;
const WIDTH: f64 = 10.0;
const THICKNESS: f64 = 2.0;
const ROUND: f64 = 0.9999;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (plate, made) =
        primitive_box(m, Point3::origin(), Point3::new(LENGTH, WIDTH, THICKNESS), &Control::NONE)?;
    // the face y = 10's two edges along x, at z = 0 and z = 2, named by the box's provenance
    let edges: Vec<Edge> = [Side::Min, Side::Max]
        .into_iter()
        .flat_map(|z| {
            let along = Role::Box(BoxPart::Edge { along: Coord::X, sides: [Side::Max, z] });
            made.generated_from(along).to_vec()
        })
        .filter_map(|s| Edge::try_from(s).ok())
        .collect();
    let (rounded, _) = fillet(m, plate, &edges, ROUND, &Control::NONE)?;
    Ok(rounded)
}
