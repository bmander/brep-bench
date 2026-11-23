//! An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1 (semi-axes 10 along
//! x and 6 along y), the ellipse where its side meets its top rounded to radius 2: the pipe a ball
//! of radius 2 sweeps rolling round the inside of the rim, touching the top and the side.
//!
//! A prism of the ellipse (a rational spline in a profile) has a swept side
//! (`Surface::Extrusion`), on which no ball rolls (`Surface::has_parallel`), so the side is lofted
//! as one closed rational B-spline sheet, the ellipse exactly, and `fillet::roll` rolls the ball on
//! the sheet. But it builds a strip of a sheet only for a run that ends: a sheet has no period
//! (`Surface::periods`), so the loop all round it is refused ("loop 0 turns the wrong way"), and a
//! run on a quarter sheet, going on tangent into the next, is cut off at its section where the
//! Boolean is refused ("a fillet's run cannot be cut off where it ends"). So the part is halved
//! along the ellipse's major axis, where the flat each half ends on is square to the rim, each
//! half's rim rolled and taken off, and the halves united again.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{kept, lofted};
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance_over, V};
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const A: f64 = 10.;
const B: f64 = 6.;
const HEIGHT: f64 = 8.;
const ROUND: f64 = 2.;

/// The tolerance (mm) the Booleans with a rolled round are made to at least: at the part's own
/// (1e-8 mm) the search for where the round's end arc, cut off at the halves' flat, touches the
/// sheet it is tangent to reads more boxes than any search may.
pub const FLOOR: f64 = 1e-5;

/// The cylinder on the ellipse of semi-axes `a` along x and `b` along y about the z axis, from
/// z = `z[0]` to `z[1]`: lofted up z on one closed rational B-spline sheet, four rational quadratic
/// quarters each on the corner of the box round it, weighted √2/2, carried straight up. The sheet's
/// seam stands at (0, −`s` b), clear of the half y `s` ≥ 0 the part is rounded in.
pub fn cylinder(a: f64, b: f64, z: [f64; 2], s: f64) -> Result<Brep, String> {
    let at = |x: f64, y: f64| vec![[s * a * x, s * b * y, z[0]], [s * a * x, s * b * y, z[1]]];
    let h = FRAC_1_SQRT_2;
    let round = [[0., -1.], [1., -1.], [1., 0.], [1., 1.], [0., 1.], [-1., 1.], [-1., 0.],
                 [-1., -1.], [0., -1.]];
    let side = Net {
        du: 2,
        dv: 1,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 1., 1.],
        poles: round.iter().map(|&[x, y]| at(x, y)).collect(),
        weights: Some([1., h, 1., h, 1., h, 1., h, 1.].iter().map(|&w| vec![w, w]).collect()),
    };
    lofted([0., 0., 0.], [0., 0., 1.], z, vec![vec![side]])
}

/// The part `part(s)` builds, its edge through `at(s)` rounded to `r` in the half of it on the
/// side `s` `across` ≥ 0 of the plane through the origin square to `across`, for both halves,
/// united: each half's run ends on that plane's flat, where its ball's section is cut off, the
/// round taken off a convex edge and filled in at a concave one. Each round's canal is fitted
/// within `fit` (mm) at least, the half's own tolerance where that is more.
pub fn rounded_by_halves(part: &dyn Fn(f64) -> Result<Brep, String>, across: V,
                         at: &dyn Fn(f64) -> V, r: f64, fit: f64) -> Result<Brep, String> {
    let mut whole: Option<Brep> = None;
    for s in [1., -1.] {
        let half = kept(&part(s)?, [0., 0., 0.], across.map(|x| s * x), 0.)?;
        let round = roll(&half, at(s), r, tolerance_over(half.size(), fit))?;
        let op = if round.concave { Op::Union } else { Op::Cut };
        let half = combined(&half, &round.piece, op, FLOOR)?;
        whole = Some(match whole {
            None => half,
            Some(w) => combined(&w, &half, Op::Union, FLOOR)?,
        });
    }
    Ok(whole.expect("two halves"))
}

pub fn build() -> Result<Brep, String> {
    // halved across y; each half's rim picked by its point on the minor axis
    let part = |s: f64| cylinder(A, B, [0., HEIGHT], s);
    rounded_by_halves(&part, [0., 1., 0.], &|s| [0., s * B, HEIGHT], ROUND, 0.)
}
