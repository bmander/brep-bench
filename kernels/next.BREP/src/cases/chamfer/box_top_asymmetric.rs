//! A box from the origin to (20, 10, 6), the four edges around its top face z = 6 chamfered
//! unequally, 1 across the top and 2 down the side, each edge its own two-distance chamfer, so the
//! solid is the box cut by the four chamfers' planes, two neighbours meeting along a slanting line
//! such as from (20, 10, 4) to (19, 9, 6).
use crate::solids::fillet::{faces_of_edge_through, lies_in_plane_z};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_box_brep, BrepSolid};

const ACROSS_TOP: f64 = 1.0;
const DOWN_SIDE: f64 = 2.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut solid = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 10.0, 6.0)?;
    // Each edge round the top, by its middle.
    for middle in [p(10.0, 0.0, 6.0), p(20.0, 5.0, 6.0), p(10.0, 10.0, 6.0), p(0.0, 5.0, 6.0)] {
        // The kernel's two-distance chamfer sets back its first distance on the edge's first
        // face, in the order of the solid's shells: across the top where that is the top.
        let faces = faces_of_edge_through(&solid, middle);
        let top_first = match faces.first() {
            Some(face) => lies_in_plane_z(face, 6.0),
            None => return Err(Refusal::from(format!("no edge through {middle:?}"))),
        };
        let (d1, d2) = if top_first { (ACROSS_TOP, DOWN_SIDE) } else { (DOWN_SIDE, ACROSS_TOP) };
        solid = chamfer_edges_asymmetric(&solid, &[middle], None, d1, d2, None)?;
    }
    Ok(solid)
}
