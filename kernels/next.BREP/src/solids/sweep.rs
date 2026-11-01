//! Helpers shared by the sweep, swept, loft, spline and nurbs cases: circles as the closed loops of
//! curves the kernel's profiles are, Bézier curves from their control points, rigid motions, a
//! wheel's sweep along a line, and the block standing under a NURBS patch.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    accept_sound, extrude_profile_brep, make_arc, make_plane, revolve_profile_brep, transform_brep,
    AffineTransform, BrepSolid, NurbsCurve, NurbsSurface, Vec3, Vec4,
};
use serde_json::{json, Value};
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

/// The block standing on the floor z = 0 under `top`, a clamped NURBS patch whose x depends on u
/// alone and y on v alone, each rising along its parameter, so that its four edges, the iso-curves
/// at the ends of its domain, lie in the planes of the block's four upright sides.
///
/// next.BREP has no operation that makes a solid from a NURBS surface, so the block is assembled
/// from the kernel's own topology records (the public data model of `BrepSolid`): eight vertices,
/// twelve edges, and six faces, the patch on top and five planes (`make_plane`) below. The top's
/// edges are its boundary iso-curves, which for a clamped patch are its control net's outer rows
/// and columns; each face trims its surface by the edges' images in its own parameters (lines in
/// the patch's, the edges' affine images in a plane's). The body is then put through the kernel's
/// own soundness acceptance.
pub fn block_under(top: &NurbsSurface) -> Result<BrepSolid, Refusal> {
    let net = &top.control_points;
    let (rows, columns) = (net.len(), net[0].len());
    let [u0, u1] = top.domain_u()?;
    let [v0, v1] = top.domain_v()?;
    // the top's four boundary iso-curves: along u at v0 and at v1, along v at u0 and at u1
    let along_u = |j: usize| -> Result<NurbsCurve, String> {
        NurbsCurve::new(top.degree_u, top.knots_u.clone(), (0..rows).map(|i| net[i][j]).collect())
    };
    let along_v = |i: usize| -> Result<NurbsCurve, String> {
        NurbsCurve::new(top.degree_v, top.knots_v.clone(), net[i].clone())
    };
    let (ab_top, dc_top) = (along_u(0)?, along_u(columns - 1)?);
    let (ad_top, bc_top) = (along_v(0)?, along_v(rows - 1)?);
    // the corners: a, b, c, d on the floor counter-clockwise from x and y least, A … D above
    let up = [top.evaluate(u0, v0)?, top.evaluate(u1, v0)?, top.evaluate(u1, v1)?,
        top.evaluate(u0, v1)?];
    let floor: Vec<Vec3> = up.iter().map(|q| Vec3::new(q.x, q.y, 0.0)).collect();
    let corners: Vec<Vec3> = floor.iter().chain(up.iter()).copied().collect();
    let (a, b, c, d, ta, tb, tc, td) = (0, 1, 2, 3, 4, 5, 6, 7);
    let (x0, y0, x1, y1) = (floor[a].x, floor[a].y, floor[c].x, floor[c].y);
    // the sides reach above the patch's highest control point
    let height = net.iter().flatten().map(|q| q.z / q.w).fold(0.0, f64::max) + 1.0;

    // the twelve edges, each from its first vertex to its second
    let mut edges: Vec<(NurbsCurve, usize, usize)> = Vec::new();
    for (from, to) in [(a, b), (b, c), (c, d), (d, a), (a, ta), (b, tb), (c, tc), (d, td)] {
        edges.push((make_line(corners[from], corners[to])?, from, to));
    }
    edges.push((ab_top, ta, tb));
    edges.push((bc_top, tb, tc));
    edges.push((dc_top, td, tc));
    edges.push((ad_top, ta, td));
    let (ab, bc, cd, da, aa, bb, cc, dd) = (0, 1, 2, 3, 4, 5, 6, 7);
    let (tab, tbc, tdc, tad) = (8, 9, 10, 11);

    let mut next = 100u64;
    let mut id = || {
        next += 1;
        next
    };
    let vertex_ids: Vec<u64> = (0..8).map(|_| id()).collect();
    let edge_ids: Vec<u64> = (0..12).map(|_| id()).collect();

    // a face on `surface`, its normal outward, bounded by one loop of (edge, forward) uses, each
    // trimmed by `pcurve`, the edge's image in the surface's parameters along the edge's own
    // parameter
    let mut face = |surface: &NurbsSurface, uses: &[(usize, bool)],
                    pcurve: &dyn Fn(usize) -> Result<NurbsCurve, String>|
     -> Result<Value, Refusal> {
        let mut coedges = Vec::new();
        for &(edge, forward) in uses {
            let image = pcurve(edge)?;
            let image = if forward { image } else { image.reversed()? };
            coedges.push(json!({"id": id(), "edge_id": edge_ids[edge], "forward": forward,
                "pcurve": serde_json::to_value(&image).map_err(|e| e.to_string())?}));
        }
        Ok(json!({"id": id(), "surface": serde_json::to_value(surface).map_err(|e| e.to_string())?,
            "same_sense": true, "loops": [{"id": id(), "coedges": coedges}]}))
    };
    // a plane through `origin` with unit directions `s` and `t` (s × t outward), and the image in
    // its parameters of every edge lying in it
    let plane = |origin: Vec3, s: Vec3, t: Vec3, width: f64| -> Result<NurbsSurface, String> {
        make_plane(origin, s, t, width, height)
    };
    let image_in = |origin: Vec3, s: Vec3, t: Vec3| {
        let curves: Vec<NurbsCurve> = edges.iter().map(|e| e.0.clone()).collect();
        move |edge: usize| -> Result<NurbsCurve, String> {
            let curve = &curves[edge];
            let poles = curve.control_points.iter().map(|q| {
                let at = Vec3::new(q.x / q.w, q.y / q.w, q.z / q.w).sub(origin);
                Vec4::from_point(Vec3::new(at.dot(s), at.dot(t), 0.0), q.w)
            });
            NurbsCurve::new(curve.degree, curve.knots.clone(), poles.collect())
        }
    };
    let (ex, ey, ez) =
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let neg = |v: Vec3| v.scale(-1.0);
    let (w, h) = (x1 - x0, y1 - y0);
    let mut faces = Vec::new();
    // the floor, facing down: its parameters along y and x from a
    let o = corners[a];
    let floor_plane = make_plane(o, ey, ex, h, w)?;
    faces.push(face(&floor_plane, &[(da, false), (cd, false), (bc, false), (ab, false)],
        &image_in(o, ey, ex))?);
    // the four sides, each facing out, its parameters along the floor's edge and up
    let o = corners[a];
    faces.push(face(&plane(o, ex, ez, w)?, &[(ab, true), (bb, true), (tab, false), (aa, false)],
        &image_in(o, ex, ez))?);
    let o = corners[b];
    faces.push(face(&plane(o, ey, ez, h)?, &[(bc, true), (cc, true), (tbc, false), (bb, false)],
        &image_in(o, ey, ez))?);
    let o = corners[c];
    faces.push(face(&plane(o, neg(ex), ez, w)?, &[(cd, true), (dd, true), (tdc, true), (cc, false)],
        &image_in(o, neg(ex), ez))?);
    let o = corners[d];
    faces.push(face(&plane(o, neg(ey), ez, h)?, &[(da, true), (aa, true), (tad, true), (dd, false)],
        &image_in(o, neg(ey), ez))?);
    // the top, its normal x × y up: each edge's image a line in (u, v) along the edge's parameter
    let uv_line = |from: [f64; 2], to: [f64; 2], [t0, t1]: [f64; 2]| {
        NurbsCurve::new(1, vec![t0, t0, t1, t1], vec![
            Vec4::from_point(Vec3::new(from[0], from[1], 0.0), 1.0),
            Vec4::from_point(Vec3::new(to[0], to[1], 0.0), 1.0),
        ])
    };
    let top_image = |edge: usize| -> Result<NurbsCurve, String> {
        match edge {
            e if e == tab => uv_line([u0, v0], [u1, v0], [u0, u1]),
            e if e == tbc => uv_line([u1, v0], [u1, v1], [v0, v1]),
            e if e == tdc => uv_line([u0, v1], [u1, v1], [u0, u1]),
            _ => uv_line([u0, v0], [u0, v1], [v0, v1]),
        }
    };
    faces.push(face(top, &[(tab, true), (tbc, true), (tdc, false), (tad, false)], &top_image)?);

    let vertices: Vec<Value> = (0..8)
        .map(|k| json!({"id": vertex_ids[k], "point": {"x": corners[k].x, "y": corners[k].y,
            "z": corners[k].z}}))
        .collect();
    let mut edge_records = Vec::new();
    for (k, (curve, from, to)) in edges.iter().enumerate() {
        let [t0, t1] = curve.domain()?;
        edge_records.push(json!({"id": edge_ids[k],
            "curve": serde_json::to_value(curve).map_err(|e| e.to_string())?, "t0": t0, "t1": t1,
            "start_vertex_id": vertex_ids[*from], "end_vertex_id": vertex_ids[*to]}));
    }
    let body = json!({"id": id(), "vertices": vertices, "edges": edge_records,
        "shells": [{"id": id(), "faces": faces}], "genus": 0});
    let solid: BrepSolid = serde_json::from_value(body).map_err(|e| e.to_string())?;
    let issues = solid.validate();
    if let Some(issue) = issues.iter().find(|issue| issue.severity == "error") {
        return Err(Refusal::from(format!("the assembled block is not valid: {issue:?}")));
    }
    Ok(accept_sound(solid, "block_under")?)
}


/// Everything within a guide of the z axis as seen along `across` (the unit x or y vector): the
/// region, in the vertical plane through the z axis square to `across`, between the Bézier curve
/// of `seen` (the guide's control points with their `across` component dropped) and its mirror
/// image in the z axis, closed by straight segments along the floor and, unless the two meet
/// there, along the top; slid along `across` from −`reach` to `reach`.
pub fn between_guides(seen: &[Vec3], across: Vec3, reach: f64) -> Result<BrepSolid, Refusal> {
    // the mirror in the z axis, within that plane: x and y negated
    let mirror: Vec<Vec3> = seen.iter().map(|q| Vec3::new(-q.x, -q.y, q.z)).collect();
    let back = across.scale(-reach);
    let place = |q: &Vec3| q.add(back);
    let guide: Vec<Vec3> = seen.iter().map(place).collect();
    let image: Vec<Vec3> = mirror.iter().rev().map(place).collect();
    let (top, foot) = (guide.len() - 1, 0);
    let mut region = vec![bezier(&guide)?];
    if guide[top].sub(image[0]).length() > 1e-12 {
        region.push(make_line(guide[top], image[0])?);
    }
    region.push(bezier(&image)?);
    region.push(make_line(image[image.len() - 1], guide[foot])?);
    Ok(extrude_profile_brep(&region, across, 2.0 * reach)?)
}
