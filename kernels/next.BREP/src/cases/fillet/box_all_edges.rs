//! A cube of side 10 between the origin and (10, 10, 10), all twelve of its edges rounded to
//! radius 1 in one fillet, which rounds each corner where three rounds meet. Each edge is
//! selected by its middle.
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
    Ok(fillet_edges(&cube, &middles, None, 1.0, false, None)?)
}
