//! The plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
//! x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y), the hole's rim on the top face chamfered
//! by 1 on both faces: 1 out from the rim across the top, square to it, and 1 down the wall. The
//! chamfer is a countersink of constant slope 45° between the ellipse at z = 3 on the wall and the
//! ellipse's outer parallel curve at distance 1 on the top; the rim on the bottom stays sharp.
//!
//! Refused, by the kernel: "`wall` (extrusion) has no offset read yet, so no ball rolls on it:
//! planes, cylinders, cones, spheres, tori and B-spline sheets are rounded". cad-kernel's chamfer
//! of a solid's edge, `fillet::flat_along`, reads each leg where one face meets the other face's
//! surface offset by the leg, and a profile has no ellipse, so the drill is the rational B-spline
//! ellipse swept by `prism` and the hole's wall an extrusion, whose offset the kernel does not
//! read. Drilled by a B-spline sheet instead (the ellipse's net carried up, sewn to its ends) it is
//! refused all the same, as "the flat of `near` with `wall` is wider than a face holds":
//! `flat_along`'s leg is read by `Surface::offset`, which has an offset in closed form for a plane,
//! a cylinder, a cone, a sphere and a torus only, none for a sheet. `build::drafted` and
//! `build::offset`, which would make the countersink as a drill drafted 45°, refuse a spline.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

const A: f64 = 6.;
const B: f64 = 3.;
const THICKNESS: f64 = 4.;
const DISTANCE: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-15., -10., 0.], [15., 10., THICKNESS])?;
    // a profile has no ellipse: the hole's outline as a closed rational quadratic B-spline, four
    // quarters each on the corner of the box round it, weighted √2/2, at z = −1
    let at = |x: f64, y: f64| [A * x, B * y, -1.];
    let poles = vec![at(1., 0.), at(1., 1.), at(0., 1.), at(-1., 1.), at(-1., 0.), at(-1., -1.),
                     at(0., -1.), at(1., -1.), at(1., 0.)];
    let h = FRAC_1_SQRT_2;
    let knots = vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.];
    let outline = BSpline::rational(2, knots, poles, vec![1., h, 1., h, 1., h, 1., h, 1.])?;
    let section = Profile { origin: [0., 0., -1.], normal: [0., 0., 1.],
                            loops: vec![vec![ProfileEdge::Spline(Arc::new(outline))]],
                            names: vec![vec!["wall".into()]] };
    // the drill runs past both faces, from z = −1 to z = 5
    let drill = prism(&section, 0., THICKNESS + 2.)?;
    let drilled = combined(&plate, &drill, Op::Cut, 0.)?;
    // the top rim picked by a point on it; the kernel would cut the flat between the leg 1 out
    // across the top (where it stands 1 from the wall's surface: the parallel curve) and the leg 1
    // down the wall (1 from the top's plane)
    let piece = flat_along(&drilled, [0., B, THICKNESS], [DISTANCE, DISTANCE], None,
                           tolerance(drilled.size()))?;
    // the rim is convex (the plate's material lies in the wedge between the top and the wall): its
    // piece is taken away
    Ok(combined(&drilled, &piece, Op::Cut, 0.)?)
}
