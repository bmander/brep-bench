//! A regular hexagonal prism from z = 0 to z = 10 on the hexagon of side 6 centred on the z axis,
//! its corners at (6 cos 60k°, 6 sin 60k°); its edge from (6, 0, 0) to (6, 0, 10), where its sides
//! meet at 120°, rounded with a radius that grows linearly along it, r = 1 + 3z/10.
//!
//! Built round the kernel's lack of a variable-radius round (`fillet::roll` rolls a ball of one
//! radius; nothing in `fillet`, `rounds` or `corners` takes a radius that changes along an edge),
//! from what the case says the round is: its sections square to the edge are arcs of radius r(z)
//! about (6 − 2r/√3, 0, z), turning 60° between the two sides, so it is part of an oblique
//! circular cone — the right frustum `build::cone` makes, of radius 1 at z = 0 and 4 at z = 10,
//! sheared across its axis by `affine::Affine` (`Brep::mapped`) until its centres run along that
//! line. The corner the round takes lies past the two planes through the cone's axis and the lines
//! it touches the sides along (each square to its side): the prism is cut there and the sector of
//! the cone between those planes put back.
use crate::sketch::polygon;
use cad_kernel::affine::Affine;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, kept, prism};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, dot, scale, sub, unit};
use std::f64::consts::PI;

const SIDE: f64 = 6.;
const HEIGHT: f64 = 10.;
/// The round's radius at z = 0 and at z = HEIGHT.
const R: [f64; 2] = [1., 4.];

pub fn build() -> Result<Brep, String> {
    let corners: Vec<[f64; 3]> = (0..6)
        .map(|k| { let a = k as f64 * PI / 3.; [SIDE * a.cos(), SIDE * a.sin(), 0.] }).collect();
    let body = prism(&polygon(&corners), 0., HEIGHT)?;
    // the two sides at the edge's foot (6, 0, 0), their outward normals at ±30°; the ball of
    // radius r touching both stands r (n1 + n2) / (1 + n1·n2) in from the edge
    let edge = corners[0];
    let n = [PI / 6., -PI / 6.].map(|a| [a.cos(), a.sin(), 0.]);
    let back = scale(add(n[0], n[1]), 1. / (1. + dot(n[0], n[1])));
    // the radius r(z) = R0 + slope z: the cone's axis runs from edge − R0 back at z = 0, rising
    // along `run` per unit of z
    let slope = (R[1] - R[0]) / HEIGHT;
    let foot = sub(edge, scale(back, R[0]));
    let run = [-slope * back[0], -slope * back[1], 1.];
    // each plane holds the axis and the way to the side it touches; kept on the edge's side
    let corner = |b: &Brep| -> Result<Brep, String> {
        n.iter().try_fold(b.clone(), |b, &nk| {
            let normal = unit(cross(nk, run));
            let side = dot(normal, sub(edge, foot)).signum();
            kept(&b, foot, scale(normal, side), 0.)
        })
    };
    // the cone: radius R0 about the z axis at z = 0 and R1 at z = HEIGHT, sheared so its section
    // at height z is centred on the axis above
    let shear = Affine::new([[1., 0., run[0]], [0., 1., run[1]], [0., 0., 1.]], foot)?;
    let round = cone([0.; 3], [0., 0., 1.], R[0], R[1], HEIGHT)?.mapped(&shear);
    let cut = combined(&body, &corner(&body)?, Op::Cut, 0.)?;
    Ok(combined(&cut, &corner(&round)?, Op::Union, 0.)?)
}
