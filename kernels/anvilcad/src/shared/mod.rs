//! Helpers shared by the case files: anvil-brep's errors in words, frames, profiles, and the
//! primitives a user of AnvilCAD would sketch and extrude or revolve, the way AnvilCAD's own
//! kernel evaluations build them (`crates/anvil-evals/src/csg.rs`): a block or a cylinder is an
//! extrusion, a cone, a ball or a torus a revolution.
#![allow(dead_code)]

use std::f64::consts::{PI, TAU};

use anvil_brep::boolean::{BoolOp, boolean};
use anvil_brep::profile::{Loop2, Region, Segment, build_regions};
use anvil_brep::{Frame, KernelError, Solid, extrude, revolve};
use glam::{DVec2, DVec3};

/// A kernel error as the reason a case gives.
pub fn e(err: KernelError) -> String {
    err.to_string()
}

pub fn v2(x: f64, y: f64) -> DVec2 {
    DVec2::new(x, y)
}

pub fn v3(x: f64, y: f64, z: f64) -> DVec3 {
    DVec3::new(x, y, z)
}

/// The frame at `origin` whose normal is `z` and whose first axis is along `x`.
pub fn frame(origin: [f64; 3], z: [f64; 3], x: [f64; 3]) -> Result<Frame, String> {
    Frame::new(
        DVec3::from_array(origin),
        DVec3::from_array(z),
        DVec3::from_array(x),
    )
    .map_err(e)
}

/// The world frame moved to `origin`.
pub fn at(origin: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from_array(origin),
        ..Frame::world()
    }
}

/// The xz plane through `origin`, its sketch x along world x and its sketch y along world z (its
/// normal is −y). A profile drawn here and revolved about its sketch y axis turns about world z.
pub fn xz(origin: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from_array(origin),
        x: DVec3::X,
        y: DVec3::Z,
        z: DVec3::NEG_Y,
    }
}

pub fn line(a: DVec2, b: DVec2) -> Segment {
    Segment::Line { a, b }
}

/// The closed polygon through `points`, in order.
pub fn polygon(points: &[(f64, f64)]) -> Loop2 {
    let p: Vec<DVec2> = points.iter().map(|&(x, y)| v2(x, y)).collect();
    Loop2::new(
        (0..p.len())
            .map(|i| line(p[i], p[(i + 1) % p.len()]))
            .collect(),
    )
}

/// The rectangle from `lo` to `hi`.
pub fn rect(lo: (f64, f64), hi: (f64, f64)) -> Loop2 {
    polygon(&[lo, (hi.0, lo.1), hi, (lo.0, hi.1)])
}

/// The circle of `radius` about `center`.
pub fn circle(center: (f64, f64), radius: f64) -> Loop2 {
    Loop2::new(vec![Segment::circle(v2(center.0, center.1), radius)])
}

/// The one region the contours bound (an outer contour and the holes in it).
pub fn region(loops: &[Loop2]) -> Result<Region, String> {
    let mut r = build_regions(loops).map_err(e)?;
    match r.len() {
        1 => Ok(r.remove(0)),
        n => Err(format!("the profile makes {n} regions, not one")),
    }
}

/// What a case answers when the kernel returned no solid at all and called that a success: not a
/// refusal, but its answer, and the runner hands it on as an empty result (see `main.rs`).
pub const NOTHING: &str = "anvil-brep returned no solid";

/// The one solid an operation left. anvil-brep's booleans and some features return a list of
/// bodies; a case describes one solid. None at all is the kernel's answer, handed on as such.
pub fn one(mut bodies: Vec<Solid>) -> Result<Solid, String> {
    match bodies.len() {
        1 => Ok(bodies.remove(0)),
        0 => Err(NOTHING.into()),
        n => Err(format!("the operation left {n} separate solids, not one")),
    }
}

pub fn union(a: &Solid, b: &Solid) -> Result<Solid, String> {
    one(boolean(a, b, BoolOp::Union).map_err(e)?)
}

/// `a` less `b`.
pub fn subtract(a: &Solid, b: &Solid) -> Result<Solid, String> {
    one(boolean(a, b, BoolOp::Subtract).map_err(e)?)
}

pub fn intersect(a: &Solid, b: &Solid) -> Result<Solid, String> {
    one(boolean(a, b, BoolOp::Intersect).map_err(e)?)
}

/// The block from `lo` to `hi`: its xy rectangle extruded from `lo.z` to `hi.z`.
pub fn block(lo: [f64; 3], hi: [f64; 3]) -> Result<Solid, String> {
    let r = region(&[rect((lo[0], lo[1]), (hi[0], hi[1]))])?;
    extrude::extrude(&Frame::world(), &r, lo[2], hi[2]).map_err(e)
}

/// The cylinder of `radius` about `frame.z`, from `z0` to `z1` along it: a circle extruded.
pub fn cylinder(frame: &Frame, radius: f64, z0: f64, z1: f64) -> Result<Solid, String> {
    let r = region(&[circle((0., 0.), radius)])?;
    extrude::extrude(frame, &r, z0, z1).map_err(e)
}

/// A profile drawn with x as the distance from `frame.z` and y along it, revolved a full turn
/// about `frame.z`.
pub fn turned(frame: &Frame, profile: Loop2) -> Result<Solid, String> {
    // the profile's plane holds the radial direction and the axis
    let plane = Frame {
        origin: frame.origin,
        x: frame.x,
        y: frame.z,
        z: -frame.y,
    };
    revolve::revolve(&plane, &region(&[profile])?, DVec2::ZERO, DVec2::Y, TAU).map_err(e)
}

/// The cone (or frustum) on `frame.z` from radius `r0` at its origin to `r1` at `height` along it.
pub fn cone(frame: &Frame, r0: f64, r1: f64, height: f64) -> Result<Solid, String> {
    let mut pts = vec![(0., 0.)];
    if r0 > 0. {
        pts.push((r0, 0.));
    }
    if r1 > 0. {
        pts.push((r1, height));
    }
    pts.push((0., height));
    turned(frame, polygon(&pts))
}

/// The ball of `radius` about `center`: a half disc revolved about its diameter.
pub fn ball(center: [f64; 3], radius: f64) -> Result<Solid, String> {
    let half = Loop2::new(vec![
        Segment::Arc {
            center: DVec2::ZERO,
            radius,
            start: -0.5 * PI,
            sweep: PI,
        },
        line(v2(0., radius), v2(0., -radius)),
    ]);
    turned(&at(center), half)
}

/// The ring torus about `frame.z`: a circle of radius `minor`, `major` from the axis, revolved.
pub fn torus(frame: &Frame, major: f64, minor: f64) -> Result<Solid, String> {
    turned(frame, circle((major, 0.), minor))
}

/// The cylinder of `radius` whose axis runs from `a` to `b`.
pub fn rod(a: [f64; 3], b: [f64; 3], radius: f64) -> Result<Solid, String> {
    let (a, b) = (DVec3::from_array(a), DVec3::from_array(b));
    let axis = b - a;
    let hint = if axis.cross(DVec3::Z).length() > 1e-9 * axis.length() {
        DVec3::Z
    } else {
        DVec3::X
    };
    let f = Frame::new(a, axis, hint.cross(axis)).map_err(e)?;
    cylinder(&f, radius, 0., axis.length())
}

/// `body` with each of `tools` cut from it, one cut after another.
pub fn cut_all(
    body: Solid,
    tools: impl IntoIterator<Item = Result<Solid, String>>,
) -> Result<Solid, String> {
    tools.into_iter().try_fold(body, |b, t| subtract(&b, &t?))
}

/// `first` united with each of `rest`, one after another.
pub fn unite_all(
    first: Solid,
    rest: impl IntoIterator<Item = Result<Solid, String>>,
) -> Result<Solid, String> {
    rest.into_iter().try_fold(first, |b, t| union(&b, &t?))
}

/// The rigid turn by `degrees` about the z axis, as a frame to move solids by.
pub fn turn_z(degrees: f64) -> Frame {
    let (s, c) = degrees.to_radians().sin_cos();
    Frame {
        origin: DVec3::ZERO,
        x: v3(c, s, 0.),
        y: v3(-s, c, 0.),
        z: DVec3::Z,
    }
}

/// Separate bodies united with one more solid, as AnvilCAD's own evaluations unite a tool with a
/// part of several bodies: the tool merges with each body it joins, and the rest are kept.
pub fn unite_into(bodies: Vec<Solid>, tool: Solid) -> Result<Vec<Solid>, String> {
    let mut merged = tool;
    let mut rest = Vec::new();
    for body in bodies {
        let mut r = boolean(&body, &merged, BoolOp::Union).map_err(e)?;
        if r.len() == 1 {
            merged = r.remove(0);
        } else {
            rest.push(body);
        }
    }
    rest.push(merged);
    Ok(rest)
}

/// The solid with the edges through `points` blended, as AnvilCAD's own fillet and chamfer
/// features drive the kernel (`anvil-model`'s `exact::blend`): an edge is picked by a point on
/// it, the picked edges that run smoothly one into the next are blended together in one step so
/// their blends join up, and the edges are found afresh after each step, since blending
/// renumbers the solid.
pub fn blend(
    solid: &Solid,
    points: &[[f64; 3]],
    kind: anvil_brep::blend::Blend,
) -> Result<Solid, String> {
    use anvil_brep::blend::{blend_chain, edge_near, smooth_run};
    let mut solid = solid.clone();
    let mut left: Vec<DVec3> = points.iter().map(|p| DVec3::from_array(*p)).collect();
    while !left.is_empty() {
        let picked = left
            .iter()
            .map(|p| {
                edge_near(&solid, *p, 1e-6).ok_or_else(|| format!("no edge passes through {p}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let run = smooth_run(&solid, picked[0], &picked);
        let tagged: Vec<_> = run.iter().map(|id| (*id, 0)).collect();
        solid = one(blend_chain(&solid, &tagged, kind, 1).map_err(e)?)?;
        left = left
            .into_iter()
            .zip(&picked)
            .filter(|(_, id)| !run.contains(id))
            .map(|(p, _)| p)
            .collect();
    }
    Ok(solid)
}

/// The solid with the edges through `points` rounded to `radius`.
pub fn fillet(solid: &Solid, points: &[[f64; 3]], radius: f64) -> Result<Solid, String> {
    blend(solid, points, anvil_brep::blend::Blend::Fillet(radius))
}

/// The solid with the edges through `points` chamfered `distance` back along both faces.
pub fn chamfer(solid: &Solid, points: &[[f64; 3]], distance: f64) -> Result<Solid, String> {
    blend(solid, points, anvil_brep::blend::Blend::Chamfer(distance))
}

/// The prism from `z0` to `z1` on the polygon through `points` in the xy plane.
pub fn prism(points: &[(f64, f64)], z0: f64, z1: f64) -> Result<Solid, String> {
    extrude::extrude(&Frame::world(), &region(&[polygon(points)])?, z0, z1).map_err(e)
}

/// The prism from `y0` to `y1` along y on the polygon through `points`, given as (x, z).
pub fn prism_xz(points: &[(f64, f64)], y0: f64, y1: f64) -> Result<Solid, String> {
    // the xz plane's normal is −y, so +y is its −z
    extrude::extrude(&xz([0., 0., 0.]), &region(&[polygon(points)])?, -y1, -y0).map_err(e)
}

/// The pyramid on the polygon `base` (in the xy plane at z = 0) whose apex is `apex`: a ruled loft
/// from the polygon to the point.
pub fn pyramid(base: &[(f64, f64)], apex: [f64; 3]) -> Result<Solid, String> {
    use anvil_brep::loft::{Section, loft_ruled};
    let a = Section::Loop {
        frame: Frame::world(),
        outline: polygon(base),
    };
    loft_ruled(&a, &Section::Point(DVec3::from_array(apex)), [None, None]).map_err(e)
}

/// The octahedron |x| + |y| + |z| ≤ `r`: two square pyramids on the square of its middle.
pub fn octahedron(r: f64) -> Result<Solid, String> {
    let square = [(r, 0.), (0., r), (-r, 0.), (0., -r)];
    union(
        &pyramid(&square, [0., 0., r])?,
        &pyramid(&square, [0., 0., -r])?,
    )
}

/// A point on the circle of `radius` about the z axis at height `z`, at 100° round from +x: away
/// from where the seams of revolved and extruded round faces lie, so that it picks the circle.
pub fn ring(radius: f64, z: f64) -> [f64; 3] {
    let (s, c) = 100f64.to_radians().sin_cos();
    [radius * c, radius * s, z]
}

/// Everything on the side of the plane through `origin` that `normal` points to, out to `reach`:
/// a block standing on the plane, to cut a solid by it.
pub fn beyond(origin: [f64; 3], normal: [f64; 3], reach: f64) -> Result<Solid, String> {
    let n = DVec3::from_array(normal);
    let hint = if n.cross(DVec3::Z).length() > 1e-9 * n.length() {
        DVec3::Z
    } else {
        DVec3::X
    };
    let f = Frame::new(DVec3::from_array(origin), n, hint.cross(n)).map_err(e)?;
    extrude::extrude(
        &f,
        &region(&[rect((-reach, -reach), (reach, reach))])?,
        0.,
        reach,
    )
    .map_err(e)
}

/// The index of the face of `solid` that the point `p` lies on.
pub fn face_at(solid: &Solid, p: [f64; 3]) -> Result<usize, String> {
    let p = DVec3::from_array(p);
    (0..solid.faces.len())
        .find(|&f| anvil_brep::boolean::face_contains(solid, f, p))
        .ok_or_else(|| format!("no face holds {p}"))
}

/// The solid with the edge through `edge` chamfered `distance` back along the face through
/// `along`, the chamfer leaving that face at `degrees`.
pub fn chamfer_angle(
    solid: &Solid,
    edge: [f64; 3],
    along: [f64; 3],
    distance: f64,
    degrees: f64,
) -> Result<Solid, String> {
    let kind = anvil_brep::blend::Blend::ChamferAngle {
        distance,
        angle: degrees.to_radians(),
        along: face_at(solid, along)?,
    };
    blend(solid, &[edge], kind)
}

/// The ball of `radius` about `frame.origin`, revolved about `frame.z`. anvil-brep's spheres are
/// laid out stereographically and have no poles; the frame only says where the revolved half disc
/// turns.
pub fn ball_about(frame: &Frame, radius: f64) -> Result<Solid, String> {
    let half = Loop2::new(vec![
        Segment::Arc {
            center: DVec2::ZERO,
            radius,
            start: -0.5 * PI,
            sweep: PI,
        },
        line(v2(0., radius), v2(0., -radius)),
    ]);
    turned(frame, half)
}

/// The solid with the faces through `points` drafted by `degrees` about the neutral plane through
/// `origin`, pulling along `pull`: a positive angle closes the faces in on the body as they go in
/// the direction of pull.
pub fn draft(
    solid: &Solid,
    points: &[[f64; 3]],
    origin: [f64; 3],
    pull: [f64; 3],
    degrees: f64,
) -> Result<Solid, String> {
    let faces = points
        .iter()
        .map(|p| face_at(solid, *p))
        .collect::<Result<Vec<_>, _>>()?;
    let n = DVec3::from_array(pull);
    let hint = if n.cross(DVec3::Z).length() > 1e-9 * n.length() {
        DVec3::Z
    } else {
        DVec3::X
    };
    let neutral = Frame::new(DVec3::from_array(origin), n, hint.cross(n)).map_err(e)?;
    anvil_brep::draft::draft_faces(solid, &faces, &neutral, degrees.to_radians()).map_err(e)
}

/// The rectangle from `lo` to `hi` with its corners rounded to `r`, as a closed loop.
pub fn rounded_rect(lo: (f64, f64), hi: (f64, f64), r: f64) -> Loop2 {
    let h = 0.5 * PI;
    let arc = |cx: f64, cy: f64, start: f64| Segment::Arc {
        center: v2(cx, cy),
        radius: r,
        start,
        sweep: h,
    };
    Loop2::new(vec![
        line(v2(lo.0 + r, lo.1), v2(hi.0 - r, lo.1)),
        arc(hi.0 - r, lo.1 + r, -h),
        line(v2(hi.0, lo.1 + r), v2(hi.0, hi.1 - r)),
        arc(hi.0 - r, hi.1 - r, 0.),
        line(v2(hi.0 - r, hi.1), v2(lo.0 + r, hi.1)),
        arc(lo.0 + r, hi.1 - r, h),
        line(v2(lo.0, hi.1 - r), v2(lo.0, lo.1 + r)),
        arc(lo.0 + r, lo.1 + r, 2. * h),
    ])
}

/// The points in the middle of each side of the rounded rectangle `rounded_rect(lo, hi, r)`, at
/// height `z`: four on its straight sides and four on its quarter circles.
pub fn rounded_rect_sides(lo: (f64, f64), hi: (f64, f64), r: f64, z: f64) -> Vec<[f64; 3]> {
    let (mx, my) = (0.5 * (lo.0 + hi.0), 0.5 * (lo.1 + hi.1));
    let d = r * std::f64::consts::FRAC_1_SQRT_2;
    vec![
        [mx, lo.1, z],
        [hi.0, my, z],
        [mx, hi.1, z],
        [lo.0, my, z],
        [hi.0 - r + d, lo.1 + r - d, z],
        [hi.0 - r + d, hi.1 - r + d, z],
        [lo.0 + r - d, hi.1 - r + d, z],
        [lo.0 + r - d, lo.1 + r - d, z],
    ]
}

/// `solid` shelled inward to a wall of `thickness`, open at the faces through `open`.
pub fn shell_open(solid: &Solid, open: &[[f64; 3]], thickness: f64) -> Result<Vec<Solid>, String> {
    let faces = open
        .iter()
        .map(|p| face_at(solid, *p))
        .collect::<Result<Vec<_>, _>>()?;
    anvil_brep::shell::shell(solid, &faces, thickness).map_err(e)
}

/// `solid` shelled outward to a wall of `thickness`, open at the faces through `open`, its edges
/// kept sharp. anvil-brep's shell only hollows inward, so the wall is built as its shell builds
/// one, from its own offset and boolean: every face but the openings moved `thickness` outward,
/// the moved faces extended until they meet and cut off by the openings' surfaces, less the solid.
pub fn shell_outward(
    solid: &Solid,
    open: &[[f64; 3]],
    thickness: f64,
) -> Result<Vec<Solid>, String> {
    let faces = open
        .iter()
        .map(|p| face_at(solid, *p))
        .collect::<Result<Vec<_>, _>>()?;
    let distance: Vec<f64> = (0..solid.faces.len())
        .map(|f| if faces.contains(&f) { 0. } else { thickness })
        .collect();
    let grown = anvil_brep::offset::offset_faces(solid, &distance).map_err(e)?;
    boolean(&grown, solid, BoolOp::Subtract).map_err(e)
}

/// The solid of revolution about the z axis whose half section, in a plane through it, is the
/// closed loop of `segments` (x the distance from the axis, y the height).
pub fn revolved(segments: Vec<Segment>) -> Result<Solid, String> {
    turned(&Frame::world(), Loop2::new(segments))
}

/// `solid` with every face moved `distance` along its outward normal (negative moves it in), the
/// moved faces extended or trimmed until they meet.
pub fn offset(solid: &Solid, distance: f64) -> Result<Solid, String> {
    anvil_brep::offset::offset_faces(solid, &vec![distance; solid.faces.len()]).map_err(e)
}

/// `solid` with each face through one of `moves`' points moved its distance along its outward
/// normal (negative moves it in), the faces beside it trimmed or extended to meet it: anvil-brep's
/// direct edit of faces.
pub fn move_faces(solid: &Solid, moves: &[([f64; 3], f64)]) -> Result<Solid, String> {
    let mut distance = vec![0.; solid.faces.len()];
    for (p, d) in moves {
        distance[face_at(solid, *p)?] = *d;
    }
    anvil_brep::offset::offset_faces(solid, &distance).map_err(e)
}

/// The zonotope `center + Σ tₖ gₖ`, every tₖ from −½ to ½: what a box (three generators, its edges)
/// fills as it slides along a fourth, the slide. It is the block that holds it cut by its facets'
/// planes: square to each pair of generators, at the support `n·center + ½ Σ |n·gₖ|`.
pub fn zonotope(center: DVec3, generators: &[DVec3]) -> Result<Solid, String> {
    let half: DVec3 = generators.iter().map(|g| g.abs()).sum::<DVec3>() * 0.5;
    let reach = 4. * half.length() + 1.;
    let mut cuts = Vec::new();
    for i in 0..generators.len() {
        for j in i + 1..generators.len() {
            let Some(n) = generators[i].cross(generators[j]).try_normalize() else {
                continue;
            };
            let h = 0.5 * generators.iter().map(|g| n.dot(*g).abs()).sum::<f64>();
            for s in [n, -n] {
                cuts.push(beyond((center + s * h).to_array(), s.to_array(), reach));
            }
        }
    }
    cut_all(
        block((center - half).to_array(), (center + half).to_array())?,
        cuts,
    )
}

/// The region of a half-plane through the z axis given by `segments` (x the distance from the
/// axis, y the height), turned about the z axis from `from` degrees by `by` degrees, right-handed
/// (from +x toward +y).
pub fn swung(segments: Vec<Segment>, from: f64, by: f64) -> Result<Solid, String> {
    // in the xz plane a negative turn about its sketch y (world z) runs from +x toward +y
    let s = revolve::revolve(
        &xz([0., 0., 0.]),
        &region(&[Loop2::new(segments)])?,
        DVec2::ZERO,
        DVec2::Y,
        -by.to_radians(),
    )
    .map_err(e)?;
    Ok(s.transformed(&turn_z(from)))
}

/// The stadium of every point within `r` of the segment from `a` to `b`: two half circles joined
/// by straight sides, as a closed loop.
pub fn stadium(a: (f64, f64), b: (f64, f64), r: f64) -> Loop2 {
    let (a, b) = (v2(a.0, a.1), v2(b.0, b.1));
    let d = (b - a).normalize();
    let n = d.perp() * r;
    let start = (-n).to_angle();
    Loop2::new(vec![
        line(a - n, b - n),
        Segment::Arc {
            center: b,
            radius: r,
            start,
            sweep: PI,
        },
        line(b + n, a + n),
        Segment::Arc {
            center: a,
            radius: r,
            start: start + PI,
            sweep: PI,
        },
    ])
}
