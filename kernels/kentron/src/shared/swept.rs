//! What the swept implementations share.
//!
//! kentron has no sweep of a solid: `sweep::pipe` refuses one ("a pipe of a Solid"), and
//! `sweep::extrude` and `sweep::revolve` take vertices, edges, wires, faces and compounds of them.
//! So where the motion allows it, a solid's sweep is built from what elementary geometry says it
//! is (SPEC.md, "A swept solid"), each implementation saying how beside its code.
use kentron::boolean;
use kentron::math::{Point, Vector};
use kentron::topology::{Model, NodeKind, Shape};

/// A compound of `shapes`.
pub fn compound(m: &mut Model, shapes: &[Shape]) -> kentron::Result<Shape> {
    let c = m.new_empty(NodeKind::Compound);
    for &s in shapes {
        m.add(c, s)?;
    }
    Ok(c)
}

/// The solids of `shape`, each once.
pub fn solids(m: &Model, shape: Shape) -> Vec<Shape> {
    let mut found: Vec<Shape> = Vec::new();
    for s in m.explore(shape, NodeKind::Solid).collect::<Vec<_>>() {
        if !found.iter().any(|f| f.node == s.node) {
            found.push(s);
        }
    }
    found
}

/// The pieces of what a torus passes through as it slides along `slide`, square to its axis:
/// the torus (centre `centre`, axis along y, radii `major` and `minor`) where it starts and
/// where it ends, and each of four quarters of its face slid along `slide`, which here runs
/// along z (`slid_torus_in` with the frame at `centre` whose z is y and whose x is x).
pub fn slid_torus(m: &mut Model, centre: Point, major: f64, minor: f64, slide: Vector) -> kentron::Result<Vec<Shape>> {
    use kentron::math::{Direction, Frame};
    slid_torus_in(m, Frame::new(centre, Direction::Y, Direction::X), major, minor, slide)
}

/// The pieces of what a torus passes through as it slides along `slide`, square to its axis:
/// the torus (in `frame`, its axis `frame`'s z, radii `major` and `minor`) where it starts and
/// where it ends, and each of four quarters of its face slid along `slide`, which must run square
/// to `frame`'s x as well as to its z (along its y).
///
/// A face slid whole is only what it passes through if the slide never runs along it inside
/// it. The torus's normal at the point turned u about its axis (from `frame`'s x) and v round its
/// tube is cos v·(cos u·x + sin u·y) + sin v·z, square to a slide along y where sin u = 0, its
/// tube's two circles in the plane through its axis square to the slide, and where cos v = 0, the
/// circles of radius `major` on its tube's two sides (z = ±minor). Split there, into the faces on
/// u ∈ [0, π], [π, 2π] and v ∈ [−π/2, π/2], [π/2, 3π/2], each slides without folding over
/// itself. A point the torus covers at some instant that no face ever crosses stays inside it all
/// the way, so is in it at both ends.
pub fn slid_torus_in(m: &mut Model, frame: kentron::math::Frame, major: f64, minor: f64, slide: Vector) -> kentron::Result<Vec<Shape>> {
    use kentron::geometry::surface::{Surface, Torus as TorusSurface};
    use kentron::math::Transform;
    use kentron::modify::{self, TransformOptions};
    use kentron::primitives::Torus;
    use kentron::{make, sweep};
    use std::f64::consts::{FRAC_PI_2, PI};
    let start = Torus::new(major, minor).with_frame(frame).build(m)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    let surface = Surface::Torus(TorusSurface::new(frame, major, minor));
    let mut quarters = Vec::new();
    for (u1, u2) in [(0.0, PI), (PI, 2.0 * PI)] {
        for (v1, v2) in [(-FRAC_PI_2, FRAC_PI_2), (FRAC_PI_2, 3.0 * FRAC_PI_2)] {
            quarters.push(make::face_on_domain(m, surface.clone(), u1, u2, v1, v2)?);
        }
    }
    let quarters = compound(m, &quarters)?;
    let slid = sweep::extrude(m, quarters, slide)?.shape;
    let mut pieces = vec![start];
    pieces.extend(solids(m, slid));
    pieces.push(end);
    Ok(pieces)
}

/// `body` with each of `pieces` cut from it in turn, as holes are cut one after another: what
/// is left is `body` less their union, and pieces that only touch are never fused to each other.
pub fn cut_each(m: &mut Model, body: Shape, pieces: &[Shape]) -> kentron::Result<Shape> {
    let mut body = body;
    for &piece in pieces {
        body = boolean::difference(m, body, piece)?;
    }
    Ok(body)
}

/// What a flat face carries out ahead of it as it turns about `axis`: the part of the planar
/// face `face`, its outward normal `normal`, where the turn's velocity a × (r − o) points out
/// of the solid it bounds, (r − o)·(n × a) > 0, with the half-space bounded by the plane
/// through the axis square to n × a. None where the face turns within its own plane (n along a,
/// sweeping nothing) or leads nowhere. Inside that part the velocity never runs along the face,
/// so the face part revolved is exactly what it passes through.
pub fn leading_part(m: &mut Model, face: Shape, normal: Vector, axis: kentron::math::Axis) -> kentron::Result<Option<Shape>> {
    use crate::shared::boolean::half_space;
    use kentron::math::Direction;
    let ahead = normal.cross(Vector::from(axis.direction));
    let Some(ahead) = Direction::try_from_vector(ahead) else { return Ok(None) };
    let side = half_space(m, axis.origin, ahead, axis.origin + Vector::from(ahead))?;
    let part = boolean::intersection(m, face, side)?;
    Ok(m.explore(part, NodeKind::Face).next().map(|_| part))
}

/// What the filled ring (everything within `minor` of the disc of radius `major` about the z
/// axis through the origin, the solid of revolution of the half stadium) passes through as it
/// slides along the x axis by `length`. The filled ring is convex, so slid it is itself at each
/// end and, between, its section in the plane x = 0 (where the slide runs along it, its
/// silhouette along x) slid along x.
pub fn slid_filled_ring(m: &mut Model, major: f64, minor: f64, length: f64) -> kentron::Result<Shape> {
    use crate::shared::geometry::{arc, flat, segment, turned};
    use kentron::geometry::curve::{Circle, Curve};
    use kentron::math::{Direction, Frame, Transform};
    use kentron::modify::{self, TransformOptions};
    use kentron::sweep;
    use std::f64::consts::{FRAC_PI_2, PI};
    // The face in the plane through the z axis along `across`: within `minor` of the segment from
    // the axis to `major` along `across`, rounded at its far end (and, if `whole`, reaching as far
    // along −`across` and rounded there too).
    let section = |m: &mut Model, across: Direction, whole: bool| -> kentron::Result<Shape> {
        let at = |s: f64, z: f64| Point::new(0.0, 0.0, z) + Vector::from(across) * s;
        let normal = Direction::from_vector(across.cross(Direction::Z));
        let round = |m: &mut Model, s: f64, from: f64| arc(m, Curve::Circle(Circle::new(Frame::new(at(s, 0.0), normal, across), minor)), from, from + PI);
        let near = if whole { -major } else { 0.0 };
        let mut edges = vec![segment(m, at(near, -minor), at(major, -minor))?, round(m, major, -FRAC_PI_2)?, segment(m, at(major, minor), at(near, minor))?];
        edges.push(if whole { round(m, -major, FRAC_PI_2)? } else { segment(m, at(0.0, minor), at(0.0, -minor))? });
        flat(m, &edges)
    };
    let slide = Vector::new(length, 0.0, 0.0);
    let half = section(m, Direction::X, false)?;
    let start = turned(m, half)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    let middle = section(m, Direction::Y, true)?;
    let middle = sweep::extrude(m, middle, slide)?.shape;
    let body = boolean::union(m, start, middle)?;
    boolean::union(m, body, end)
}

/// The rod, the cylinder of `radius` about the z axis from z = 0 to `height`, less what a bar
/// passes through as a screw about the z axis, rising `lead` a turn, carries it from wholly below
/// the rod to wholly above it: the bar the prism on `outline` (in the xy plane) from
/// z = −`thickness`/2 to `thickness`/2.
///
/// What the bar fills there is screwed into itself, so the groove is its section at z = 0 screwed
/// up the rod. At z = 0 the bar lies while its heights hold 0, over turns within
/// ±(`thickness`/2)/(`lead`/2π), and every slice of it is its outline: that section is what the
/// outline sweeps turning through that range about the z axis, kentron's planar cut by a motion
/// (`sweep::cut_by_motion`) of the prism from a disc, of which the face at z = 0 is the rod's
/// section less the groove's.
pub fn rod_screw_cut_by_bar(m: &mut Model, radius: f64, height: f64, outline: Vec<kentron::math::Point2>, thickness: f64, lead: f64) -> Result<Shape, crate::Refusal> {
    use crate::bench::face_through;
    use kentron::math::{Axis, Frame};
    use kentron::sweep::{self, Blank, Motion, MotionStep, Tool};
    let disc = Blank::Disc { frame: Frame::WORLD, radius, height: 1.0 };
    let bar = Tool::Prism { frame: Frame::WORLD, outline, from: -1.0, to: 2.0 };
    let reach = thickness / 2.0 / (lead / std::f64::consts::TAU);
    let turn = MotionStep { axis: Axis::Z, turn: 1.0, advance: 0.0 };
    let cut = sweep::cut_by_motion(m, &disc, &bar, &Motion { steps: vec![turn], from: -reach, to: reach })?;
    let section = face_through(m, cut, Point::new(0.0, 0.0, 0.0))?;
    Ok(sweep::screw_extrude(m, section, Axis::Z, lead, height)?.shape)
}

/// What of the rod about the z axis (radius 5) a square bar passes through as a screw about the
/// z axis, rising `lead` a turn, carries it from a turn of `from` (radians), wholly below the
/// rod, to `to`: the bar the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 as placed, at turn 0.
///
/// A point of the rod the bar covers at some instant (and not where it starts, below the rod)
/// came into it through a face the screw carries forward there. The screw's velocity at p,
/// (−y, x, `lead`/2π) a radian, runs out of the bar's top face z = 1 all over, and of its tip
/// x = 4 where y > 0; its bottom face and the tip's other half it leaves behind, and its other
/// faces never reach the rod. Those two faces, carried forward all over, never fold over
/// themselves, and the bar, 2 tall, rises more than that a turn, so they never meet themselves:
/// each sweeps its screwed prism (`sweep::screw_extrude`) from where the bar starts. Only their
/// points inside the rod matter, so each is taken only as far as y = ±4 and x = 6, its new edges
/// all more than 5 from the axis. Those two prisms are the pieces returned, the tip's first, to be
/// cut from the rod in that order. (Screwed whole, the faces' prisms are cut from the rod into a
/// shell that does not close, as the top's is cut first for `swept/square-bar-screwed-partway-up-rod`.)
pub fn screwed_square_bar(m: &mut Model, lead: f64, from: f64, to: f64) -> kentron::Result<Vec<Shape>> {
    use crate::bench::p;
    use crate::shared::offset::polygon_face;
    use kentron::math::{Axis, Transform};
    use kentron::modify::{self, TransformOptions};
    use kentron::sweep;
    let k = lead / std::f64::consts::TAU;
    let start = Transform::translation(Vector::new(0.0, 0.0, k * from)) * Transform::rotation(Axis::Z, from);
    let top = polygon_face(m, &[p(4.0, -4.0, 1.0), p(6.0, -4.0, 1.0), p(6.0, 4.0, 1.0), p(4.0, 4.0, 1.0)])?;
    let tip = polygon_face(m, &[p(4.0, 0.0, -1.0), p(4.0, 4.0, -1.0), p(4.0, 4.0, 1.0), p(4.0, 0.0, 1.0)])?;
    let mut pieces = Vec::new();
    for face in [tip, top] {
        let placed = modify::transform(m, face, &start, TransformOptions::default())?;
        pieces.push(sweep::screw_extrude(m, placed, Axis::Z, lead, k * (to - from))?.shape);
    }
    Ok(pieces)
}

/// The bull-nose mill drawn about the z axis from its flat end at the origin up 3 (a cylinder of
/// radius 2, its end corner rounded to 0.5: a disc of radius 1.5, a quarter torus, the side),
/// its meridian revolved, then moved by `place`.
pub fn bull_nose(m: &mut Model, place: &kentron::math::Transform) -> kentron::Result<Shape> {
    use kentron::geometry::curve::{Circle, Curve};
    use kentron::math::{Axis, Direction, Frame};
    let xz = |x: f64, z: f64| Point::new(x, 0.0, z);
    let round = Circle::new(Frame::new(xz(1.5, 0.5), Direction::new(0.0, -1.0, 0.0), Direction::X), 0.5);
    let edges = vec![
        kentron::make::edge_between(m, xz(0.0, 0.0), xz(1.5, 0.0))?,
        kentron::make::edge_from_curve_range(m, Curve::Circle(round), -std::f64::consts::FRAC_PI_2, 0.0)?,
        kentron::make::edge_between(m, xz(2.0, 0.5), xz(2.0, 3.0))?,
        kentron::make::edge_between(m, xz(2.0, 3.0), xz(0.0, 3.0))?,
        kentron::make::edge_between(m, xz(0.0, 3.0), xz(0.0, 0.0))?,
    ];
    let wire = kentron::make::wire(m, &edges)?;
    let meridian = kentron::make::planar_face(m, wire)?;
    let upright = kentron::sweep::revolve(m, meridian, Axis::Z, std::f64::consts::TAU)?.shape;
    kentron::modify::transform(m, upright, place, Default::default())
}

/// The sector of the ring about the z axis between radii 8 and 12, z from −1 to 1, polar angle
/// from 0° to 60°.
pub fn ring_sector(m: &mut Model) -> kentron::Result<Shape> {
    let z = kentron::math::Axis::new(Point::new(0.0, 0.0, 0.0), kentron::math::Direction::Z);
    let section = crate::shared::primitives::rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    Ok(kentron::sweep::revolve(m, section, z, std::f64::consts::FRAC_PI_3)?.shape)
}

/// The frame riding the cubic Bézier curve through (0, 0, 0), (10, 0, 0), (20, 20, 0) and
/// (30, 20, 0) at `t`: its origin the curve's point, its x axis the tangent, its z axis up.
pub fn bezier_frame(t: f64) -> kentron::math::Transform {
    let p = [(0.0, 0.0), (10.0, 0.0), (20.0, 20.0), (30.0, 20.0)];
    let s = 1.0 - t;
    let at = |k: usize| s * s * s * [p[0].0, p[0].1][k] + 3.0 * s * s * t * [p[1].0, p[1].1][k] + 3.0 * s * t * t * [p[2].0, p[2].1][k] + t * t * t * [p[3].0, p[3].1][k];
    let along = |k: usize| 3.0 * (s * s * ([p[1].0, p[1].1][k] - [p[0].0, p[0].1][k]) + 2.0 * s * t * ([p[2].0, p[2].1][k] - [p[1].0, p[1].1][k]) + t * t * ([p[3].0, p[3].1][k] - [p[2].0, p[2].1][k]));
    let (dx, dy) = (along(0), along(1));
    let l = dx.hypot(dy);
    let (c, sn) = (dx / l, dy / l);
    kentron::math::Transform::from_rows([[c, -sn, 0.0, at(0)], [sn, c, 0.0, at(1)], [0.0, 0.0, 1.0, 0.0]]).expect("a rigid frame")
}
