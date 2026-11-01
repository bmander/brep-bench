//! A solid arris has no primitive for, made the arris way and shaped like an arris operation.
use crate::Refusal;
use arris::check::classify::{classify_point, Classification};
use arris::geom::{
    pcurve_on, Curve, Curve2, GeomError, NurbsCurve, NurbsCurve2, NurbsSurface, Profile,
    ProfileLoop, ProfileSegment, Surface,
};
use arris::math::{
    Axis, Control, Frame, Interval, Meter, Point2, Point3, UnitVec2, UnitVec3, Vec2, Vec3,
};
use arris::ops::{build, revolve, BuildKeys, OpError};
use arris::topo::builder::{
    Assembly, Builder, EdgeKey, EdgeSpec, FaceSpec, UseSpec, VertexKey, VertexSpec,
};
use arris::topo::entity::EdgeGeometry;
use arris::topo::provenance::Provenance;
use arris::topo::{Body, Edge, Face, Model, Orientation};
use std::collections::BTreeMap;
use std::f64::consts::TAU;

/// A ball of `radius` about `centre`: the half disc of that radius in the vertical plane through
/// `centre`, its diameter on the vertical axis, turned a full turn about that axis.
pub fn ball(
    m: &mut Model,
    centre: Point3,
    radius: f64,
    control: &Control<'_>,
) -> Result<(Body, Provenance), OpError> {
    // the plane through the centre whose u runs along x and v along z
    let meridian = Frame::new(centre, -Vec3::y(), Vec3::x())?;
    let half_disc = Profile {
        plane: meridian,
        outer: ProfileLoop::Path {
            start: Point2::new(0.0, -radius),
            segments: vec![
                ProfileSegment::LineTo(Point2::new(0.0, radius)),
                ProfileSegment::ArcTo {
                    to: Point2::new(0.0, -radius),
                    via: Point2::new(radius, 0.0),
                },
            ],
        },
        holes: Vec::new(),
    };
    revolve(m, &half_disc, Axis::z_at(centre), TAU, control)
}

/// A loft through level rectangles centred on the z axis, their sides parallel to the x and y
/// axes and their corners on four guides: the cubic Bézier curve through `control`, (p, q, z) at
/// each of its points, and its mirror images in the xz plane, the yz plane and both. Where the
/// guide ends on the z axis the rectangles close to one point there, an apex, and there is no top.
///
/// arris's `loft` takes no guides, and its profiles hold no NURBS curve, so the solid's faces are
/// assembled as `spline/peanut-prism`'s are and proven by `ops::build`. Each wall, swept by one
/// side of the rectangle, lies on the extrusion of the guide seen square to it: the wall facing
/// +x on the planar Bézier (p, z) extruded along y, every point (p, y, z) of it. The guides are
/// the walls' edges, at v = ±q on that wall: on its extrusion they are exactly the cubic Bézier
/// (t, ±q) in (u, v), the guide's own parameter in u.
pub fn rectangle_on_guides(m: &mut Model, control: [[f64; 3]; 4]) -> Result<Body, Refusal> {
    let geom = |e: GeomError| Refusal::from(e.to_string());
    let tol = m.precision().tolerance();
    let tolerance = m.precision().default_tolerance;
    let bezier = || vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let unit = Interval::new(0.0, 1.0).map_err(|e| e.to_string())?;
    let [first, .., last] = control;
    let apex = last[0] == 0.0 && last[1] == 0.0;
    // the corners walked counter-clockwise about +z, (+, −), (+, +), (−, +), (−, −); wall k joins
    // corner k to corner k + 1 and faces `out[k]`
    let signs = [(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)];
    let out = [Vec3::x(), Vec3::y(), -Vec3::x(), -Vec3::y()];
    let corner = |[p, q, z]: [f64; 3], (sx, sy): (f64, f64)| Point3::new(sx * p, sy * q, z);

    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let vertex = |vertices: &mut Vec<VertexSpec>, point| {
        vertices.push(VertexSpec::New { point, tolerance });
        VertexKey::New(vertices.len() - 1)
    };
    let foot: Vec<VertexKey> =
        signs.iter().map(|&s| vertex(&mut vertices, corner(first, s))).collect();
    let head: Vec<VertexKey> = if apex {
        vec![vertex(&mut vertices, corner(last, (1.0, 1.0))); 4]
    } else {
        signs.iter().map(|&s| vertex(&mut vertices, corner(last, s))).collect()
    };
    let mut edge = |m: &mut Model, curve: Curve, range: Interval, start, end| {
        let geometry = EdgeGeometry::Curve { curve: m.add_curve(curve), range };
        edges.push(EdgeSpec::New { geometry, start, end, tolerance });
        edges.len() - 1
    };
    // the straight edge from corner `a` to corner `b` of the section through `at`
    let line = |at: [f64; 3], a: (f64, f64), b: (f64, f64)| {
        let (from, to) = (corner(at, a), corner(at, b));
        let range = Interval::new(0.0, (to - from).norm()).map_err(|e| e.to_string())?;
        let direction = UnitVec3::new_normalize(to - from);
        Ok::<_, Refusal>((Curve::Line { origin: from, direction }, range))
    };
    let mut guides = Vec::new();
    let (mut floor_edges, mut top_edges) = (Vec::new(), Vec::new());
    for k in 0..4 {
        let points = control.iter().map(|&c| corner(c, signs[k])).collect();
        let guide = NurbsCurve::new(3, bezier(), points, vec![1.0; 4]).map_err(geom)?;
        guides.push(edge(m, Curve::Nurbs(guide), unit, foot[k], head[k]));
    }
    for k in 0..4 {
        let next = (k + 1) % 4;
        // round the foot from corner k to k + 1, and round the top back from k + 1 to k
        let (curve, range) = line(first, signs[k], signs[next])?;
        floor_edges.push((edge(m, curve.clone(), range, foot[k], foot[next]), curve, range));
        if !apex {
            let (curve, range) = line(last, signs[next], signs[k])?;
            top_edges.push((edge(m, curve.clone(), range, head[next], head[k]), curve, range));
        }
    }

    let use_of = |m: &mut Model, edge: usize, along: bool, pcurve: Curve2| UseSpec {
        edge: EdgeKey::New(edge),
        orientation: if along { Orientation::Forward } else { Orientation::Reversed },
        pcurve: m.add_curve2(pcurve),
    };
    let straight = |(u, v): (f64, f64), along: Vec2| Curve2::Line {
        origin: Point2::new(u, v),
        direction: UnitVec2::new_normalize(along),
    };
    let face = |m: &mut Model, surface: Surface, walk: Vec<UseSpec>| FaceSpec::New {
        surface: m.add_surface(surface),
        orientation: Orientation::Forward,
        loops: vec![walk],
        tolerance,
    };
    // how far a wall's surface reaches either side of the z axis: past every control point
    let reach = control.iter().map(|c| c[0].abs().max(c[1].abs())).fold(0.0, f64::max) + 1.0;
    let across = Interval::new(-reach, reach).map_err(|e| e.to_string())?;
    let mut faces = Vec::new();
    for k in 0..4 {
        let next = (k + 1) % 4;
        let n = out[k];
        // the guide seen square to the wall, in the plane through the z axis facing n: (p, z) or
        // (q, z), and w, the half-width along the wall, q or p
        let (seen, w): (Vec<Point3>, Vec<f64>) = control.iter()
            .map(|&[p, q, z]| {
                let (h, w) = if n.x != 0.0 { (p, q) } else { (q, p) };
                (Point3::new(n.x * h, n.y * h, z), w)
            })
            .unzip();
        let profile = NurbsCurve::new(3, bezier(), seen, vec![1.0; 4]).map_err(geom)?;
        // extruded along n turned a quarter clockwise about z, so ∂u × ∂v faces out along n
        let along = Vec3::new(n.y, -n.x, 0.0);
        let wall = NurbsSurface::extrusion(&profile, along, across).map_err(geom)?;
        // corner k + 1 lies at v = −w on the wall, corner k at v = +w
        let side = |sign: f64| {
            let points = w.iter().enumerate()
                .map(|(i, &w)| Point2::new(i as f64 / 3.0, sign * w)).collect();
            NurbsCurve2::new(3, bezier(), points, vec![1.0; 4]).map(Curve2::Nurbs).map_err(geom)
        };
        // counter-clockwise in (u, v): up the guide of corner k + 1, across the top, down the
        // guide of corner k, across the foot
        let mut walk = vec![use_of(m, guides[next], true, side(-1.0)?)];
        if !apex {
            let top = top_edges[k].0;
            walk.push(use_of(m, top, true, straight((1.0, -w[3]), Vec2::y())));
        }
        walk.push(use_of(m, guides[k], false, side(1.0)?));
        walk.push(use_of(m, floor_edges[k].0, true, straight((0.0, w[0]), -Vec2::y())));
        faces.push(face(m, Surface::Nurbs(wall), walk));
    }
    // the flat ends, each walked counter-clockwise about its outward normal
    let mut meter = Meter::new(&Control::NONE);
    // each rim edge is walked against its own direction: the floor's from corner k + 1 back to k,
    // so in the order 3, 2, 1, 0, and the top's from k to k + 1, in the order 0, 1, 2, 3
    let mut cap = |m: &mut Model, z: f64, up: Vec3, rim: Vec<&(usize, Curve, Interval)>| {
        let frame = Frame::new(Point3::new(0.0, 0.0, z), up, Vec3::x()).map_err(OpError::from)?;
        let plane = Surface::Plane { frame };
        let mut walk = Vec::new();
        for (edge, curve, range) in rim {
            let pcurve = pcurve_on(curve, *range, &plane, tol, &mut meter).map_err(geom)?;
            walk.push(use_of(m, *edge, false, pcurve));
        }
        Ok::<_, Refusal>(face(m, plane, walk))
    };
    faces.push(cap(m, first[2], -Vec3::z(), floor_edges.iter().rev().collect())?);
    if !apex {
        faces.push(cap(m, last[2], Vec3::z(), top_edges.iter().collect())?);
    }
    let (builder, slots) =
        Builder::assemble(m, tolerance, Assembly { vertices, edges, shells: vec![faces] })
            .map_err(OpError::from)?;
    let keys = BuildKeys {
        namespace: 0,
        vertices: keyed(&slots.vertices),
        edges: keyed(&slots.edges),
        faces: keyed(&slots.faces[0]),
        shells: vec![0],
        body: 0,
    };
    let (body, _) = build(m, builder, &keys, &Control::NONE)?;
    Ok(body)
}

/// Every slot keyed by its place in the assembly.
fn keyed<S: Copy + Ord>(slots: &[S]) -> BTreeMap<S, u64> {
    slots.iter().enumerate().map(|(i, &s)| (s, i as u64)).collect()
}

/// The face of `body` that `point` lies on, as arris's point classifier names it.
pub fn face_through(m: &Model, body: Body, point: Point3) -> Result<Face, Refusal> {
    match classify_point(m, body, point).map_err(|e| Refusal::from(e.to_string()))? {
        Classification::On(shape) => Face::try_from(shape)
            .map_err(|_| Refusal::from(format!("{point:?} lies on an edge or a vertex, not a face"))),
        _ => Err(Refusal::from(format!("{point:?} lies on no face"))),
    }
}

/// The edge of `body` that `point` lies on, as arris's point classifier names it.
pub fn edge_through(m: &Model, body: Body, point: Point3) -> Result<Edge, Refusal> {
    match classify_point(m, body, point).map_err(|e| Refusal::from(e.to_string()))? {
        Classification::On(shape) => Edge::try_from(shape)
            .map_err(|_| Refusal::from(format!("{point:?} lies on a face or a vertex, not an edge"))),
        _ => Err(Refusal::from(format!("{point:?} lies on no edge"))),
    }
}
