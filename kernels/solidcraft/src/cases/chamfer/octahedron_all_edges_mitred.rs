use super::octahedron_all_edges::{middles, octahedron};
use crate::solids::*;

pub fn build() -> R<Body> {
    // Each edge its own chamfer, ending on the faces at its ends, the earlier chamfers among
    // them, so the four at a corner mitre. An edge's middle stays on what is left of it: a chamfer
    // of a neighbouring edge cuts it only within 2 of its corner (the chamfer x + z = 8 meets the
    // edge from (10, 0, 0) to (0, 10, 0) at (8, 2, 0)), and the same off either end.
    let mut solid = octahedron()?;
    for edge in middles() {
        solid = solidcraft_kernel::chamfer(&solid, &[edge], 6f64.sqrt())?;
    }
    Ok(solid)
}
