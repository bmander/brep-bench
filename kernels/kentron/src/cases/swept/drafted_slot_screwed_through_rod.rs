//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a drafted bar
//! carried along a screw: its section in the plane x = 12 the slot of two half discs of radius 1
//! about (12, ±1.5, 0) and the rectangle between, carried along −x to x = 4 and drafted 3° inward,
//! screwed about the z axis, right-handed, rising 6 a turn, from a turn of −90° to +1350°.
//! Everything the bar passes through, cut.
//!
//! The bar is the slot drafted (`sweep::extrude_drafted`); its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Vector};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    // The slot in the plane x = 12: its two half discs' arcs (about x, from −z through ±y to +z)
    // and the lines between.
    let arc = |m: &mut Model, y: f64, from: f64| -> kentron::Result<Shape> {
        let circle = Circle::new(Frame::new(p(12.0, y, 0.0), Direction::X, Direction::Y), 1.0);
        make::edge_from_curve_range(m, Curve::Circle(circle), from, from + std::f64::consts::PI)
    };
    let edges = vec![
        make::edge_between(m, p(12.0, -1.5, -1.0), p(12.0, 1.5, -1.0))?,
        arc(m, 1.5, -FRAC_PI_2)?,
        make::edge_between(m, p(12.0, 1.5, 1.0), p(12.0, -1.5, 1.0))?,
        arc(m, -1.5, FRAC_PI_2)?,
    ];
    let wire = make::wire(m, &edges)?;
    let slot = make::planar_face(m, wire)?;
    let bar = sweep::extrude_drafted(m, slot, Vector::new(-8.0, 0.0, 0.0), 3f64.to_radians())?;
    let screw = RigidMotion::Screw { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), lead: 6.0, from: -FRAC_PI_2, to: 1350f64.to_radians() };
    let swept = sweep::solid(m, bar, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
