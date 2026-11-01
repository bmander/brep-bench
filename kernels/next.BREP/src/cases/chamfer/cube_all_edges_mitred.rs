//! A cube of side 10 between the origin and (10, 10, 10), all twelve of its edges chamfered with
//! distance 2 on both faces, one after another, each edge its own chamfer ending on the faces at
//! its ends (the chamfers made before it among them), so the three at each corner mitre in a
//! point, at (10, 10, 10) the point (9, 9, 9).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut solid = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    // Each edge is selected by its middle (one coordinate 5, the other two each 0 or 10), which
    // the chamfers before it, cutting as much off either end, leave on what is left of it.
    for u in [0.0, 10.0] {
        for v in [0.0, 10.0] {
            for middle in [p(5.0, u, v), p(u, 5.0, v), p(u, v, 5.0)] {
                // A right-angled edge: the chamfer's distance is its setback on both faces.
                solid = fillet_edges(&solid, &[middle], None, 2.0, true, None)?;
            }
        }
    }
    Ok(solid)
}
