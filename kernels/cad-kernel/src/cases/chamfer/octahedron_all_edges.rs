//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces, four
//! chamfers meeting at each corner, the corner cut flat by a square (the plane x = 6 at the corner
//! (10, 0, 0)).
//!
//! Built round the kernel's lack of a flat corner, as `chamfer/cube-all-edges` is:
//! `fillet::flat_along` chamfers each edge, but chamfers taken off one after another mitre at each
//! corner, the four meeting in a point (`chamfer/octahedron-all-edges-mitered`), and nothing in
//! `fillet`, `rounds`, `corners` or a recipe's lowered corner places the square that chamfering
//! the four together leaves. So the twelve edges are chamfered in turn and each corner then cut
//! off by the plane of its square, as the case states it: square to the corner's axis through the
//! four nearest corners of the faces left, such as (6, 2, 2) — 4 in from the corner.
//!
//! `flat_along` measures each leg from the other face's plane, not across its own face: the line
//! √6 across the face x + y + z = 10 from its edge (z = 2) stands 4/√3 off the neighbouring face
//! x + y − z = 10, so each leg is 4/√3.
use crate::sketch::named;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{hull, kept};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const REACH: f64 = 10.;
/// How far in from its corner each corner's square stands, along the corner's axis.
const CORNER_CUT: f64 = 4.;

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
    let mut solid = hull(&corners)?;
    // each edge picked by its middle, halfway between two corners on the axes; each leg the
    // distance of the line z = 2 on one face from the other face's plane
    let leg = 4. / 3f64.sqrt();
    let h = REACH / 2.;
    let mut edges = Vec::new();
    for (i, j) in [(0, 1), (1, 2), (2, 0)] {
        for si in [-1., 1.] {
            for sj in [-1., 1.] {
                let mut at = [0.; 3];
                (at[i], at[j]) = (si * h, sj * h);
                edges.push(at);
            }
        }
    }
    for at in edges {
        solid = named(solid);
        let flat = flat_along(&solid, at, [leg, leg], None, tolerance(solid.size()))?;
        solid = combined(&solid, &flat, Op::Cut, 0.)?;
    }
    // each corner cut off by the plane square to its axis CORNER_CUT in from it, kept on the
    // octahedron's side
    for c in corners {
        let inward = c.map(|x| -x / REACH);
        let at = c.map(|x| x * (REACH - CORNER_CUT) / REACH);
        solid = kept(&solid, at, inward, 0.)?;
    }
    Ok(solid)
}
