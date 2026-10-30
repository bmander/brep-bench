//! A cube of side 10 at the origin with all twelve edges chamfered 2 on both faces, three chamfers
//! meeting at each corner, the corner cut flat by a triangle.
//!
//! Refused. What cad-kernel offers, and why none of it builds this:
//!
//! - `fillet::flat_along(&cube, at, legs, on_a, tol)` chamfers the run of edges nearest `at`, the
//!   kernel placing the flat and building the piece it takes off; a run that ends is cut off at
//!   each end by the faces there. On the plain cube it chamfers the four vertical edges, but each
//!   top or bottom edge then ends on a vertical one's flat, which it fits as a B-spline face, and
//!   it refuses: "runs out onto `flat` (bspline), which cuts nothing off yet".
//! - Even past that, chamfers taken off one after another mitre where three meet (the three
//!   planes x + y = 18, y + z = 18, x + z = 18 meet at the point (9, 9, 9)): the corner's
//!   triangle in the plane x + y + z = 26 belongs to chamfering the three edges together, and
//!   nothing in `fillet`, `corners` or a recipe's lowered corner places a flat corner.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no chamfer of edges that meet at a vertex: fillet::flat_along chamfers the cube's four \
         vertical edges but refuses its top and bottom ones (\"runs out onto `flat` (bspline), \
         which cuts nothing off yet\"), and chamfers taken off one at a time would mitre at each \
         corner, where the triangle a chamfer of all three edges leaves has no operation that \
         places it"
        .into())
}
