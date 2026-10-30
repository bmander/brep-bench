//! An L-block, its section in the xz plane extruded 10 along +y, its inside corner x = 3, z = 3
//! chamfered 7 on both faces, so the chamfer consumes both faces beside it.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{chamfer, extrude, OpError};
use arris::topo::provenance::{Role, SweepPart};
use arris::topo::{Body, Edge, Model};

const OUTLINE: [(f64, f64); 6] =
    [(0.0, 0.0), (10.0, 0.0), (10.0, 3.0), (3.0, 3.0), (3.0, 10.0), (0.0, 10.0)];
/// The outline's inside corner, (3, 3): the vertex its fourth segment starts at.
const INSIDE_CORNER: usize = 3;
const DEPTH: f64 = 10.0;
const DISTANCE: f64 = 7.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane, u along x and v along z, extruded along +y
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let corner = |(u, v): (f64, f64)| Point2::new(u, v);
    let segments = OUTLINE[1..]
        .iter()
        .chain(&OUTLINE[..1])
        .map(|&p| ProfileSegment::LineTo(corner(p)))
        .collect();
    let section = Profile {
        plane: xz,
        outer: ProfileLoop::Path { start: corner(OUTLINE[0]), segments },
        holes: vec![],
    };
    let (block, made) = extrude(m, &section, Vec3::y(), DEPTH, &Control::NONE)?;
    // the concave edge the inside corner sweeps, named by the extrude's provenance
    let inside = Role::Extrude(SweepPart::Rise { loop_index: 0, vertex: INSIDE_CORNER });
    let edges: Vec<Edge> =
        made.generated_from(inside).iter().filter_map(|&s| Edge::try_from(s).ok()).collect();
    let (filled, _) = chamfer(m, block, &edges, DISTANCE, &Control::NONE)?;
    Ok(filled)
}
