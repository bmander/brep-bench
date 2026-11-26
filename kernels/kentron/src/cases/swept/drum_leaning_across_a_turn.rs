//! A ring, distance from the z axis between 3 and 8 and z between −2.5 and 2.5, with a channel cut
//! inside it by a drum leaning across a turn: the solid cylinder of radius 1 and length 3 whose
//! axis runs along (sin 25°, 0, cos 25°) through its middle (5, 1.2, 0), turned right-handed about
//! the z axis by 100°. Everything the drum passes through, cut.
//!
//! The drum's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the ring.
use crate::bench::{d, frame, p};
use kentron::math::{Axis, Direction, Vector};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outer = Cylinder::new(8.0, 5.0).with_frame(frame(p(0.0, 0.0, -2.5), d(0.0, 0.0, 1.0))).build(m)?;
    let inner = Cylinder::new(3.0, 5.0).with_frame(frame(p(0.0, 0.0, -2.5), d(0.0, 0.0, 1.0))).build(m)?;
    let ring = kentron::boolean::difference(m, outer, inner)?;
    let lean = 25f64.to_radians();
    let axis = Vector::new(lean.sin(), 0.0, lean.cos());
    let drum = Cylinder::new(1.0, 3.0).with_frame(frame(p(5.0, 1.2, 0.0) - axis * 1.5, Direction::from_vector(axis))).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), from: 0.0, to: 100f64.to_radians() };
    let swept = sweep::solid(m, drum, &turn)?;
    kentron::boolean::difference(m, ring, swept)
}
