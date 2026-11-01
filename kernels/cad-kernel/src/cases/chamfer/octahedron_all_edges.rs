//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 across both faces, four
//! chamfers meeting at each corner, each corner cut flat by a square.
//!
//! Refused. cad-kernel's chamfer, `fillet::flat_along`, cuts the run of edges nearest a point,
//! and a run that ends is cut off at each end by the faces there: chamfered one after another the
//! edges mitre (`chamfer/octahedron-all-edges-mitred`). The square closing a corner belongs to
//! chamfering the four edges there together, and nothing in `fillet`, `corners` or a recipe's
//! lowered corner places a flat corner, at three faces or at four.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no chamfer of edges that meet at a vertex closes the corner: fillet::flat_along chamfers \
         one run of edges, cut off at each end by the faces there, so chamfers taken one at a \
         time mitre where four meet, and no operation places the corner's square"
        .into())
}
