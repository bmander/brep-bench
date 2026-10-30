//! A cylinder of radius 5 and height 10 with the edge around its top rounded to radius 1.
use arris::math::{Axis, Control, Point3};
use arris::ops::{fillet, primitive_cylinder, OpError};
use arris::topo::provenance::{CylinderPart, Role};
use arris::topo::{Body, Edge, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const ROUND: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cylinder, made) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    // the top cap's circle, named by the cylinder's provenance
    let rim: Vec<Edge> = made
        .generated_from(Role::Cylinder(CylinderPart::TopRim))
        .iter()
        .filter_map(|&s| Edge::try_from(s).ok())
        .collect();
    let (rounded, _) = fillet(m, cylinder, &rim, ROUND, &Control::NONE)?;
    Ok(rounded)
}
