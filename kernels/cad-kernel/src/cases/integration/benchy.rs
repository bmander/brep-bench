//! #3DBenchy, as near as a few solids come to it. The hull is lofted (ruled) through its plan at
//! seven heights, each read from the reference on eleven rays from the origin — a bilinear sheet
//! for each side of each band between two plans, sewn with the lowest and highest plans into a
//! solid — and kept below its side profile; its deck well is the same loft with each plan moved in
//! by the wall (`offset`), kept above the deck. The wheelhouse is a block less its room, under a
//! roof sloping 1 in 10, with a console at its front and a sill at its back; its doors are a block
//! and a cylinder through both sides, its front window a block, its rear window a cylinder. The
//! chimney and the rod holder are bored cylinders, the cargo box a block less its hold. Every
//! number was measured from the reference's sections.
use crate::sketch::{block, polygon};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, face, offset, prism, sheet, ProfileEdge};
use cad_kernel::geom::tolerance;
use cad_kernel::sew::sewn;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

/// The hull's plan at each height, its half on y ≥ 0 from the bow round to the stern.
const PLAN: [(f64, [[f64; 2]; 11]); 7] = [
    (0., [[13.9, 0.], [13.1, 1.8], [11.1, 4.0], [7.7, 6.5], [2.9, 7.9], [-1.4, 7.9], [-6.4, 7.6],
          [-12.8, 7.4], [-18.9, 6.9], [-25.1, 4.4], [-25.1, 0.]]),
    (2., [[16.1, 0.], [15.0, 2.1], [12.8, 4.6], [8.8, 7.4], [3.3, 9.1], [-1.6, 9.1], [-7.5, 8.9],
          [-14.5, 8.4], [-21.5, 7.8], [-25.9, 4.6], [-25.1, 0.]]),
    (5., [[19.6, 0.], [18.3, 2.6], [15.5, 5.7], [10.8, 9.1], [4.1, 11.1], [-2.0, 11.2],
          [-8.9, 10.6], [-17.1, 9.9], [-25.1, 9.1], [-27.1, 4.8], [-27.1, 0.]]),
    (8.5, [[23.4, 0.], [21.6, 3.0], [18.4, 6.7], [12.7, 10.6], [4.8, 13.1], [-2.3, 13.1],
           [-10.4, 12.4], [-19.7, 11.4], [-28.0, 10.2], [-28.6, 5.1], [-28.6, 0.]]),
    (12., [[26.1, 0.], [24.3, 3.4], [20.6, 7.5], [14.2, 11.9], [5.3, 14.6], [-2.6, 14.6],
           [-11.0, 13.1], [-20.6, 11.9], [-28.7, 10.4], [-29.1, 5.1], [-29.1, 0.]]),
    (17., [[28.9, 0.], [26.7, 3.8], [22.5, 8.2], [15.4, 12.9], [5.6, 15.4], [-2.6, 14.6],
           [-11.0, 13.1], [-20.6, 11.9], [-28.7, 10.4], [-29.1, 5.1], [-29.1, 0.]]),
    (28., [[30.9, 0.], [28.4, 4.0], [23.8, 8.7], [16.0, 13.4], [5.6, 15.4], [-2.6, 14.6],
           [-11.0, 13.1], [-20.6, 11.9], [-28.7, 10.4], [-29.1, 5.1], [-29.1, 0.]]),
];
/// The side profile (x, z): the sheer, the stern and the keel, the bow.
const SIDE: [[f64; 2]; 12] = [[30.9, 25.9], [28.8, 27.0], [9.8, 21.2], [5.4, 15.1], [2.5, 13.8],
    [-9.5, 11.5], [-24.7, 10.8], [-28.9, 11.9], [-29.1, 9.1], [-25.0, 0.0], [13.8, 0.0],
    [26.9, 12.9]];
/// The deck (x, z), and the well above it.
const DECK: [[f64; 2]; 6] =
    [[-40., 8.5], [5., 8.5], [10.5, 15.], [40., 23.2], [40., 60.], [-40., 60.]];

fn op(a: Brep, bs: &[Brep], op: Op) -> Result<Brep, String> {
    bs.iter().try_fold(a, |a, b| Ok(combined(&a, b, op, 0.)?))
}

/// The prism of the polygon `pts` of the xz plane, from y = −20 to 20.
fn across(pts: &[[f64; 2]]) -> Result<Brep, String> {
    prism(&polygon(&pts.iter().map(|&[x, z]| [x, 0., z]).collect::<Vec<_>>()), -20., 20.)
}

fn cylinder(foot: [f64; 3], along: [f64; 3], r: f64, h: f64) -> Result<Brep, String> {
    prism(&disc(foot, along, r), 0., h)
}

/// The hull's plans lofted, each moved in by `inset`: a ruled band between each two plans, one
/// bilinear sheet for each side.
fn hull(inset: f64) -> Result<Brep, String> {
    let rings = PLAN.iter().map(|(z, half)| {
        // counter-clockwise about z: up the port side from the bow, back down the starboard
        let ring: Vec<_> = half[..10].iter().map(|&[x, y]| [x, y, *z])
            .chain(half[1..].iter().rev().map(|&[x, y]| [x, -y, *z])).collect();
        let moved = offset(&polygon(&ring), inset)?;
        moved.loops[0].iter().map(|e| match *e {
            ProfileEdge::Line { a, .. } => Ok(a),
            _ => Err("a plan moved in is lines".to_string()),
        }).collect::<Result<Vec<_>, String>>()
    }).collect::<Result<Vec<Vec<_>>, String>>()?;
    // a sheet for each side of each band between two plans, sewn with the lowest and the highest
    let mut faces = vec![face(&polygon(&rings[0]))?, face(&polygon(&rings[rings.len() - 1]))?];
    for w in rings.windows(2) {
        let n = w[0].len();
        for k in 0..n {
            let poles = vec![vec![w[0][k], w[1][k]], vec![w[0][(k + 1) % n], w[1][(k + 1) % n]]];
            let knots = vec![0., 0., 1., 1.];
            faces.push(sheet(Net { du: 1, dv: 1, uknots: knots.clone(), vknots: knots, poles,
                                   weights: None })?);
        }
    }
    Ok(sewn(&faces, tolerance(60.))?)
}

pub fn build() -> Result<Brep, String> {
    let (up, port, ahead) = ([0., 0., 1.], [0., 1., 0.], [1., 0., 0.]);
    let well = combined(&hull(1.3)?, &across(&DECK)?, Op::Common, 0.)?;
    let boat = op(hull(0.)?, &[across(&SIDE)?], Op::Common)?;
    let boat = op(boat, &[well], Op::Cut)?;
    let house = op(block([-7.25, -8.3, 8.], [12.5, 8.3, 36.6])?,
                   &[block([-5.5, -6.5, 8.5], [11., 6.5, 37.])?], Op::Cut)?;
    let roof = op(block([-9., -9., 36.5], [14., 9., 45.])?,
                  &[across(&[[-20., 0.], [40., 0.], [40., 42.9], [-20., 36.9]])?], Op::Common)?;
    let stack = op(cylinder([-4.2, 0., 36.6], up, 2.9, 11.4)?,
                   &[cylinder([-4.2, 0., 36.9], up, 1.5, 12.)?], Op::Cut)?;
    let rod = op(cylinder([-26.5, 0., 8.], up, 1.6, 4.3)?,
                 &[cylinder([-26.5, 0., 9.], up, 1.1, 4.)?], Op::Cut)?;
    let console = block([6.8, -6.5, 8.], [12.5, 6.5, 20.5])?;
    let sill = block([-9., -6.5, 8.], [-5.5, 6.5, 15.4])?;
    let cargo = block([-17.7, -5.75, 8.], [-9., 5.75, 15.4])?;
    let boat = op(boat, &[house, roof, console, sill, stack, cargo, rod], Op::Union)?;
    op(boat, &[block([-2.5, -10., 8.5], [6.8, 10., 27.5])?, // the doors
               cylinder([2.15, -10., 27.5], port, 4.65, 20.)?,
               block([10., -5.3, 24.4], [14., 5.3, 34.])?, // the front window
               cylinder([-9., 0., 29.5], ahead, 4.6, 5.)?, // the rear window
               block([-16.2, -4.1, 6.4], [-10.5, 4.1, 16.])?], Op::Cut) // the hold
}
