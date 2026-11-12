//! Helpers for the swept cases built by elementary geometry where next.BREP's own sweep of a solid
//! (`sweep_solid_within`) does not reach: a solid of revolution's meridian drawn in a plane, the
//! curved block the ring-sector cases cut, a solid slid along a line by its faces, and an upright
//! solid of revolution slid along a line or turned partway about an upright axis. And, for the
//! cases it does reach, the tools and motions `sweep_solid` takes, in the cases' own terms.
#![allow(unused_imports)]
use super::*;
use super::sweep::{moved, turn_z};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, extrude_profile_brep, make_arc, make_line, revolve_profile_brep,
    BooleanOperation, BrepSolid, NurbsCurve, OutlinePiece, ScrewStep, SolidMotion, SolidTool, Vec3,
};
use std::f64::consts::{PI, TAU};

/// One piece of a planar chain in (r, z) coordinates: r along a chosen direction, z along another.
#[derive(Clone, Copy, Debug)]
pub enum Piece {
    /// The segment from `from` to `to`.
    Line { from: [f64; 2], to: [f64; 2] },
    /// The arc about `centre` of `radius` from angle `start` to `end` (radians, from +r toward
    /// +z; `end < start` runs clockwise).
    Arc { centre: [f64; 2], radius: f64, start: f64, end: f64 },
}

impl Piece {
    /// The piece mirrored in r = 0 and run backwards.
    fn mirrored_back(self) -> Piece {
        match self {
            Piece::Line { from, to } => Piece::Line { from: [-to[0], to[1]], to: [-from[0], from[1]] },
            Piece::Arc { centre, radius, start, end } => Piece::Arc {
                centre: [-centre[0], centre[1]],
                radius,
                start: PI - end,
                end: PI - start,
            },
        }
    }
}

/// The curves of `pieces` in the plane through `origin` with r along the unit `r` and z along the
/// unit `z`.
pub fn planar(pieces: &[Piece], origin: Vec3, r: Vec3, z: Vec3) -> Result<Vec<NurbsCurve>, String> {
    let at = |q: [f64; 2]| origin.add(r.scale(q[0])).add(z.scale(q[1]));
    pieces
        .iter()
        .map(|piece| match *piece {
            Piece::Line { from, to } => make_line(at(from), at(to)),
            Piece::Arc { centre, radius, start, end } => {
                if end >= start {
                    make_arc(at(centre), r, z, radius, start, end)
                } else {
                    // clockwise: the same arc about the mirrored frame
                    make_arc(at(centre), r, z.scale(-1.0), radius, -start, -end)
                }
            }
        })
        .collect()
}

/// A solid of revolution's meridian half: a chain in (r, z) from a point on its axis (r = 0) out
/// and back to another point on its axis, r ≥ 0 all along.
pub struct Meridian(pub Vec<Piece>);

impl Meridian {
    fn ends(&self) -> ([f64; 2], [f64; 2]) {
        let start = match self.0[0] {
            Piece::Line { from, .. } => from,
            Piece::Arc { centre, radius, start, .. } => {
                [centre[0] + radius * start.cos(), centre[1] + radius * start.sin()]
            }
        };
        let end = match *self.0.last().unwrap() {
            Piece::Line { to, .. } => to,
            Piece::Arc { centre, radius, end, .. } => {
                [centre[0] + radius * end.cos(), centre[1] + radius * end.sin()]
            }
        };
        (start, end)
    }

    /// The closed half: the chain and the segment of the axis that closes it.
    pub fn half(&self) -> Vec<Piece> {
        let (start, end) = self.ends();
        let mut pieces = self.0.clone();
        pieces.push(Piece::Line { from: end, to: start });
        pieces
    }

    /// The whole section through the axis: the chain and its mirror image in the axis.
    pub fn whole(&self) -> Vec<Piece> {
        let mut pieces = self.0.clone();
        pieces.extend(self.0.iter().rev().map(|piece| piece.mirrored_back()));
        pieces
    }

    /// The solid of revolution about the upright line through `foot` (z measured from `foot.z`).
    pub fn upright_solid(&self, foot: Vec3) -> Result<BrepSolid, String> {
        let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
        revolve_profile_brep(&planar(&self.half(), foot, x, z)?, foot, z, TAU)
    }
}

/// The curved block of the ring-sector cases: everything between `r0` and `r1` from the z axis,
/// with z between `z0` and `z1`, at polar angles from `from` to `to` (radians, from +x toward +y).
pub fn ring_sector(r0: f64, r1: f64, z0: f64, z1: f64, from: f64, to: f64)
    -> Result<BrepSolid, String> {
    let section = polygon(&[p(r0, 0.0, z0), p(r1, 0.0, z0), p(r1, 0.0, z1), p(r0, 0.0, z1)])?;
    let sector = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), to - from)?;
    moved(&sector, turn_z(from, p(0.0, 0.0, 0.0))?)
}

/// What a solid bounded by flat faces passes through as it slides without turning along `v`:
/// the solid where it starts (`solid`), where it stops, and each of its faces (the closed
/// polygons `faces`) extruded along `v`, the faces parallel to `v` left out.
///
/// This holds for any solid P: a point q lies in P + sv for some s in [0, 1] exactly when the
/// segment from q − v to q along v meets P. If q is in neither P nor P + v, the parameters s with
/// q − sv in P form a closed set inside (0, 1), whose least element s₀ puts q − s₀v on P's
/// boundary, on some face F that the line crosses (not one parallel to v, which the line meets
/// only within the boundary of a crossing face), so q lies in F slid along v.
pub fn polyhedron_slid(solid: &BrepSolid, faces: &[Vec<Vec3>], v: Vec3)
    -> Result<Vec<BrepSolid>, Refusal> {
    let mut parts = vec![solid.clone(), moved(solid, slide(v)?)?];
    for face in faces {
        let normal = face[1].sub(face[0]).cross(face[2].sub(face[0]));
        if normal.dot(v).abs() <= 1e-9 * normal.length() * v.length() {
            continue;
        }
        parts.push(extrude_profile_brep(&polygon(face)?, v, v.length())?);
    }
    Ok(parts)
}

/// The six faces of the box with corner `o` and edges `a`, `b`, `c`, as closed polygons.
pub fn box_faces(o: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Vec<Vec<Vec3>> {
    let corner = |i: f64, j: f64, k: f64| o.add(a.scale(i)).add(b.scale(j)).add(c.scale(k));
    let mut faces = Vec::new();
    for s in [0.0, 1.0] {
        faces.push(vec![corner(s, 0.0, 0.0), corner(s, 1.0, 0.0), corner(s, 1.0, 1.0),
            corner(s, 0.0, 1.0)]);
        faces.push(vec![corner(0.0, s, 0.0), corner(1.0, s, 0.0), corner(1.0, s, 1.0),
            corner(0.0, s, 1.0)]);
        faces.push(vec![corner(0.0, 0.0, s), corner(1.0, 0.0, s), corner(1.0, 1.0, s),
            corner(0.0, 1.0, s)]);
    }
    faces
}

/// The translation by `v`.
pub fn slide(v: Vec3) -> Result<brep_kernel::AffineTransform, String> {
    turn_z(0.0, v)
}

/// What an upright solid of revolution (`meridian`, its axis the upright line through `foot`)
/// passes through as it slides without turning by `v`, a level vector: the tool where it starts
/// and where it stops, and its whole section through its axis square to the slide extruded along
/// the slide from where it starts.
///
/// Each line along the slide at height z meets the tool, a disc of radius r(z) in that level, in a
/// chord about the axis's foot, or not at all, as its distance from the axis is within r(z) or not:
/// so the points the tool passes on that line run from the start's chord to the stop's, and between
/// them are exactly those of the section slid.
pub fn upright_slid(meridian: &Meridian, foot: Vec3, v: Vec3) -> Result<Vec<BrepSolid>, Refusal> {
    let start = meridian.upright_solid(foot)?;
    let stop = moved(&start, slide(v)?)?;
    let along = v.scale(1.0 / v.length());
    let across = p(0.0, 0.0, 1.0).cross(along);
    let section = planar(&meridian.whole(), foot, across, p(0.0, 0.0, 1.0))?;
    Ok(vec![start, extrude_profile_brep(&section, along, v.length())?, stop])
}

/// What an upright solid of revolution passes through as it turns rigidly about the z axis from
/// `from` to `to` (radians, right-handed), its own axis the upright line through (`d`, 0) when the
/// turn is 0: the tool where it starts and where it stops, and its section by the half plane
/// through the z axis and its own (r from `d` − r(z) to `d` + r(z)) turned from the start to the
/// stop.
///
/// In each level the tool is a disc of radius r(z) about its axis, d from the z axis, and a circle
/// about the z axis meets a disc in one arc symmetric about the disc's centre's angle, holding that
/// angle when it meets it at all. So the angles at which the tool covers a point of that circle, as
/// it turns, run from the start's arc to the stop's, and between them are exactly the section's.
/// `meridian` may be the tool's own or, where the tool has a hole, that of the tool with its hole
/// filled, when the case shows that the turn sweeps the hole too.
pub fn upright_turned(meridian: &Meridian, d: f64, from: f64, to: f64)
    -> Result<Vec<BrepSolid>, Refusal> {
    let tool = meridian.upright_solid(p(d, 0.0, 0.0))?;
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let section = planar(&meridian.whole(), p(d, 0.0, 0.0), x, z)?;
    let turned = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), z, to - from)?;
    let at = |angle: f64| -> Result<BrepSolid, String> { moved(&tool, turn_z(angle, p(0.0, 0.0, 0.0))?) };
    Ok(vec![at(from)?, moved(&turned, turn_z(from, p(0.0, 0.0, 0.0))?)?, at(to)?])
}

/// The box with corner `o` and edges `a`, `b`, `c`, as a tool for the kernel's sweep of a solid: a
/// convex polyhedron, its eight corners and six faces.
pub fn box_tool(o: Vec3, a: Vec3, b: Vec3, c: Vec3) -> SolidTool {
    let corner = |i: usize| {
        let on = |bit: usize, edge: Vec3| if i >> bit & 1 == 1 { edge } else { p(0.0, 0.0, 0.0) };
        o.add(on(0, a)).add(on(1, b)).add(on(2, c))
    };
    SolidTool::Polyhedron {
        vertices: (0..8).map(corner).collect(),
        faces: vec![vec![0, 2, 6, 4], vec![1, 3, 7, 5], vec![0, 1, 5, 4], vec![2, 3, 7, 6],
            vec![0, 1, 3, 2], vec![4, 5, 7, 6]],
    }
}

/// The solid of revolution about the line through `origin` along the unit `axis` whose meridian,
/// in the half plane from the axis toward the unit `radial`, is the closed chain `meridian` in
/// (distance from the axis, place along it): a tool for the kernel's sweep of a solid.
pub fn revolved_tool(origin: Vec3, axis: Vec3, radial: Vec3, meridian: &[Piece]) -> SolidTool {
    let piece = |piece: &Piece| match *piece {
        Piece::Line { from, to } => OutlinePiece::Line { from, to },
        Piece::Arc { centre, radius, start, end } => OutlinePiece::Arc { centre, radius, start, end },
    };
    SolidTool::Revolved {
        axis_origin: origin,
        axis_direction: axis,
        radial,
        meridian: meridian.iter().map(piece).collect(),
    }
}

/// The slide without turning by `v`, as a motion for the kernel's sweep of a solid.
pub fn slide_by(v: Vec3) -> SolidMotion {
    let step = ScrewStep {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: v.scale(1.0 / v.length()),
        turn: 0.0,
        advance: v.length(),
    };
    SolidMotion { steps: vec![step], from: 0.0, to: 1.0 }
}

/// The rigid turn about the z axis from the angle `from` to the angle `to` (radians,
/// right-handed), rising `rise` for each radian turned, as a motion for the kernel's sweep of a
/// solid.
pub fn turn_about_z(from: f64, to: f64, rise: f64) -> SolidMotion {
    let step = ScrewStep {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        turn: 1.0,
        advance: rise,
    };
    SolidMotion { steps: vec![step], from, to }
}

/// `block` less each of `swept`.
pub fn less_all(block: BrepSolid, swept: Vec<BrepSolid>) -> Result<BrepSolid, Refusal> {
    let operands: Vec<_> = std::iter::once(block).chain(swept).collect();
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
