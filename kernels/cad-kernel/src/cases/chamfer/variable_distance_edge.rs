//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet chamfered with a distance on the face x = 10 growing linearly along the edge,
//! d₁ = 1 + z/5, and a constant distance d₂ = 2 on the face y = 10. At each height the chamfer's
//! section is the line from (10, 10 − d₁, z) to (10 − d₂, 10, z): the chamfer is the hyperbolic
//! paraboloid ruled by those horizontal lines between two skew lines.
//!
//! cad-kernel's chamfer, `fillet::flat_along(&b, at, legs, ..)`, takes two legs constant along the
//! whole edge, and `corners::chamfer` chamfers a planar region's corner; neither takes a distance
//! that changes along the edge. So the wedge the chamfer takes away is built and cut from the cube:
//! its section at each height is the triangle the chamfer's line cuts off the corner, and every
//! corner of it moves linearly with z, so it is a `build::lofted` solid whose walls are bilinear
//! B-spline patches, the chamfer's own the hyperbolic paraboloid exactly.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{block, lofted};
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
/// d₁ at z = 0 and its rate along the edge: d₁ = 1 + z/5.
const D1_FOOT: f64 = 1.;
const D1_RATE: f64 = 1. / 5.;
const D2: f64 = 2.;
/// How far the wedge reaches past the cube: below its floor and over its top, and along the
/// chamfer's line past the faces x = 10 and y = 10, as a fraction of its length.
const PAST: f64 = 1.;
const ON: f64 = 0.5;

/// The distance on the face x = 10 at height `z`.
fn d1(z: f64) -> f64 { D1_FOOT + D1_RATE * z }

/// A wall of the wedge, linear across the section (`u`, from the corner `a` to the corner `b`, each
/// a function of the height) and linear along z (`v`).
fn wall(a: &dyn Fn(f64) -> [f64; 3], b: &dyn Fn(f64) -> [f64; 3], z: [f64; 2]) -> Net {
    let linear = vec![0., 0., 1., 1.];
    Net { du: 1, dv: 1, uknots: linear.clone(), vknots: linear,
          poles: vec![vec![a(z[0]), a(z[1])], vec![b(z[0]), b(z[1])]], weights: None }
}

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    // the chamfer's line at height z, from (10, 10 − d₁) at s = 0 to (10 − d₂, 10) at s = 1
    let line = |s: f64, z: f64| [SIDE - D2 * s, SIDE - d1(z) * (1. - s), z];
    // the wedge's section: the chamfer's line run on past both faces, from s = −½ (past x = 10) to
    // s = 3/2 (past y = 10), closed by the corner over both; each corner linear in z
    let a = |z: f64| line(-ON, z);
    let b = |z: f64| line(1. + ON, z);
    let c = |z: f64| [a(z)[0], b(z)[1], z];
    // walked counter-clockwise about +z: along the plane x = 11 to the corner, along the plane past
    // y = 10 back to the line, and along the chamfer's hyperbolic paraboloid
    let z = [-PAST, SIDE + PAST];
    let walls = vec![wall(&a, &c, z), wall(&c, &b, z), wall(&b, &a, z)];
    let wedge = lofted([0.; 3], [0., 0., 1.], z, vec![walls])?;
    Ok(combined(&cube, &wedge, Op::Cut, 0.)?)
}
