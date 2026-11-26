//! A plate, the box from (−10, −10, 0) to (10, 10, 4), cut right through by a core drill pushed
//! along its own leaning axis: the tube between radii 1.5 and 2.5 about the line through (0, 0, 2)
//! along u = (sin 30°, 0, cos 30°), running from 18 to 6 before (0, 0, 2), slid along u by 24 to
//! run from 6 to 18 past it. The core inside the tube is cut free and stays: two solids.
//!
//! The tube is a cylinder less a narrower one; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one slide), cut from the plate. kentron refuses it: following where the slide
//! grazes the tube's faces (all along its two cylinders, which slide along themselves), a point
//! does not settle on the curve.
use crate::bench::{frame, p, v};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 4.0)).build(m)?;
    let (sin, cos) = (30f64.to_radians().sin(), 30f64.to_radians().cos());
    let start = frame(p(-18.0 * sin, 0.0, 2.0 - 18.0 * cos), Direction::new(sin, 0.0, cos));
    let outer = Cylinder::new(2.5, 12.0).with_frame(start).build(m)?;
    let inner = Cylinder::new(1.5, 12.0).with_frame(start).build(m)?;
    let tube = boolean::difference(m, outer, inner)?;
    let swept = sweep::solid(m, tube, &RigidMotion::Slide { by: v(sin, 0.0, cos) * 24.0 })?;
    boolean::difference(m, plate, swept)
}
