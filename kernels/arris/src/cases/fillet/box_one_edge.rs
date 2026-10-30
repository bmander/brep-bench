//! A cube of side 10 with the edge where its faces x = 10 and y = 10 meet rounded to 2.
use arris::math::{Control, Point3};
use arris::ops::{fillet, primitive_box, OpError};
use arris::topo::provenance::{BoxPart, Coord, Role, Side};
use arris::topo::{Body, Edge, Model};

const SIDE: f64 = 10.0;
const ROUND: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cube, made) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    // the edge running along z on the box's x and y maximum sides, named by the box's provenance
    let corner = Role::Box(BoxPart::Edge { along: Coord::Z, sides: [Side::Max, Side::Max] });
    let edges: Vec<Edge> =
        made.generated_from(corner).iter().filter_map(|&s| Edge::try_from(s).ok()).collect();
    let (rounded, _) = fillet(m, cube, &edges, ROUND, &Control::NONE)?;
    Ok(rounded)
}
