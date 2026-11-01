//! A cube of side 10 between the origin and (10, 10, 10), all twelve of its edges chamfered with
//! distance 2 on both faces in one chamfer, each corner where three meet cut flat by a triangle:
//! the kernel's chamfer of a selection stops the three chamfers at a corner where the corner's
//! ball touches them and closes the corner with the facet their end chords bound, at (10, 10, 10)
//! the triangle through (8, 8, 10), (8, 10, 8) and (10, 8, 8).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    // The middle of each edge: one coordinate 5, the other two each 0 or 10.
    let mut middles: Vec<Vec3> = Vec::new();
    for u in [0.0, 10.0] {
        for v in [0.0, 10.0] {
            middles.extend([p(5.0, u, v), p(u, 5.0, v), p(u, v, 5.0)]);
        }
    }
    // A right-angled edge: the chamfer's distance is its setback on both faces.
    Ok(fillet_edges(&cube, &middles, None, 2.0, true, None)?)
}
