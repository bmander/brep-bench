//! An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x
//! and 6 along y), the ellipse where its side meets its top chamfered by 2 on both faces: 2 in from
//! the rim across the top, square to it, and 2 down the side. The chamfer is the ruled surface of
//! constant slope 45° between the ellipse's inner parallel curve at distance 2 on the top and the
//! ellipse at z = 6 on the side.
//!
//! Refused, by the kernel: "`side` (extrusion) has no offset read yet, so no ball rolls on it:
//! planes, cylinders, cones, spheres, tori and B-spline sheets are rounded". cad-kernel's chamfer
//! of a solid's edge, `fillet::flat_along`, reads each leg where one face meets the other face's
//! surface offset by the leg, and a profile has no ellipse, so the side is the rational B-spline
//! ellipse swept up by `prism`: an extrusion, whose offset the kernel does not read. Made a
//! B-spline sheet instead (the ellipse's net carried up, sewn to its ends) it is refused all the
//! same, as "the flat of `side` with `` is wider than a face holds": `flat_along`'s leg is read by
//! `Surface::offset`, which has an offset in closed form for a plane, a cylinder, a cone, a sphere
//! and a torus only, none for a sheet. Nor is there another road: `build::drafted` (a 45° draft
//! from z = 6 would be this chamfer) and `build::offset` refuse a spline, and the solid a 90° cone
//! sweeps with its tip riding the ellipse at z = 6 (`swept::closed::cut_from` along
//! `trajectory::Along`) is refused by the sweep (`Unbuilt::Curved`, at a point of the cone).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const A: f64 = 10.;
const B: f64 = 6.;
const HEIGHT: f64 = 8.;
const DISTANCE: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    // a profile has no ellipse: the rim as a closed rational quadratic B-spline, four quarters each
    // on the corner of the box round it, weighted √2/2
    let at = |x: f64, y: f64| [A * x, B * y, 0.];
    let poles = vec![at(1., 0.), at(1., 1.), at(0., 1.), at(-1., 1.), at(-1., 0.), at(-1., -1.),
                     at(0., -1.), at(1., -1.), at(1., 0.)];
    let h = FRAC_1_SQRT_2;
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    let rim = BSpline::rational(2, knots, poles, vec![1., h, 1., h, 1., h, 1., h, 1.])?;
    let floor = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                          loops: vec![vec![ProfileEdge::Spline(Arc::new(rim))]],
                          names: vec![vec!["side".into()]] };
    let cylinder = prism(&floor, 0., HEIGHT)?;
    // the top rim picked by a point on it; the kernel would cut the flat between the leg 2 in
    // across the top (where it stands 2 from the side's surface: the parallel curve) and the leg 2
    // down the side (2 from the top's plane)
    let piece = flat_along(&cylinder, [0., B, HEIGHT], [DISTANCE, DISTANCE], None,
                           tolerance(cylinder.size()))?;
    // the rim is convex: its piece is taken away
    Ok(combined(&cylinder, &piece, Op::Cut, 0.)?)
}
