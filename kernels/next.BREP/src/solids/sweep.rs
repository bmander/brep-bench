//! Helpers shared by the sweep, swept, loft, spline and nurbs cases: circles as the closed loops of
//! curves the kernel's profiles are, Bézier curves from their control points, rigid motions, a
//! wheel's sweep along a line, and the block standing under a NURBS patch.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    extrude_profile_brep, face_from_surface, face_from_surface_trimmed, make_arc, make_line,
    make_plane, revolve_profile_brep, solid_from_sheets, transform_brep, AffineTransform, BrepSolid,
    NurbsCurve, NurbsSurface, Vec3, Vec4,
};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The circle of `radius` about `centre` in the plane of the unit vectors `a` and `b`, starting at
/// `centre + radius·a` and running towards `b`: two half arcs, since a profile is a loop of at
/// least two curves.
pub fn circle(centre: Vec3, a: Vec3, b: Vec3, radius: f64) -> Result<Vec<NurbsCurve>, String> {
    Ok(vec![make_arc(centre, a, b, radius, 0.0, PI)?, make_arc(centre, a, b, radius, PI, TAU)?])
}

/// The Bézier curve of `points`, its degree one less than their number.
pub fn bezier(points: &[Vec3]) -> Result<NurbsCurve, String> {
    rational_bezier(points, &vec![1.0; points.len()])
}

/// The rational Bézier curve of `points` with `weights`.
pub fn rational_bezier(points: &[Vec3], weights: &[f64]) -> Result<NurbsCurve, String> {
    let degree = points.len() - 1;
    let knots = [vec![0.0; degree + 1], vec![1.0; degree + 1]].concat();
    let poles = points.iter().zip(weights).map(|(&q, &w)| Vec4::from_point(q, w)).collect();
    NurbsCurve::new(degree, knots, poles)
}

/// The rigid motion turning by `angle` (radians, right-handed) about the z axis, then moving by
/// `shift`.
pub fn turn_z(angle: f64, shift: Vec3) -> Result<AffineTransform, String> {
    let (s, c) = angle.sin_cos();
    AffineTransform::new([
        c, -s, 0.0, shift.x, //
        s, c, 0.0, shift.y, //
        0.0, 0.0, 1.0, shift.z, //
        0.0, 0.0, 0.0, 1.0,
    ])
}

/// `solid` moved by the rigid motion `motion`.
pub fn moved(solid: &BrepSolid, motion: AffineTransform) -> Result<BrepSolid, String> {
    transform_brep(solid, motion, false)
}

/// What a wheel sweeps as it slides without turning up the z axis by `rise`: a torus whose axis
/// is parallel to the y axis, centred at `centre`, the circle through its tube's centres of radius
/// `major` and its tube of radius `minor`.
///
/// Every vertical line meets the torus, if at all, between a lowest point and a highest one, which
/// lie at the same distance below and above the wheel's mid-plane z = `centre.z`; the gaps between
/// (across the wheel's hole) are at most 2·`major`, shorter than the slide. So along each such line
/// the wheel passes through everything from its lowest point at the start to its highest at the
/// end, and the sweep is the union of three solids: the wheel with its hole filled (everything
/// between the lowest and highest points: the revolve, about the wheel's axis, of its tube's half
/// disc joined to the axis by a rectangle) at the start and at the end, and between them its
/// outline seen along z (the stadium of the filled wheel's section by its mid-plane, the band
/// |y − centre.y| ≤ `minor` out to `major` + √(`minor`² − (y − centre.y)²) from the axis) slid
/// from the start's mid-plane to the end's. Returned in that order: start, slid outline, end.
pub fn torus_slid_up(centre: Vec3, major: f64, minor: f64, rise: f64)
    -> Result<Vec<BrepSolid>, Refusal> {
    let (x, y, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let at = |dx: f64, dy: f64| centre.add(x.scale(dx)).add(y.scale(dy));
    // the filled wheel's meridian, in the half plane through its axis towards +x
    let meridian = vec![
        make_line(at(0.0, minor), at(0.0, -minor))?,
        make_line(at(0.0, -minor), at(major, -minor))?,
        make_arc(at(major, 0.0), x, y, minor, -FRAC_PI_2, FRAC_PI_2)?,
        make_line(at(major, minor), at(0.0, minor))?,
    ];
    let filled = revolve_profile_brep(&meridian, centre, y, TAU)?;
    let lifted = moved(&filled, AffineTransform::new([
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, rise, //
        0.0, 0.0, 0.0, 1.0,
    ])?)?;
    // the outline: the band's two straight sides and its two round ends, in the plane z = centre.z
    let outline = vec![
        make_line(at(-major, -minor), at(major, -minor))?,
        make_arc(at(major, 0.0), x, y, minor, -FRAC_PI_2, FRAC_PI_2)?,
        make_line(at(major, minor), at(-major, minor))?,
        make_arc(at(-major, 0.0), x, y, minor, FRAC_PI_2, 3.0 * FRAC_PI_2)?,
    ];
    let slid = extrude_profile_brep(&outline, z, rise)?;
    Ok(vec![filled, slid, lifted])
}

/// The block standing on the floor z = 0 under `top` ([`block_to`]).
pub fn block_under(top: &NurbsSurface) -> Result<BrepSolid, Refusal> {
    block_to(top, 0.0)
}

/// The block between `patch` and the plane z = `level` (below the patch or above it), a patch
/// whose four sides lie in the planes of the block's four upright walls: the patch as a face of
/// its own (`face_from_surface`), the flat end and the four walls each a flat face bounded by its
/// edges (`face_from_surface_trimmed`: a wall's are its edge in the flat end, its two uprights and
/// the patch's side), and the six joined into the solid they bound (`solid_from_sheets`).
pub fn block_to(patch: &NurbsSurface, level: f64) -> Result<BrepSolid, Refusal> {
    let [u0, u1] = patch.domain_u()?;
    let [v0, v1] = patch.domain_v()?;
    // the patch's corners in turn, and its side from each to the next
    let on = [patch.evaluate(u0, v0)?, patch.evaluate(u1, v0)?, patch.evaluate(u1, v1)?,
        patch.evaluate(u0, v1)?];
    let sides = [patch.iso_curve_v(v0)?, patch.iso_curve_u(u1)?, patch.iso_curve_v(v1)?.reversed()?,
        patch.iso_curve_u(u0)?.reversed()?];
    let flat: Vec<Vec3> = on.iter().map(|q| p(q.x, q.y, level)).collect();
    // the walls' planes run from the flat end past the patch's farthest control point
    let reach = |q: &Vec4| (q.z / q.w - level).abs();
    let height = patch.control_points.iter().flatten().map(reach).fold(0.0, f64::max) + 1.0;
    let toward = p(0.0, 0.0, (on[0].z - level).signum());
    let (across, along) = (flat[1].sub(flat[0]), flat[3].sub(flat[0]));
    let unit = |d: Vec3| d.scale(1.0 / d.length());
    let end = make_plane(flat[0], unit(across), unit(along), across.length(), along.length())?;
    let mut sheets =
        vec![face_from_surface(patch)?, face_from_surface_trimmed(&end, &[polygon(&flat)?])?];
    for k in 0..4 {
        let next = (k + 1) % 4;
        let run = flat[next].sub(flat[k]);
        let wall = make_plane(flat[k], unit(run), toward, run.length(), height)?;
        let edges = vec![make_line(flat[k], flat[next])?, make_line(flat[next], on[next])?,
            sides[k].reversed()?, make_line(on[k], flat[k])?];
        sheets.push(face_from_surface_trimmed(&wall, &[edges])?);
    }
    Ok(solid_from_sheets(&sheets, 1e-7)?)
}
