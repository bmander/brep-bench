//! A cylinder of radius 5 and height 10 on the z axis, the edge round its top chamfered 5 on both
//! faces, its own radius, so the top becomes a cone with its apex at (0, 0, 10).
use arris::math::{Axis, Control, Point3};
use arris::ops::{chamfer, primitive_cylinder, OpError};
use arris::topo::provenance::{CylinderPart, Role};
use arris::topo::{Body, Edge, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const DISTANCE: f64 = RADIUS;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (cylinder, made) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    // the top cap's circle, named by the cylinder's provenance
    let rim: Vec<Edge> = made
        .generated_from(Role::Cylinder(CylinderPart::TopRim))
        .iter()
        .filter_map(|&s| Edge::try_from(s).ok())
        .collect();
    let (pointed, _) = chamfer(m, cylinder, &rim, DISTANCE, &Control::NONE)?;
    Ok(pointed)
}
