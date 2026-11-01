//! A cube of side 10 with all twelve of its edges chamfered 2 on both faces, one edge after
//! another, so the three chamfers at each corner run on and meet in a point, the corner mitred.
use arris::math::{Control, Point3};
use arris::ops::{chamfer, primitive_box, OpError};
use arris::topo::provenance::{BoxPart, Coord, Role, Side};
use arris::topo::{Body, Edge, Model};

const SIDE: f64 = 10.0;
const DISTANCE: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (mut cube, mut history) =
        primitive_box(m, Point3::origin(), Point3::new(SIDE, SIDE, SIDE), &Control::NONE)?;
    let sides = [Side::Min, Side::Max];
    // Each edge its own chamfer: a chamfer made alone ends on the faces at its ends, the chamfers
    // made before it among them, so the three at a corner mitre. (All twelve in one `chamfer`
    // close each corner with a triangle: chamfer/cube-all-edges.) arris 0.9.1 chamfers the four
    // edges along x, no two sharing a corner, and declines the fifth, the first to end on an
    // earlier chamfer: at the same distance its contact on the face they share runs exactly to
    // the far end of that chamfer's end segment ("the blend leaves its face through an edge that
    // is not the corner's own"), where a narrower one it would take. In any order, some edge is
    // the second at a corner.
    for along in [Coord::X, Coord::Y, Coord::Z] {
        for a in sides {
            for b in sides {
                // what is left of the box's edge, named by its provenance carried through the
                // chamfers made so far, each of which shortened it or left it alone
                let role = Role::Box(BoxPart::Edge { along, sides: [a, b] });
                let edge: Vec<Edge> = history
                    .generated_from(role)
                    .iter()
                    .filter_map(|&s| Edge::try_from(s).ok())
                    .collect();
                let (chamfered, made) = chamfer(m, cube, &edge, DISTANCE, &Control::NONE)?;
                cube = chamfered;
                history = history.then(&made);
            }
        }
    }
    Ok(cube)
}
