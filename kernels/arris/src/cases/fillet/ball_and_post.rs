//! A ball of radius 10 on the origin joined to a post of radius 4 on the z axis from z = 0 to 15,
//! the circle where the post leaves the ball rounded to radius 2.
use crate::solids::ball;
use arris::math::{Axis, Control, Point3};
use arris::ops::{fillet, fuse, primitive_cylinder, OpError};
use arris::topo::provenance::{CylinderPart, Role, SweepPart};
use arris::topo::{Body, Edge, Model};

const BALL_RADIUS: f64 = 10.0;
const POST_RADIUS: f64 = 4.0;
const POST_HEIGHT: f64 = 15.0;
const ROUND: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (sphere, turned) = ball(m, Point3::origin(), BALL_RADIUS, &Control::NONE)?;
    let axis = Axis::z_at(Point3::origin());
    let (post, made) = primitive_cylinder(m, axis, POST_RADIUS, POST_HEIGHT, &Control::NONE)?;
    let (joined, fused) = fuse(m, sphere, post, &Control::NONE)?;
    // the circle where the post leaves the ball: the section edge the fuse made from the sphere,
    // the half disc's arc turned, and the post's wall
    let arc = Role::Revolve(SweepPart::Side { loop_index: 0, segment: 1 });
    let sphere_face = turned.generated_from(arc)[0];
    let wall = made.generated_from(Role::Cylinder(CylinderPart::Wall))[0];
    let collar: Vec<Edge> = fused
        .generated_pair(sphere_face, wall)
        .iter()
        .filter_map(|&s| Edge::try_from(s).ok())
        .collect();
    let (rounded, _) = fillet(m, joined, &collar, ROUND, &Control::NONE)?;
    Ok(rounded)
}
