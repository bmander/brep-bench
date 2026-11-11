//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges chamfered √6 on both faces (the
//! chamfer of the edge from (10, 0, 0) to (0, 10, 0) the plane x + y = 8), one after another: each
//! chamfer runs on until it meets the ones before, the four at each corner meeting in a point.
//!
//! `flat_along` measures each leg from the other face's plane, not across its own face: the line
//! √6 across the face x + y + z = 10 from its edge (z = 2) stands 4/√3 off the neighbouring face
//! x + y − z = 10, so each leg is 4/√3.
use crate::sketch::named;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::hull;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;

const REACH: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // the octahedron: the convex hull of its six corners, REACH out along each axis either way
    let (r, o) = (REACH, 0.);
    let corners = [[r, o, o], [-r, o, o], [o, r, o], [o, -r, o], [o, o, r], [o, o, -r]];
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
    Ok(solid)
}
