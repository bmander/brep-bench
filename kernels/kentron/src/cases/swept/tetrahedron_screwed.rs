//! The solid a regular tetrahedron fills as it is carried along a screw: the tetrahedron with
//! corners (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1, 1), turned right-handed about the z axis
//! while rising 5 a turn, rigidly, from a turn of 0° to 540°.
//!
//! The tetrahedron is the solid its four triangles enclose; its sweep is kentron's sweep of a
//! solid (`sweep::solid`, one screw).
use crate::bench::p;
use crate::shared::offset::polygon_face;
use kentron::boolean;
use kentron::math::Axis;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (a, b, c, d) = (p(5.0, 1.0, 1.0), p(5.0, -1.0, -1.0), p(3.0, 1.0, -1.0), p(3.0, -1.0, 1.0));
    let mut faces = Vec::new();
    for triangle in [[a, b, c], [a, c, d], [a, d, b], [b, d, c]] {
        faces.push(polygon_face(m, &triangle)?);
    }
    let tetrahedron = boolean::enclosed_solids(m, &faces)?;
    sweep::solid(m, tetrahedron, &RigidMotion::Screw { axis: Axis::Z, lead: 5.0, from: 0.0, to: 1.5 * TAU })
}
