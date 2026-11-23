//! A cube of side 10 at the origin with all twelve edges chamfered 2 on both faces, three chamfers
//! meeting at each corner, the corner cut flat by a triangle.
//!
//! Built round the kernel's lack of a flat corner: `fillet::flat_along` chamfers each edge (the
//! kernel placing the flat and cutting each run off by the flats before it), but chamfers taken
//! off one after another mitre where three meet, and nothing in `fillet`, `rounds`, `corners` or a
//! recipe's lowered corner places the triangle that chamfering the three together leaves. So the
//! twelve edges are chamfered in turn and each corner then cut off by the plane of its triangle,
//! as the case states it: through the three points of the chamfers' edges nearest the corner, each
//! 2 in from it along two of the cube's edges, such as (8, 8, 10), (8, 10, 8) and (10, 8, 8) at
//! (10, 10, 10).
use crate::sketch::{block, named};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::kept;
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, dot, scale, sub, unit};

const SIDE: f64 = 10.;
const LEG: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let mut cube = block([0., 0., 0.], [SIDE; 3])?;
    // each edge picked by its middle: the four along x, then y, then z
    let h = SIDE / 2.;
    let mut edges = Vec::new();
    for axis in 0..3 {
        for a in [0., SIDE] {
            for b in [0., SIDE] {
                let mut at = [0.; 3];
                (at[axis], at[(axis + 1) % 3], at[(axis + 2) % 3]) = (h, a, b);
                edges.push(at);
            }
        }
    }
    for at in edges {
        cube = named(cube);
        let flat = flat_along(&cube, at, [LEG, LEG], None, tolerance(cube.size()))?;
        cube = combined(&cube, &flat, Op::Cut, 0.)?;
    }
    // each corner cut off by the plane through the three points LEG in from it along two of its
    // edges, kept on the cube's side
    let middle = [h; 3];
    for k in 0..8 {
        let corner = [0, 1, 2].map(|i| if k >> i & 1 == 1 { SIDE } else { 0. });
        let inward = sub(middle, corner).map(|x| x.signum());
        let point = |skip: usize| {
            add(corner, scale([0, 1, 2].map(|i| if i == skip { 0. } else { inward[i] }), LEG))
        };
        let [p, q, r] = [0, 1, 2].map(point);
        let n = unit(cross(sub(q, p), sub(r, p)));
        let keep = if dot(n, sub(middle, p)) > 0. { n } else { scale(n, -1.) };
        cube = kept(&cube, p, keep, 0.)?;
    }
    Ok(cube)
}
