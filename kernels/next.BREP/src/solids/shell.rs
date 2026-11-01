//! Helpers shared by the shell cases: a face picked by a point inside it, the kernel's shell with
//! its openings picked that way, a solid less another, and the bodies several cases shell.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, fillet_edges, make_arc, make_box_brep, make_cylinder_brep, make_line,
    make_sphere_brep, offset_shell, parameter_point_in_face, project_point_to_surface,
    revolve_profile_brep, split_solid_by_plane, BooleanOperation, BooleanOptions, BrepSolid,
    PolygonClass, Vec2, Vec3,
};
use std::f64::consts::{FRAC_PI_2, TAU};

/// The id of the face of `solid` with `point` inside it: the point's foot on each face's surface,
/// classified against that face's trim by the kernel.
pub fn face_at(solid: &BrepSolid, point: Vec3) -> Result<u64, String> {
    for face in solid.shells.iter().flat_map(|shell| &shell.faces) {
        let Ok(foot) = project_point_to_surface(&face.surface, point) else { continue };
        if foot.distance > 1e-7 {
            continue;
        }
        if parameter_point_in_face(face, Vec2 { x: foot.u, y: foot.v }, 1e-9)?
            == PolygonClass::Inside
        {
            return Ok(face.id);
        }
    }
    Err(format!("no face has ({}, {}, {}) inside it", point.x, point.y, point.z))
}

/// `solid` shelled by the kernel to a wall `thickness` thick, open at the faces with the points
/// `openings` inside them. The kernel's sign: a positive thickness moves the kept faces inward,
/// into the material; a negative one moves them outward.
pub fn shell(solid: &BrepSolid, openings: &[Vec3], thickness: f64) -> Result<BrepSolid, Refusal> {
    let faces = openings.iter().map(|&q| face_at(solid, q)).collect::<Result<Vec<_>, _>>()?;
    Ok(offset_shell(solid, &faces, thickness)?.solid)
}

/// `solid` less `tool`.
pub fn less(solid: &BrepSolid, tool: &BrepSolid) -> Result<BrepSolid, Refusal> {
    Ok(boolean_operation(solid, tool, BooleanOperation::Subtract, &BooleanOptions::default())?)
}

/// The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2.
pub fn rounded_plate() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 14.0, 10.0)?;
    let edges = [p(0.0, 0.0, 5.0), p(20.0, 0.0, 5.0), p(20.0, 14.0, 5.0), p(0.0, 14.0, 5.0)];
    Ok(fillet_edges(&block, &edges, None, 2.0, false, None)?)
}

/// The block from the origin to (20, 20, 10) with a hole of radius 3 bored right through it along
/// z about the line x = `x`, y = 10, the drill running past both faces.
pub fn drilled_block(x: f64) -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 20.0, 10.0)?;
    let drill = make_cylinder_brep(p(x, 10.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 12.0)?;
    less(&block, &drill)
}

/// The part of the ball of radius 5 centred on the origin below the plane z = `h`.
pub fn ball_below(h: f64) -> Result<BrepSolid, Refusal> {
    let ball = make_sphere_brep(p(0.0, 0.0, 0.0), 5.0, p(0.0, 0.0, 1.0))?;
    Ok(split_solid_by_plane(&ball, p(0.0, 0.0, h), p(0.0, 0.0, 1.0))?.0)
}

/// A capsule's end: the cylinder of radius 5 on the z axis from z = 0 to z = 10, capped by the
/// half ball of radius 5 about (0, 0, 10). Its half section in the xz plane, the rectangle and
/// the quarter disc, revolved a full turn about the z axis.
pub fn capsule() -> Result<BrepSolid, Refusal> {
    let (centre, x, z) = (p(0.0, 0.0, 10.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let dome = make_arc(centre, x, z, 5.0, 0.0, FRAC_PI_2)?;
    let section = [
        make_line(p(0.0, 0.0, 15.0), p(0.0, 0.0, 0.0))?,
        make_line(p(0.0, 0.0, 0.0), p(5.0, 0.0, 0.0))?,
        make_line(p(5.0, 0.0, 0.0), p(5.0, 0.0, 10.0))?,
        dome,
    ];
    Ok(revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?)
}
