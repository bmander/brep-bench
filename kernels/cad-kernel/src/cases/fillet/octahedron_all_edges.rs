//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges rounded to radius 2, four rounds meeting
//! at each corner, which the ball touching all four faces there closes with a patch of its sphere:
//! the octahedron shrunk by 2 and grown by 2.
//!
//! Built as that composition, the case's own definition, by cad-kernel's offsets. Rounding the
//! edges one at a time with `fillet::roll` is refused by the kernel: the first edge is rolled and
//! taken off (its run cut off at each end by the corner's other two faces), but the second, which
//! shares a corner with it, is not — "the fillet of `face 2` with `face 3` runs out onto `face 8`
//! (bspline), which cuts nothing off yet": a run ends only on faces that cut it off, planes and the
//! solids of cylinders, cones and spheres, and the first round is a fitted canal. Nor could the
//! corner be closed by the kernel once the runs were rolled: `fillet::corner_among`, the ball's
//! patch where rounds meet, stands between three faces, and four meet here.
use cad_kernel::build::hull;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::topo::Brep;

const REACH: f64 = 10.;
const ROUND: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // the octahedron: the convex hull of its six corners, REACH out along each axis either way
    let (r, o) = (REACH, 0.);
    let corners = [[r, o, o], [-r, o, o], [o, r, o], [o, -r, o], [o, o, r], [o, o, -r]];
    let solid = hull(&corners)?;
    // the union of every ball of radius ROUND inside it: shrunk by ROUND, then grown by it
    grown(&shrunk(&solid, ROUND, Join::Round)?, ROUND, Join::Round)
}
