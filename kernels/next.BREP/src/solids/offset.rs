//! Helpers shared by the offset and thicken cases.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, boolean_operation_nary, extrude_profile_brep, make_arc, make_box_brep,
    make_line, offset_shell, offset_solid, parameter_point_in_face, project_point_to_surface,
    thicken_face_sheet, thicken_trimmed_sheet, BooleanOperation,
    BooleanOptions, BrepSolid, KernelRefusal, NurbsCurve, NurbsSurface, OffsetCorners,
    PolygonClass, Vec2, Vec3,
};
use std::f64::consts::FRAC_PI_2;

/// `solid` grown by `distance` (shrunk where it is negative) to its parallel body: every point
/// within `distance` of it, or at least `-distance` from its outside. The kernel rounds the joins
/// the ball rolls along (the convex edges of a growth, the concave ones of a shrink) and leaves
/// the others sharp.
pub fn parallel(solid: &BrepSolid, distance: f64) -> Result<BrepSolid, KernelRefusal> {
    Ok(offset_solid(solid, distance, OffsetCorners::Rounded)?.solid)
}

/// `solid` with every face moved `distance` along its outward normal and the moved faces extended
/// until they meet: no join rounded.
pub fn sharp(solid: &BrepSolid, distance: f64) -> Result<BrepSolid, KernelRefusal> {
    Ok(offset_solid(solid, distance, OffsetCorners::Sharp)?.solid)
}

/// The union of `a` and `b`.
pub fn union(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Union, &BooleanOptions::default())
}

/// `a` less `b`.
pub fn subtract(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Subtract, &BooleanOptions::default())
}

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

/// The faces of `solid` other than those with the points `openings` inside them, thickened by the
/// kernel's shell to a wall `thickness` thick, their joins extended until they meet. The kernel's
/// sign: a positive thickness moves the kept faces inward, into the material; a negative one
/// outward.
pub fn shell(solid: &BrepSolid, openings: &[Vec3], thickness: f64) -> Result<BrepSolid, Refusal> {
    let faces = openings.iter().map(|&q| face_at(solid, q)).collect::<Result<Vec<_>, _>>()?;
    Ok(offset_shell(solid, &faces, thickness)?.solid)
}

/// The sheet of one face on `surface`, bounded by the pcurve `loops` (the outer one first,
/// counter-clockwise in (u, v), then its holes clockwise) or by its whole domain where there are
/// none, thickened by the kernel by `thickness` to the side `side` gives at a point of it (the
/// thickener's own side is that of its normal Su × Sv, read here at the middle of the domain).
pub fn thicken(
    surface: &NurbsSurface,
    loops: &[Vec<NurbsCurve>],
    thickness: f64,
    side: impl Fn(Vec3) -> Vec3,
) -> Result<BrepSolid, Refusal> {
    let ([u0, u1], [v0, v1]) = (surface.domain_u()?, surface.domain_v()?);
    let (u, v) = (0.5 * (u0 + u1), 0.5 * (v0 + v1));
    let along = surface.normal(u, v)?.dot(side(surface.evaluate(u, v)?)) > 0.0;
    let signed = if along { thickness } else { -thickness };
    one(if loops.is_empty() {
        thicken_face_sheet(surface, signed, false)?
    } else {
        thicken_trimmed_sheet(surface, loops, signed, false)?
    })
}

/// The sheet as in [`thicken`], thickened by the kernel by `total` in all, half to each side.
pub fn thicken_both(
    surface: &NurbsSurface,
    loops: &[Vec<NurbsCurve>],
    total: f64,
) -> Result<BrepSolid, Refusal> {
    one(if loops.is_empty() {
        thicken_face_sheet(surface, total, true)?
    } else {
        thicken_trimmed_sheet(surface, loops, total, true)?
    })
}

/// The thickener's bodies as one solid: the body itself, or their union where it made several.
fn one(mut bodies: Vec<BrepSolid>) -> Result<BrepSolid, Refusal> {
    match bodies.len() {
        0 => Err("next.BREP's thickener made no body".into()),
        1 => Ok(bodies.remove(0)),
        _ => Ok(boolean_operation_nary(&bodies, BooleanOperation::Union)?),
    }
}

/// The angle bracket's sheet as faces of a solid: the box from the origin to (10, 8, 10), whose
/// floor (z = 0) and wall (x = 0) are the sheet's two faces, and the points inside its four other
/// faces, the openings a shell leaves.
pub fn bracket() -> Result<(BrepSolid, [Vec3; 4]), Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 8.0, 10.0)?;
    Ok((block, [p(5.0, 4.0, 10.0), p(10.0, 4.0, 5.0), p(5.0, 0.0, 5.0), p(5.0, 8.0, 5.0)]))
}

/// The open tray's sheet as faces of a solid: the cube from the origin to (10, 10, 10), whose
/// floor and four walls are the sheet's faces, and a point inside its top, the opening.
pub fn tray() -> Result<(BrepSolid, [Vec3; 1]), Refusal> {
    Ok((make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?, [p(5.0, 5.0, 10.0)]))
}

/// The bent flange's sheet as faces of a solid: its profile, the floor from (−10, 0) to (0, 0),
/// the quarter circle of radius 2 about (0, 2) to (2, 2) and the wall up to (2, 12) in the xz
/// plane, closed by the lines through (−10, 12) back to (−10, 0) and extruded 8 along +y. Its
/// floor, bend and wall are the sheet's three faces, the bend on the side towards its axis; the
/// points are inside its four other faces, the openings a shell leaves.
pub fn flange() -> Result<(BrepSolid, [Vec3; 4]), Refusal> {
    let profile = [
        make_line(p(-10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?,
        make_arc(p(0.0, 0.0, 2.0), p(0.0, 0.0, -1.0), p(1.0, 0.0, 0.0), 2.0, 0.0, FRAC_PI_2)?,
        make_line(p(2.0, 0.0, 2.0), p(2.0, 0.0, 12.0))?,
        make_line(p(2.0, 0.0, 12.0), p(-10.0, 0.0, 12.0))?,
        make_line(p(-10.0, 0.0, 12.0), p(-10.0, 0.0, 0.0))?,
    ];
    let body = extrude_profile_brep(&profile, p(0.0, 1.0, 0.0), 8.0)?;
    Ok((body, [p(-4.0, 4.0, 12.0), p(-10.0, 4.0, 6.0), p(-4.0, 0.0, 6.0), p(-4.0, 8.0, 6.0)]))
}
