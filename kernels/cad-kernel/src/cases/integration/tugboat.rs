//! A tugboat test print (a benchy): 60 long, 30 wide and 48 high, symmetric about y = 0, its stern
//! at x = 0, its bow toward +x and the flat of its keel on z = 0. The union of six parts less the
//! openings cut into them:
//!
//! 1. the hull, the common part of the plan P extruded along z, the profile S extruded along y and
//!    the chined section z ≥ |y| − 10 extruded along x. P: the transom x = 0 (|y| ≤ 10), quarter
//!    circles of radius 5 about (5, ±10), the sides y = ±15 to x = 30, and arcs of radius 37.5
//!    about (30, ∓22.5) meeting at the stem (60, 0). S: below the sheer z = 14 + x/10 and above
//!    the keel line, the counter (radius 20 about (x, z) = (12, 20), from (0, 4) to (12, 0)), the
//!    flat keel z = 0 to x = 40, and the stem (radius 25 about (40, 25), from (40, 0) to (60, 10));
//! 2. the deck well taken out of it: z ≥ 9 over the plan moved in by 1.5;
//! 3. the wheelhouse: walls between the rounded rectangles [12, 32] × [−10, 10] (radius 2) and
//!    [13.5, 30.5] × [−8.5, 8.5] (radius 0.5), from the deck up to the roof's underside
//!    z = 30 + (x − 12)/10; the roof, a slab 1.5 thick above that plane over [10, 34] × [−12, 12]
//!    (radius 3); arched doors (16 ≤ x ≤ 22, 9 ≤ z ≤ 20 and a half disc of radius 3 about
//!    (19, 20)) through both side walls; a front window (|y| ≤ 7, 22 ≤ z ≤ 28, radius 1.5) and a
//!    round rear window (radius 4 about (y, z) = (0, 22));
//! 4. the chimney, radius 4 about the vertical through (18, 0), from z = 31.5 (inside the roof)
//!    to 48, bored to radius 2.75 down to z = 38;
//! 5. the cargo box, [2.5, 9.5] × [−5, 5] × [9, 14] less [3.5, 8.5] × [−4, 4] × [10, 14];
//! 6. the rod holder, a tube of radius 1.5 about the line through (8, 9, 9) along
//!    (−1/2, 0, √3/2), between the planes z = 8 and z = 15, bored to radius 0.9 down to z = 10;
//! 7. and the hawse holes, radius 1.25 about the line along y through (52, 0, 16).
//!
//! Each region of a plane is a profile of lines and arcs swept past where it is cut, and the
//! sloping and level cuts are `kept` at their planes.
use crate::sketch::{block, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, prism, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{cross, unit};
use std::f64::consts::{PI, TAU};

/// A coordinate plane: where its two coordinates (u, v) put a point, along its axes `x` and `y`
/// from the origin `o`, its normal `x × y`.
#[derive(Clone, Copy)]
struct Plane { o: V, x: V, y: V }

impl Plane {
    fn at(&self, [u, v]: [f64; 2]) -> V {
        std::array::from_fn(|k| self.o[k] + u * self.x[k] + v * self.y[k])
    }
    fn normal(&self) -> V { cross(self.x, self.y) }
}

/// A piece of an outline in a plane's coordinates: a line from a to b, or the arc about a center
/// from a to b the short way round.
enum Piece { Line([f64; 2], [f64; 2]), Arc([f64; 2], [f64; 2], [f64; 2]) }
use Piece::{Arc, Line};

/// The region inside the closed outline `pieces` in `plane`.
fn outline(plane: Plane, pieces: &[Piece]) -> Profile {
    let edges = pieces.iter().map(|piece| match *piece {
        Line(a, b) => ProfileEdge::Line { a: plane.at(a), b: plane.at(b) },
        Arc(c, a, b) => {
            let r = ((a[0] - c[0]).powi(2) + (a[1] - c[1]).powi(2)).sqrt();
            let (ta, tb) = ((a[1] - c[1]).atan2(a[0] - c[0]), (b[1] - c[1]).atan2(b[0] - c[0]));
            // counter-clockwise from a to b, or from b to a, whichever is the short way
            let turn = (tb - ta).rem_euclid(TAU);
            let span = if turn < PI { [ta, ta + turn] } else { [tb, tb + TAU - turn] };
            let frame = Frame::new(plane.at(c), plane.normal(), plane.x);
            ProfileEdge::Arc { frame, r, span: Some(span) }
        }
    }).collect();
    Profile { origin: plane.o, normal: plane.normal(), loops: vec![edges], names: Vec::new() }
}

/// The plane z = h, in (x, y); the plane y = h, in (x, z); the plane x = h, in (y, z).
fn xy(h: f64) -> Plane { Plane { o: [0., 0., h], x: [1., 0., 0.], y: [0., 1., 0.] } }
fn xz(h: f64) -> Plane { Plane { o: [0., h, 0.], x: [1., 0., 0.], y: [0., 0., 1.] } }
fn yz(h: f64) -> Plane { Plane { o: [h, 0., 0.], x: [0., 1., 0.], y: [0., 0., 1.] } }

/// The hull's plan moved in by `inset`: the transom, the stern's quarter circles, the sides and
/// the bow's two arcs, which meet on y = 0.
fn plan(inset: f64, z: f64) -> Profile {
    let (side, bow) = (15. - inset, 37.5 - inset);
    let tip = 30. + (bow * bow - 22.5 * 22.5).sqrt();
    outline(xy(z), &[
        Line([inset, -10.], [inset, 10.]), Arc([5., 10.], [inset, 10.], [5., side]),
        Line([5., side], [30., side]), Arc([30., -22.5], [30., side], [tip, 0.]),
        Arc([30., 22.5], [tip, 0.], [30., -side]), Line([30., -side], [5., -side]),
        Arc([5., -10.], [5., -side], [inset, -10.]),
    ])
}

fn op(a: &Brep, b: &Brep, op: Op) -> Result<Brep, String> { Ok(combined(a, b, op, 0.)?) }

/// The roof's slope: the unit normal of the planes z = c + (x − 12)/10, upward.
fn slope() -> V { unit([-0.1, 0., 1.]) }

fn hull() -> Result<Brep, String> {
    let above_plan = prism(&plan(0., -2.), 0., 30.)?;
    // the profile, run past x = 0 and x = 60 along its circles: the counter from x = −1, the stem
    // to x = 62, then up past the sheer and back along it
    let counter = [-1., 20. - (400f64 - 13. * 13.).sqrt()];
    let stem = [62., 25. - (625f64 - 22. * 22.).sqrt()];
    let profile = outline(xz(0.), &[
        Arc([12., 20.], counter, [12., 0.]), Line([12., 0.], [40., 0.]),
        Arc([40., 25.], [40., 0.], stem), Line(stem, [62., 20.2]),
        Line([62., 20.2], [-1., 13.9]), Line([-1., 13.9], counter),
    ]);
    let along_profile = prism(&profile, -20., 20.)?;
    // the chines, z ≥ |y| − 10: a V whose point is under the keel, closed high above the sheer
    let chines = outline(yz(-2.), &[
        Line([0., -10.], [30., 20.]), Line([30., 20.], [30., 30.]), Line([30., 30.], [-30., 30.]),
        Line([-30., 30.], [-30., 20.]), Line([-30., 20.], [0., -10.]),
    ]);
    let along_section = prism(&chines, 0., 66.)?;
    let h = op(&op(&above_plan, &along_profile, Op::Common)?, &along_section, Op::Common)?;
    let well = prism(&plan(1.5, 9.), 0., 20.)?;
    op(&h, &well, Op::Cut)
}

fn wheelhouse() -> Result<Brep, String> {
    let outer = prism(&rounded_rectangle([12., -10.], [32., 10.], 2., 9.), 0., 30.)?;
    let inner = prism(&rounded_rectangle([13.5, -8.5], [30.5, 8.5], 0.5, 8.), 0., 31.)?;
    let n = slope();
    let walls = kept(&op(&outer, &inner, Op::Cut)?, [12., 0., 30.], n.map(|c| -c), 0.)?;
    let slab = prism(&rounded_rectangle([10., -12.], [34., 12.], 3., 25.), 0., 15.)?;
    let roof = kept(&kept(&slab, [12., 0., 30.], n, 0.)?, [12., 0., 31.5], n.map(|c| -c), 0.)?;
    let mut house = op(&walls, &roof, Op::Union)?;
    let door = outline(xz(-11.), &[
        Line([16., 9.], [22., 9.]), Line([22., 9.], [22., 20.]), Arc([19., 20.], [22., 20.], [19., 23.]),
        Arc([19., 20.], [19., 23.], [16., 20.]), Line([16., 20.], [16., 9.]),
    ]);
    // (the plane y = −11 in (x, z) faces −y: the doors run 22 along −(−y))
    house = op(&house, &prism(&door, -22., 0.)?, Op::Cut)?;
    let window = outline(yz(30.), &[
        Line([-5.5, 22.], [5.5, 22.]), Arc([5.5, 23.5], [5.5, 22.], [7., 23.5]),
        Line([7., 23.5], [7., 26.5]), Arc([5.5, 26.5], [7., 26.5], [5.5, 28.]),
        Line([5.5, 28.], [-5.5, 28.]), Arc([-5.5, 26.5], [-5.5, 28.], [-7., 26.5]),
        Line([-7., 26.5], [-7., 23.5]), Arc([-5.5, 23.5], [-7., 23.5], [-5.5, 22.]),
    ]);
    house = op(&house, &prism(&window, 0., 3.)?, Op::Cut)?;
    let rear = prism(&disc([11., 0., 22.], [1., 0., 0.], 4.), 0., 3.)?;
    op(&house, &rear, Op::Cut)
}

fn chimney() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let stack = prism(&disc([18., 0., 31.5], up, 4.), 0., 16.5)?;
    let bore = prism(&disc([18., 0., 38.], up, 2.75), 0., 11.)?;
    op(&stack, &bore, Op::Cut)
}

fn cargo_box() -> Result<Brep, String> {
    op(&block([2.5, -5., 9.], [9.5, 5., 14.])?, &block([3.5, -4., 10.], [8.5, 4., 15.])?, Op::Cut)
}

fn rod_holder() -> Result<Brep, String> {
    let s3 = 3f64.sqrt();
    let lean = [-0.5, 0., s3 / 2.];
    // the axis from where it is at z = 6, well under the deck, for 12 along it (to z ≈ 16.4)
    let start = [8. + s3, 9., 6.];
    let (up, down) = ([0., 0., 1.], [0., 0., -1.]);
    let tube = prism(&disc(start, lean, 1.5), 0., 12.)?;
    let tube = kept(&kept(&tube, [0., 0., 8.], up, 0.)?, [0., 0., 15.], down, 0.)?;
    let bore = kept(&prism(&disc(start, lean, 0.9), 0., 12.)?, [0., 0., 10.], up, 0.)?;
    op(&tube, &bore, Op::Cut)
}

pub fn build() -> Result<Brep, String> {
    let mut boat = hull()?;
    for part in [wheelhouse()?, chimney()?, cargo_box()?, rod_holder()?] {
        boat = op(&boat, &part, Op::Union)?;
    }
    let hawse = prism(&disc([52., -12., 16.], [0., 1., 0.], 1.25), 0., 24.)?;
    op(&boat, &hawse, Op::Cut)
}
