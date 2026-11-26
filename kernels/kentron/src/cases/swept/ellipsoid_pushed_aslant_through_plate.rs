//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched by an ellipsoid pushed
//! aslant through it: semi-axes 3, 2 and 1.5 along x, y and z, centred at (−2, −1.5, −4), slid
//! without turning along (4, 3, 12), from wholly below the plate to wholly above it. Everything the
//! ellipsoid passes through, cut.
//!
//! kentron has no ellipsoid, so it is the unit ball stretched by 3, 2 and 1.5 along x, y and z and
//! moved to its centre, a general affine map that `modify::transform_affine` carries out exactly,
//! as a rational B-spline. Its sweep is kentron's sweep of a solid (`sweep::solid`, one slide),
//! cut from the plate.
use crate::bench::{p, v};
use kentron::math::{AffineTransform, Matrix3};
use kentron::modify;
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 4.0)).build(m)?;
    let ball = Sphere::new(1.0).build(m)?;
    let stretch = Matrix3::from_rows([[3.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.5]]);
    let ellipsoid = modify::transform_affine(m, ball, &AffineTransform::new(stretch, v(-2.0, -1.5, -4.0)))?;
    let swept = sweep::solid(m, ellipsoid, &RigidMotion::Slide { by: v(4.0, 3.0, 12.0) })?;
    kentron::boolean::difference(m, plate, swept)
}
