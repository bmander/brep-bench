//! A cube of side 10 from the origin with all twelve of its edges chamfered, distance 2 on both
//! faces of each, one edge after another, so the three chamfers at each corner meet in a point.
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection, EdgeQuery};
use vcad_kernel::vcad_kernel_math::Point3;
use vcad_kernel::Solid;

const SIDE: f64 = 10.0;
const CHAMFER: f64 = 2.0;
/// How near a vertex must be to the line of an edge to be on it.
const ON: f64 = 1e-7;

/// The edges of `solid`, each as its two ends, read from its B-rep.
pub fn edges(solid: &Solid) -> Vec<(Point3, Point3)> {
    let Some(brep) = solid.brep() else { return Vec::new() };
    let topology = &brep.topology;
    let ends = |e: &vcad_kernel::vcad_kernel_topo::Edge| {
        let half = &topology.half_edges[e.half_edge];
        let twin = &topology.half_edges[half.twin?];
        Some((topology.vertices[half.origin].point, topology.vertices[twin.origin].point))
    };
    topology.edges.values().filter_map(ends).collect()
}

/// The edge of `solid` that is what the chamfers made so far left of the edge from `a` to `b`: the
/// one with both its ends on that line, picked by them, as `edge_blend` picks an edge.
pub fn left_of(solid: &Solid, (a, b): (Point3, Point3)) -> Result<EdgeQuery, crate::Refusal> {
    let along = (b - a) / (b - a).norm();
    let on = |p: Point3| (p - a).cross(along).norm() < ON;
    let left: Vec<_> = edges(solid).into_iter().filter(|&(p, q)| on(p) && on(q)).collect();
    match left[..] {
        [(a, b)] => Ok(EdgeQuery::Endpoints { a, b }),
        _ => Err(format!(
            "the solid has {} edges on the line of the edge from {a:?} to {b:?}, not one",
            left.len()
        )
        .into()),
    }
}

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut cube = Solid::cube(SIDE, SIDE, SIDE);
    // shape 0 is a flat chamfer, its size the distance on both faces
    let chamfer = BlendKey { t: 0.0, section: BlendSection { size: CHAMFER, shape: 0.0 } };
    // each of the cube's edges its own chamfer, found again in the solid the ones before it left.
    // `edge_blend`'s loft ends a chamfer in the section square to the edge, spliced into the faces
    // at its ends: right on a face of the cube, but not on a chamfer made before it, so the
    // second chamfer, beside the first, already leaves 963.99 where the mitre leaves 962.67, and
    // the third, ending on the first, `edge_blend` declines as a cracked shell or a volume gained.
    for edge in edges(&cube) {
        cube = cube.edge_blend(&left_of(&cube, edge)?, &[chamfer])?;
    }
    Ok(cube)
}
