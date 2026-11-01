//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve of its edges chamfered, distance √6 on both
//! faces of each, one edge after another, so the four chamfers at each corner meet in a point.
use super::cube_all_edges_mitred::{edges, left_of};
use super::octahedron_all_edges::octahedron;
use vcad_kernel::vcad_kernel_fillet::{BlendKey, BlendSection};
use vcad_kernel::Solid;

const CHAMFER: f64 = 2.449489742783178; // √6

pub fn build() -> Result<Solid, crate::Refusal> {
    let mut solid = octahedron()?;
    // shape 0 is a flat chamfer, its size the distance on both faces
    let chamfer = BlendKey { t: 0.0, section: BlendSection { size: CHAMFER, shape: 0.0 } };
    // each of the octahedron's edges its own chamfer, found again in the solid the ones before it
    // left. `edge_blend` declines the first: its loft ends the chamfer in the section square to
    // the edge and splices that into the faces at the edge's ends, and an octahedron's faces there
    // are not square to it, so they come out bent and the shell cracked (at any size).
    for edge in edges(&solid) {
        solid = solid.edge_blend(&left_of(&solid, edge)?, &[chamfer])?;
    }
    Ok(solid)
}
