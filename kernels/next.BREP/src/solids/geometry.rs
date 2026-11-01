//! Helpers shared by the geometry cases.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    build_pcurve_on_surface, loft_profile_brep, make_line, AffineTransform, ArenaCoedge, ArenaEdge,
    ArenaFace, ArenaLoop, ArenaShell, ArenaVertex, BrepSolid, EdgeId, NurbsCurve, NurbsSurface,
    TopologyArena, Vec3, Vec4, VertexId,
};

/// The axis-aligned stretch by `sx`, `sy` and `sz` about the origin, an affine map the kernel
/// applies exactly to every pole of a B-rep (`transform_brep`).
pub fn stretch(sx: f64, sy: f64, sz: f64) -> Result<AffineTransform, String> {
    AffineTransform::new([
        sx, 0.0, 0.0, 0.0,
        0.0, sy, 0.0, 0.0,
        0.0, 0.0, sz, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

/// The closed periodic uniform quadratic B-spline on the square control polygon `corners`, taken in
/// order: four parabolic arcs, each from the middle of one side over a corner to the middle of the
/// next, written clamped (double knots at the sides' middles) as two curves of two arcs each, from
/// the middle of the last side to the middle of the second and back.
pub fn rounded_square(corners: &[Vec3; 4]) -> Result<Vec<NurbsCurve>, String> {
    let middle = |k: usize| corners[k % 4].add(corners[(k + 1) % 4]).scale(0.5);
    let half = |k: usize| {
        let poles = [middle(k + 3), corners[k], middle(k), corners[k + 1], middle(k + 1)];
        NurbsCurve::new(
            2,
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0],
            poles.iter().map(|&q| Vec4::from_point(q, 1.0)).collect(),
        )
    };
    Ok(vec![half(0)?, half(2)?])
}

/// The block under the saddle z = 10 + xy/8 over the square −10 ≤ x, y ≤ 10, down to z = −5: the
/// loft of its two ends, the sections in the planes x = −10 and x = 10. Each of its four faces is
/// ruled between corresponding sides of the ends; the top, between the saddle's edges at x = ±10,
/// is the bilinear patch on its four corners (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5) and
/// (10, 10, 22.5).
pub fn under_saddle() -> Result<BrepSolid, Refusal> {
    let saddle = |x: f64, y: f64| 10.0 + x * y / 8.0;
    let end = |x: f64| {
        polygon(&[p(x, -10.0, -5.0), p(x, 10.0, -5.0), p(x, 10.0, saddle(x, 10.0)),
                  p(x, -10.0, saddle(x, -10.0))])
    };
    Ok(loft_profile_brep(&[end(-10.0)?, end(10.0)?])?)
}

/// A B-rep written as next.BREP's own topology (its public `TopologyArena`: vertices, edges, faces
/// and their loops), for a solid bounded by a surface the kernel has no operation to make a face
/// of: there is no "face from a surface", so each face is stated directly on its carrier, and the
/// kernel's `validate` checks the result.
pub struct Records {
    arena: TopologyArena,
    next: u64,
}

impl Records {
    pub fn new() -> Self {
        let arena = TopologyArena {
            wire_solid_id: 1, genus: 0, vertices: Default::default(), edges: Default::default(),
            coedges: Default::default(), loops: Default::default(), faces: Default::default(),
            shells: Default::default(),
        };
        Records { arena, next: 1 }
    }

    fn id(&mut self) -> u64 {
        self.next += 1;
        self.next
    }

    /// A vertex at `point`.
    pub fn vertex(&mut self, point: Vec3) -> VertexId {
        let wire_id = self.id();
        self.arena.vertices.insert(ArenaVertex { wire_id, point })
    }

    /// An edge on `curve`, over its whole domain, from `start` to `end`.
    pub fn edge(&mut self, curve: NurbsCurve, start: VertexId, end: VertexId)
        -> Result<EdgeId, String> {
        self.edge_of(curve, start, end, false)
    }

    /// The degenerate edge of a pole at the vertex `at`: a surface's edge collapsed to a point.
    pub fn pole(&mut self, at: VertexId) -> Result<EdgeId, String> {
        let point = self.arena.vertices[at].point;
        self.edge_of(make_line(point, point)?, at, at, true)
    }

    fn edge_of(&mut self, curve: NurbsCurve, start: VertexId, end: VertexId, degenerate: bool)
        -> Result<EdgeId, String> {
        let [t0, t1] = curve.domain()?;
        let wire_id = self.id();
        Ok(self.arena.edges.insert(ArenaEdge { wire_id, curve, t0, t1, start, end, degenerate,
                                               name: None }))
    }

    /// The curve of an edge.
    pub fn curve(&self, edge: EdgeId) -> NurbsCurve {
        self.arena.edges[edge].curve.clone()
    }

    /// A face on `surface` (its normal outward) bounded by one loop, each coedge an edge, whether
    /// it runs along the edge, and its pcurve in the surface's parameters, running as the coedge
    /// does.
    pub fn face(&mut self, surface: NurbsSurface, coedges: Vec<(EdgeId, bool, NurbsCurve)>) {
        let mut ids = Vec::new();
        for (edge, forward, pcurve) in coedges {
            let wire_id = self.id();
            ids.push(self.arena.coedges.insert(ArenaCoedge { wire_id, edge, forward, pcurve }));
        }
        let wire_id = self.id();
        let loop_id = self.arena.loops.insert(ArenaLoop { wire_id, coedges: ids });
        let wire_id = self.id();
        self.arena.faces.insert(ArenaFace { wire_id, surface, same_sense: true,
                                            loops: vec![loop_id], name: None });
    }

    /// A flat face on `plane` (its normal outward) bounded by one loop of edges, each run
    /// forward or not; the pcurves are the kernel's (`build_pcurve_on_surface`, exact on a plane).
    pub fn flat(&mut self, plane: NurbsSurface, coedges: &[(EdgeId, bool)]) -> Result<(), String> {
        let mut with_pcurves = Vec::new();
        for &(edge, forward) in coedges {
            let curve = self.curve(edge);
            let run = if forward { curve } else { curve.reversed()? };
            with_pcurves.push((edge, forward, build_pcurve_on_surface(&plane, &run)?));
        }
        self.face(plane, with_pcurves);
        Ok(())
    }

    /// The solid these faces bound, one closed shell of the given genus, as the kernel validates
    /// it.
    pub fn solid(mut self, genus: i64) -> Result<BrepSolid, String> {
        let faces = self.arena.faces.keys().collect();
        let wire_id = self.id();
        self.arena.shells.insert(ArenaShell { wire_id, faces });
        self.arena.genus = genus;
        self.arena.wire_solid_id = self.id();
        let solid = self.arena.to_brep()?;
        let issues = solid.validate();
        if issues.is_empty() {
            Ok(solid)
        } else {
            Err(format!("the faces do not bound a valid solid: {issues:?}"))
        }
    }
}

/// The straight segment from (u0, v0) to (u1, v1) of a surface's parameter plane: a pcurve.
pub fn uv_line(u0: f64, v0: f64, u1: f64, v1: f64) -> Result<NurbsCurve, String> {
    make_line(p(u0, v0, 0.0), p(u1, v1, 0.0))
}
