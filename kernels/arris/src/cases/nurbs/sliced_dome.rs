//! The block under the Bézier dome of `nurbs/bezier-dome`, with everything above the inclined
//! plane z = 4 + x/2, which passes through the block's corner (0, 0, 4), cut away.
use super::bezier_dome;
use crate::Refusal;
use arris::math::nalgebra::UnitQuaternion;
use arris::math::{Control, Isometry, Point3, Vec3};
use arris::ops::{cut, primitive_box, transform};
use arris::topo::{Body, Model};

/// The plane z = HEIGHT + SLOPE·x.
const HEIGHT: f64 = 4.0;
const SLOPE: f64 = 0.5;
/// How far the lid reaches from the corner along the plane each way, and above it.
const REACH: f64 = 30.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let block = bezier_dome::build(m)?;
    // A lid standing on the plane: a box on z = 0, tipped about −y until its floor rises 1 in 2
    // along x, and lifted to the corner. Splitting the block by the plane itself, arris 0.9.1
    // declines: "kernel bug: a geometry query failed: the section of the NURBS surface and the
    // plane surface is degenerate: turning points of the section that cannot be told apart".
    let (lid, _) = primitive_box(
        m,
        Point3::new(-REACH, -REACH, 0.0),
        Point3::new(REACH, REACH, REACH),
        &Control::NONE,
    )?;
    let tip = UnitQuaternion::from_axis_angle(&-Vec3::y_axis(), SLOPE.atan());
    let lift = Isometry::new(tip, Vec3::new(0.0, 0.0, HEIGHT));
    let (lid, _) = transform(m, lid, &lift, &Control::NONE)?;
    let (sliced, _) = cut(m, block, lid, &Control::NONE)?;
    Ok(sliced)
}
