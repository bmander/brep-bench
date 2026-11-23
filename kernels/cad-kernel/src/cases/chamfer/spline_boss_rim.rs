//! The prism from z = 0 to z = 6 on the periodic uniform cubic B-spline whose eight control points,
//! in order round it, are (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4),
//! a bean dented at the top, the spline where its side meets its top chamfered by 1 on both faces:
//! 1 in from the rim across the top, square to it, and 1 down the side. The chamfer is the ruled
//! surface of constant slope 45° between the spline's inner parallel curve at distance 1 on the top
//! and the spline at z = 5 on the side.
//!
//! Refused, by the kernel: "`side` (extrusion) has no offset read yet, so no ball rolls on it:
//! planes, cylinders, cones, spheres, tori and B-spline sheets are rounded". cad-kernel's chamfer
//! of a solid's edge, `fillet::flat_along`, reads each leg where one face meets the other face's
//! surface offset by the leg, and the side of a `prism` over a spline is an extrusion, whose offset
//! the kernel does not read. Made a B-spline sheet instead (the spline's net carried up, sewn to
//! its ends) it is refused all the same, as "the flat of `side` with `` is wider than a face
//! holds": `flat_along`'s leg is read by `Surface::offset`, which has an offset in closed form for
//! a plane, a cylinder, a cone, a sphere and a torus only, none for a sheet. Nor is there another
//! road: `build::drafted` (a 45° draft from z = 5 would be this chamfer) and `build::offset`
//! refuse a spline, and the solid a 90° cone sweeps with its tip riding the spline at z = 5
//! (`swept::closed::cut_from` along `trajectory::Along`) is refused by the sweep's own judge
//! (`Unbuilt::Wrong`).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const POLES: [[f64; 2]; 8] = [
    [-10., -5.], [0., -7.], [10., -5.], [12., 4.], [4., 7.], [0., 3.], [-4., 7.], [-12., 4.],
];
const HEIGHT: f64 = 6.;
const DISTANCE: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    // the periodic curve written the way cad-kernel holds a B-spline: its eight control points with
    // the first three repeated after the last, on uniform knots, so its domain [3, 11] runs once
    // round
    let poles: Vec<_> = (0..POLES.len() + 3).map(|i| {
        let [x, y] = POLES[i % POLES.len()];
        [x, y, 0.]
    }).collect();
    let knots: Vec<f64> = (0..poles.len() + 4).map(|i| i as f64).collect();
    let outline = BSpline::new(3, knots, poles)?;
    // a point on the rim: the curve's, where its first and second segments join, lifted to the top
    let [x, y, _] = outline.point(4.);
    let floor = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                          loops: vec![vec![ProfileEdge::Spline(Arc::new(outline))]],
                          names: vec![vec!["side".into()]] };
    let boss = prism(&floor, 0., HEIGHT)?;
    // the top rim picked by that point; the kernel would cut the flat between the leg 1 in across
    // the top and the leg 1 down the side
    let piece = flat_along(&boss, [x, y, HEIGHT], [DISTANCE, DISTANCE], None,
                           tolerance(boss.size()))?;
    // the rim is convex: its piece is taken away
    Ok(combined(&boss, &piece, Op::Cut, 0.)?)
}
