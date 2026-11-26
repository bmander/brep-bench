//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground round it by a wheel shaped as an oblate spheroid: centred at (10.5, 0, 0), semi-axes 6, 6
//! and 1.5, its short axis along (0, cos h, −sin h), h = atan(2π·4.75/30), carried along the screw
//! about the z axis, right-handed, rising 30 a turn, from a turn of −90° to +330°. Everything the
//! wheel passes through, cut.
//!
//! The wheel is its meridian (half an ellipse and its axis) revolved and placed; its sweep is
//! kentron's sweep of a solid (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Point, Transform, Vector};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    // The spheroid about the z axis, its meridian in the xz plane: from its pole below round
    // through (6, 0, 0) to its pole above, and back down the axis.
    let meridian = Ellipse::new(Frame::new(Point::ORIGIN, Direction::NEG_Y, Direction::X), 6.0, 1.5);
    let edges = vec![
        make::edge_from_curve_range(m, Curve::Ellipse(meridian), -FRAC_PI_2, FRAC_PI_2)?,
        make::edge_between(m, p(0.0, 0.0, 1.5), p(0.0, 0.0, -1.5))?,
    ];
    let wire = make::wire(m, &edges)?;
    let half = make::planar_face(m, wire)?;
    let upright = sweep::revolve(m, half, Axis::Z, TAU)?.shape;
    // Its axis turned from z to (0, cos h, −sin h) (about x by −(90° + h)), then moved to its
    // centre.
    let h = (TAU * 4.75 / 30.0).atan();
    let place = Transform::translation(Vector::new(10.5, 0.0, 0.0)) * Transform::rotation(Axis::X, -(FRAC_PI_2 + h));
    let wheel = kentron::modify::transform(m, upright, &place, Default::default())?;
    let screw = RigidMotion::Screw { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), lead: 30.0, from: -FRAC_PI_2, to: 330f64.to_radians() };
    let swept = sweep::solid(m, wheel, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
