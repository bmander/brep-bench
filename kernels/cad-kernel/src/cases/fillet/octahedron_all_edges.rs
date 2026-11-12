//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges rounded to radius 2, four rounds meeting
//! at each corner, which the ball touching all four faces there closes with a patch of its sphere:
//! the octahedron shrunk by 2 and grown by 2.
//!
//! Built as that composition, the case's own definition, by cad-kernel's offsets. Rounding the
//! edges one at a time with `fillet::roll` is refused by the kernel: two of the four edges at a
//! corner are rolled and taken off, one after the other, but the third there is not — "the fillet
//! of `face 0` with `face 5` runs on smoothly into `face 8`, but its contact crosses their seam in
//! another section than its edge does (7.07e-1 off it there): no one section joins the run to the
//! piece going on". Nor could the corner be closed by the kernel once the runs were rolled:
//! `fillet::corner_among`, the ball's patch where rounds meet, stands between three faces, and four
//! meet here.
use cad_kernel::build::hull;
use cad_kernel::offset::{grown, shrunk, Join};
use cad_kernel::topo::Brep;

const REACH: f64 = 10.;
const ROUND: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // the octahedron: the convex hull of its six corners, REACH out along each axis
    let mut corners = Vec::new();
    for axis in 0..3 {
        for s in [-1., 1.] {
            let mut at = [0.; 3];
            at[axis] = s * REACH;
            corners.push(at);
        }
    }
    let solid = hull(&corners)?;
    // the union of every ball of radius ROUND inside it: shrunk by ROUND, then grown by it
    grown(&shrunk(&solid, ROUND, Join::Round)?, ROUND, Join::Round)
}
