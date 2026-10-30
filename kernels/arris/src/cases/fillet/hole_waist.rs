//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 on the z axis, both of the
//! hole's rims rounded to radius 1, half the plate's thickness, so its wall vanishes.
use arris::math::{Axis, Control, Point3};
use arris::ops::{cut, fillet, primitive_box, primitive_cylinder, OpError};
use arris::topo::provenance::{BoxPart, Coord, CylinderPart, Role, Side};
use arris::topo::{Body, Edge, Model};

const HALF_WIDTH: f64 = 10.0;
const THICKNESS: f64 = 2.0;
const HOLE_RADIUS: f64 = 3.0;
const ROUND: f64 = 1.0;
/// How far the drill runs past each face, so its ends lie clear of the plate's.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (plate, boxed) = primitive_box(
        m,
        Point3::new(-HALF_WIDTH, -HALF_WIDTH, 0.0),
        Point3::new(HALF_WIDTH, HALF_WIDTH, THICKNESS),
        &Control::NONE,
    )?;
    let base = Point3::new(0.0, 0.0, -OVERRUN);
    let (drill, made) = primitive_cylinder(
        m,
        Axis::z_at(base),
        HOLE_RADIUS,
        THICKNESS + 2.0 * OVERRUN,
        &Control::NONE,
    )?;
    let (drilled, bored) = cut(m, plate, drill, &Control::NONE)?;
    // each rim is the section the cut made from a face of the plate and the drill's wall
    let wall = made.generated_from(Role::Cylinder(CylinderPart::Wall))[0];
    let mut rims: Vec<Edge> = Vec::new();
    for side in [Side::Max, Side::Min] {
        let face = boxed.generated_from(Role::Box(BoxPart::Face(Coord::Z, side)))[0];
        let rim = bored.generated_pair(face, wall);
        rims.extend(rim.iter().filter_map(|&s| Edge::try_from(s).ok()));
    }
    let (rounded, _) = fillet(m, drilled, &rims, ROUND, &Control::NONE)?;
    Ok(rounded)
}
