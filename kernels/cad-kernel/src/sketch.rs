//! The solids and picks the cases share: a closed polygon as a profile, a rectangle and a rounded
//! one, a block between two corners, the octahedron, the L-block, a ball cut by a level, a rounded plate, a
//! capsule, a drilled block, a bent flange and a half pipe, a face picked by a point inside it,
//! drafts about a plane a profile or a face does not start on, and a section swept along a path of
//! straight runs mitred at its corners. Everything else a case builds with cad-kernel's own `build`
//! functions.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, drafted, hull, kept, offset, prism, revolve, sphere, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, Frame, Rigid, V};
use cad_kernel::query::{Located, Place};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, distance, dot, norm, scale, sub, unit};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The closed polygon through `points` as a profile of lines, its normal by Newell's rule (so a
/// polygon walked counter-clockwise seen from above faces up).
pub fn polygon(points: &[V]) -> Profile {
    let n = points.len();
    let normal = (0..n).fold([0.; 3], |s, k| add(s, cross(points[k], points[(k + 1) % n])));
    let edges = (0..n).map(|k| ProfileEdge::Line { a: points[k], b: points[(k + 1) % n] }).collect();
    Profile { origin: points[0], normal: unit(normal), loops: vec![edges], names: Vec::new() }
}

/// The rectangle from `lo` to `hi` (corners in x and y) in the plane z = `z`, facing +z.
pub fn rectangle(lo: [f64; 2], hi: [f64; 2], z: f64) -> Profile {
    polygon(&[[lo[0], lo[1], z], [hi[0], lo[1], z], [hi[0], hi[1], z], [lo[0], hi[1], z]])
}

/// The block with its least corner at `lo` and its greatest at `hi`: its floor, a rectangle,
/// swept up.
pub fn block(lo: V, hi: V) -> Result<Brep, String> {
    prism(&rectangle([lo[0], lo[1]], [hi[0], hi[1]], lo[2]), 0., hi[2] - lo[2])
}

/// The octahedron |x| + |y| + |z| ≤ `reach`: the convex hull of its six corners on the axes.
pub fn octahedron(reach: f64) -> Result<Brep, String> {
    let (r, o) = (reach, 0.);
    hull(&[[r, o, o], [-r, o, o], [o, r, o], [o, -r, o], [o, o, r], [o, o, -r]])
}

/// The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high.
pub fn l_block() -> Result<Brep, String> {
    let l = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 5., 0.], [5., 5., 0.], [5., 15., 0.], [0., 15., 0.]]);
    prism(&l, 0., 8.)
}

/// The faces of `b` through which the points `on` pass, one each: a face picked by a point inside
/// it, as a description names a face by where it is.
pub fn faces_through(b: &Brep, on: &[V]) -> Result<Vec<usize>, String> {
    if on.is_empty() { return Ok(Vec::new()); }
    let tol = tolerance(b.size());
    let located = Located::new(b, tol);
    on.iter().map(|&p| {
        (0..b.faces.len()).find(|&fi| {
            // (a face whose box the point is clear of is not read onto)
            located.face_gap(fi, p) <= tol
                && distance(b.faces[fi].surface.point(located.uv(fi, p)), p) <= 1e-9 * (1. + b.size())
                && located.face_place(fi, p) == Place::In
        }).ok_or_else(|| format!("no face through {p:?}"))
    }).collect()
}

/// The ball of radius `r` about `c` kept below the plane `z = h` (its axis along z, so the plane
/// cuts it along one of its parallels).
pub fn ball_below(c: V, r: f64, h: f64) -> Result<Brep, String> {
    kept(&sphere(c, [0., 0., 1.], r)?, [c[0], c[1], h], [0., 0., -1.], 0.)
}

/// The rectangle from `lo` to `hi` in the plane z = `z`, its corners rounded to `r`, as a profile
/// facing +z.
pub fn rounded_rectangle(lo: [f64; 2], hi: [f64; 2], r: f64, z: f64) -> Profile {
    let corners = [([hi[0] - r, lo[1] + r], -FRAC_PI_2), ([hi[0] - r, hi[1] - r], 0.),
                   ([lo[0] + r, hi[1] - r], FRAC_PI_2), ([lo[0] + r, lo[1] + r], PI)];
    let mut edges = Vec::new();
    for k in 0..4 {
        let (c, a) = corners[k];
        let frame = Frame::new([c[0], c[1], z], [0., 0., 1.], [1., 0., 0.]);
        edges.push(ProfileEdge::Arc { frame, r, span: Some([a, a + FRAC_PI_2]) });
        let end = [c[0] + r * (a + FRAC_PI_2).cos(), c[1] + r * (a + FRAC_PI_2).sin(), z];
        let (n, b) = corners[(k + 1) % 4];
        let start = [n[0] + r * b.cos(), n[1] + r * b.sin(), z];
        edges.push(ProfileEdge::Line { a: end, b: start });
    }
    Profile { origin: [0., 0., z], normal: [0., 0., 1.], loops: vec![edges], names: Vec::new() }
}

/// The plate whose outline is the rectangle from `lo` to `hi` in the plane z = 0, its corners
/// rounded to `r`, `depth` high.
pub fn rounded_plate(lo: [f64; 2], hi: [f64; 2], r: f64, depth: f64) -> Result<Brep, String> {
    prism(&rounded_rectangle(lo, hi, r, 0.), 0., depth)
}

/// The capsule's end: the cylinder of radius `r` on the z axis from z = 0 to `h`, capped by the
/// half ball of radius `r` about (0, 0, h), its section turned about z.
pub fn capsule(r: f64, h: f64) -> Result<Brep, String> {
    let arc = ProfileEdge::Arc { frame: Frame::new([0., 0., h], [0., -1., 0.], [1., 0., 0.]), r, span: Some([0., FRAC_PI_2]) };
    let edges = vec![ProfileEdge::Line { a: [0., 0., 0.], b: [r, 0., 0.] },
                     ProfileEdge::Line { a: [r, 0., 0.], b: [r, 0., h] }, arc,
                     ProfileEdge::Line { a: [0., 0., h + r], b: [0., 0., 0.] }];
    revolve(&Profile { origin: [0., 0., 0.], normal: [0., -1., 0.], loops: vec![edges], names: Vec::new() },
            [0., 0., 0.], [0., 0., 1.], TAU)
}

/// The bent flange's solid: the profile from (−10, 0, 0) to the origin, round the quarter circle
/// of radius 2 about (0, 0, 2) to (2, 0, 2) and up to (2, 0, 12), closed above and to the left,
/// swept 8 along y; and the points its floor, bend and wall are picked by.
pub fn bent_flange() -> Result<(Brep, [V; 3]), String> {
    let bend = Frame::new([0., 0., 2.], [0., 1., 0.], [1., 0., 0.]);
    let section = Profile { origin: [0., 0., 0.], normal: [0., 1., 0.], names: Vec::new(), loops: vec![vec![
        ProfileEdge::Line { a: [-10., 0., 0.], b: [0., 0., 0.] },
        ProfileEdge::Arc { frame: bend, r: 2., span: Some([0., FRAC_PI_2]) },
        ProfileEdge::Line { a: [2., 0., 2.], b: [2., 0., 12.] },
        ProfileEdge::Line { a: [2., 0., 12.], b: [-10., 0., 12.] },
        ProfileEdge::Line { a: [-10., 0., 12.], b: [-10., 0., 0.] },
    ]] };
    let h = std::f64::consts::SQRT_2;
    Ok((prism(&section, 0., 8.)?, [[-5., 4., 0.], [h, 4., 2. - h], [2., 4., 7.]]))
}

/// The half of the cylinder of radius 5 on the z axis from z = 0 to 10 where y ≥ 0.
pub fn half_pipe() -> Result<Brep, String> {
    kept(&prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?, [0., 0., 0.], [0., 1., 0.], 0.)
}

/// The block from the origin to (20, 20, 10) with a hole of radius 3 bored right through it along
/// z, its axis the line x = `x`, y = 10.
pub fn drilled(x: f64) -> Result<Brep, String> {
    let drill = prism(&disc([x, 10., -1.], [0., 0., 1.], 3.), 0., 12.)?;
    Ok(combined(&block([0., 0., 0.], [20., 20., 10.])?, &drill, Op::Cut, 0.)?)
}

/// Every face of `b` given a name of its own (`face 3`): a chamfer reads faces of one name as
/// pieces of one face, and a block's sides and caps, or a polyhedron's cuts (which keep the
/// cutting block's `far` and `near`), are each their own.
pub fn named(mut b: Brep) -> Brep {
    for (i, f) in b.faces.iter_mut().enumerate() { f.name = format!("face {i}"); }
    b
}

/// The profile `p`, lying in the neutral plane, swept along its normal from `below` under that
/// plane to `above` over it, its sides tapered by `draft` and leaning the same way throughout: into
/// its material as they go along the normal, so out of it below the plane. (cad-kernel's drafted
/// prism leans away from its profile's plane both ways.) Its section at `−below` is `p` moved
/// `below·tan draft` out of its material, tapered from there.
pub fn drafted_through(p: &Profile, below: f64, above: f64, draft: f64) -> Result<Brep, String> {
    let down = Rigid { t: scale(unit(p.normal), -below), ..Rigid::identity() };
    drafted(&offset(p, -below * draft.tan())?.moved(&down), 0., below + above, draft)
}

/// `solid` with its upright plane face through `on`, facing `outward` (horizontal), drafted by
/// `draft` with the pull direction +z about the neutral plane through `on`: the face turned about
/// its line in that plane until it leans `draft` from z, inward as it rises. The solid is kept
/// inside the turned plane, which trims the faces beside it.
pub fn drafted_face(solid: &Brep, on: V, outward: V, draft: f64) -> Result<Brep, String> {
    let (s, c) = draft.sin_cos();
    kept(solid, on, [-c * outward[0], -c * outward[1], -s], 0.)
}

/// `section`, standing square to the polyline `path` at its start, swept straight along each of its
/// runs (and back from its last point to its first where `closed`), mitred at every corner: each
/// run is cut by, and the runs meet in, the plane through the corner that bisects it, square to the
/// sum of the two runs' directions. The section is carried from run to run by the least turn at
/// each corner, about the line square to both runs. `reach` is how far the section reaches from
/// the path. An open path's ends are flat, square to it.
///
/// cad-kernel has no sweep along a path with corners, so each run is the section swept along it
/// and on past the planes at its ends (a prism), kept between them, and the runs are joined.
pub fn mitred(section: &Profile, path: &[V], closed: bool, reach: f64) -> Result<Brep, String> {
    let n = path.len();
    let runs = if closed { n } else { n - 1 };
    let direction = |k: usize| unit(sub(path[(k + 1) % n], path[k]));
    // how far past a corner a run reaches before its mitre plane, from `a` turning to `b`: the
    // section's reach times the tangent of half the turn, and the reach again to spare
    let past = |a: V, b: V| reach * (1. + norm(cross(a, b)) / (1. + dot(a, b)));
    let mut section = section.clone();
    let mut pipe: Option<Brep> = None;
    for k in 0..runs {
        let d = direction(k);
        if k > 0 {
            // carried along the last run, then turned at the corner from its direction to this one
            let last = direction(k - 1);
            let carry = Rigid { t: sub(path[k], path[k - 1]), ..Rigid::identity() };
            let turn = Rigid::turn(path[k], cross(last, d), dot(last, d).clamp(-1., 1.).acos());
            section = section.moved(&carry.then(&turn));
        }
        let before = (k > 0 || closed).then(|| direction((k + runs - 1) % runs));
        let after = (k + 1 < runs || closed).then(|| direction((k + 1) % runs));
        let (back, on) = (before.map_or(0., |b| past(b, d)), after.map_or(0., |a| past(d, a)));
        let length = distance(path[k], path[(k + 1) % n]);
        // the prism's heights run along the section's normal, which may face back along the run
        let sense = dot(unit(section.normal), d).signum();
        let mut run = prism(&section, -sense * back, sense * (length + on))?;
        if let Some(b) = before {
            run = kept(&run, path[k], unit(add(b, d)), 0.)?;
        }
        if let Some(a) = after {
            run = kept(&run, path[(k + 1) % n], scale(unit(add(d, a)), -1.), 0.)?;
        }
        pipe = Some(match pipe {
            None => run,
            Some(p) => combined(&p, &run, Op::Union, 0.)?,
        });
    }
    pipe.ok_or_else(|| "a path of no runs".into())
}
