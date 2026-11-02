//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, four
//! chamfers meeting at each corner, the corner cut flat by a square (the plane x = 6 at the corner
//! (10, 0, 0)).
//!
//! Refused, as `chamfer/cube-all-edges` is: `fillet::flat_along` chamfers each edge, and chamfers
//! taken off one after another mitre at each corner, the four meeting in a point
//! (`chamfer/octahedron-all-edges-mitred`). The square closing the corner belongs to chamfering
//! the four edges together, and nothing in `fillet`, `corners` or a recipe's lowered corner places
//! a flat corner. Cutting each corner off by its plane would be the caller placing the corner, not
//! the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no chamfer of edges that meet at a vertex: fillet::flat_along chamfers each edge, but \
         chamfers taken off one at a time mitre at each corner, where the square a chamfer of all \
         four edges leaves has no operation that places it"
        .into())
}
