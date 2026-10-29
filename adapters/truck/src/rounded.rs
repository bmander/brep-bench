//! Rounding an edge with truck-shapeops' `fillet_with_side`.
//!
//! The fillet needs curve and surface types that can hold its own geometry (a rolling-ball
//! surface, curves on it, curves where it meets a side face), which truck-modeling's `Curve` and
//! `Surface` cannot. So, as truck-shapeops' own fillet tests do, this module declares those types
//! with truck's derive macros and carries the target's geometry over into them unchanged.
use serde_json::Value;
use truck_geometry::prelude::*;
use truck_shapeops::fillet::{fillet_with_side, FilletWithSide, SimpleFillet};

use crate::Built;

#[derive(
    Clone,
    Debug,
    ParametricCurve,
    BoundedCurve,
    Cut,
    SearchNearestParameterD1,
    ParameterDivision1D,
    Invertible,
)]
pub enum Curve {
    Line(Line<Point3>),
    Nurbs(NurbsCurve<Vector4>),
    Parametric(PCurve<BSplineCurve<Point2>, Box<Surface>>),
    Intersection(IntersectionCurve<Box<Curve>, Box<Surface>, Box<Surface>>),
}

impl ToSameGeometry<Curve> for IntersectionCurve<Curve, Surface, Surface> {
    fn to_same_geometry(&self) -> Curve {
        let (surface0, surface1, leader) = self.clone().destruct();
        Curve::Intersection(IntersectionCurve::new(
            Box::new(surface0),
            Box::new(surface1),
            Box::new(leader),
        ))
    }
}

impl ToSameGeometry<Curve> for PCurve<BSplineCurve<Point2>, Surface> {
    fn to_same_geometry(&self) -> Curve {
        let (curve, surface) = self.clone().decompose();
        Curve::Parametric(PCurve::new(curve, Box::new(surface)))
    }
}

#[derive(
    Clone,
    Debug,
    ParametricSurface3D,
    SearchParameterD2,
    SearchNearestParameterD2,
    ParameterDivision2D,
)]
pub enum Surface {
    Plane(Plane),
    Nurbs(NurbsSurface<Vector4>),
    Fillet(ApproxFilletSurface<Box<Surface>, Box<Surface>>),
    Processor(Processor<Box<Surface>, Matrix4>),
}

impl ToSameGeometry<Surface> for ApproxFilletSurface<Surface, Surface> {
    fn to_same_geometry(&self) -> Surface {
        Surface::Fillet(self.clone().into())
    }
}

impl Invertible for Surface {
    fn invert(&mut self) {
        match self {
            Self::Plane(plane) => plane.invert(),
            Self::Nurbs(surface) => surface.invert(),
            Self::Fillet(_) => {
                let mut processor = Processor::new(Box::new(self.clone()));
                processor.invert();
                *self = Self::Processor(processor);
            }
            Self::Processor(processor) => processor.invert(),
        }
    }
}

truck_topology::prelude!(Point3, Curve, Surface, pub);

pub fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(_) => "plane",
        Surface::Nurbs(_) => "bspline",
        Surface::Fillet(_) | Surface::Processor(_) => "other",
    }
}

/// truck-modeling's geometry, carried over as it is.
fn curve(c: &truck_modeling::Curve) -> Option<Curve> {
    match c {
        truck_modeling::Curve::Line(l) => Some(Curve::Line(*l)),
        truck_modeling::Curve::BSplineCurve(b) => Some(Curve::Nurbs(NurbsCurve::from(b.clone()))),
        truck_modeling::Curve::NurbsCurve(n) => Some(Curve::Nurbs(n.clone())),
        truck_modeling::Curve::IntersectionCurve(_) => None,
    }
}

fn surface(s: &truck_modeling::Surface) -> Option<Surface> {
    match s {
        truck_modeling::Surface::Plane(p) => Some(Surface::Plane(*p)),
        truck_modeling::Surface::BSplineSurface(b) => {
            Some(Surface::Nurbs(NurbsSurface::from(b.clone())))
        }
        truck_modeling::Surface::NurbsSurface(n) => Some(Surface::Nurbs(n.clone())),
        truck_modeling::Surface::RevolutedCurve(_) => None,
    }
}

fn samples(e: &Edge) -> Vec<Point3> {
    let c = e.curve();
    let (t0, t1) = c.range_tuple();
    (0..=256)
        .map(|k| c.subs(t0 + (t1 - t0) * k as f64 / 256.0))
        .collect()
}

/// The diagonal of the bounding box of the solid's vertices.
pub fn size_of(s: &Solid) -> f64 {
    let b: BoundingBox<Point3> = s.vertex_iter().map(|v| v.point()).collect();
    b.diameter()
}

/// The edges the selector picks, each once: every edge, or for each point the one nearest it
/// (by sampling the edges' curves).
fn selected(shell: &Shell, sel: &Value) -> Built<Vec<Edge>> {
    let mut edges: Vec<Edge> = Vec::new();
    for e in shell.edge_iter() {
        if !edges.iter().any(|d| d.id() == e.id()) {
            edges.push(e);
        }
    }
    if sel.get("all").and_then(Value::as_bool) == Some(true) {
        return Ok(edges);
    }
    let points = sel
        .get("nearest")
        .and_then(Value::as_array)
        .ok_or("an unknown edge selector")?;
    let mut out: Vec<Edge> = Vec::new();
    for p in points {
        let p = Point3::from_vec(crate::vec3(p)?);
        let distance = |e: &Edge| {
            samples(e)
                .iter()
                .map(|q| q.distance(p))
                .fold(f64::MAX, f64::min)
        };
        let best = edges
            .iter()
            .min_by(|a, b| distance(a).total_cmp(&distance(b)))
            .ok_or("a solid with no edges")?;
        if !out.iter().any(|d| d.id() == best.id()) {
            out.push(best.clone());
        }
    }
    Ok(out)
}

/// The faces of `shell` with an edge for which `test` holds.
fn faces_with(shell: &Shell, test: impl Fn(&Edge) -> bool) -> Vec<usize> {
    (0..shell.len())
        .filter(|&i| shell[i].edge_iter().any(|e| test(&e)))
        .collect()
}

fn touches(e: &Edge, v: &Vertex) -> bool {
    e.front().id() == v.id() || e.back().id() == v.id()
}

/// The direction of `e`'s curve where it meets `v`, either way along it.
fn tangent_at(e: &Edge, v: &Vertex) -> Vector3 {
    let c = e.curve();
    let (t0, t1) = c.range_tuple();
    let t = match c.subs(t0).distance(v.point()) <= c.subs(t1).distance(v.point()) {
        true => t0,
        false => t1,
    };
    c.der(t).normalize()
}

/// Whether another edge leaves an end of `edge` in the direction `edge` arrives there, so that
/// the two are one smooth curve split in two.
fn runs_on(shell: &Shell, edge: &Edge) -> bool {
    [edge.front(), edge.back()].into_iter().any(|v| {
        let t = tangent_at(edge, v);
        shell.edge_iter().any(|e| {
            e.id() != edge.id() && touches(&e, v) && tangent_at(&e, v).cross(t).magnitude() < 1e-6
        })
    })
}

/// `target` with the selected edges rounded to `radius`, one `fillet_with_side` per edge.
pub fn fillet(target: &truck_modeling::Solid, radius: f64, sel: &Value, tol: f64) -> Built<Solid> {
    let unsupported = "truck-shapeops rounds only a single edge whose ends each meet exactly \
                       three faces (`fillet_with_side`); its fillet along continuous edges is \
                       unsupported (commented out in truck-shapeops)";
    let mut shells = target
        .boundaries()
        .iter()
        .map(|shell| shell.try_mapped(|p| Some(*p), curve, surface));
    let shell = match (shells.next(), shells.next()) {
        (Some(Some(shell)), None) => shell,
        _ => return Err("a fillet target whose geometry is not lines, NURBS and planes".into()),
    };
    // `fillet_with_side` trims a side face stored inverted (as tsweep stores the face it starts
    // from) into an open wire and panics, so each face is restated forward: the same edges, its
    // surface inverted
    let mut shell: Shell = shell
        .face_iter()
        .map(|f| match f.orientation() {
            true => f.clone(),
            false => Face::new(f.boundaries(), f.oriented_surface()),
        })
        .collect();
    let edges = selected(&shell, sel)?;
    for (i, a) in edges.iter().enumerate() {
        if edges[i + 1..]
            .iter()
            .any(|b| touches(b, a.front()) || touches(b, a.back()))
        {
            return Err(format!("{unsupported}: the selected edges meet at a vertex").into());
        }
        if runs_on(&shell, a) {
            return Err(format!(
                "{unsupported}: the selected edge runs on smoothly into the next one, so the \
                 curve to round is a chain of edges (truck draws a circle as two edges or more: \
                 its `Edge::new` takes no closed edge)"
            )
            .into());
        }
    }
    for edge in edges {
        let on_edge = faces_with(&shell, |e| e.id() == edge.id());
        let &[i0, i1] = on_edge.as_slice() else {
            return Err(
                format!("{unsupported}: the selected edge does not bound two faces").into(),
            );
        };
        // `fillet_with_side` trims `side0` at the front of the edge as `face0` runs it
        let run = shell[i0]
            .edge_iter()
            .find(|e| e.id() == edge.id())
            .expect("the face holds it");
        let side = |v: &Vertex| -> Built<usize> {
            let at = faces_with(&shell, |e| touches(e, v));
            match at.len() == 3 {
                true => Ok(at
                    .into_iter()
                    .find(|&i| i != i0 && i != i1)
                    .expect("a third face")),
                false => Err(format!(
                    "{unsupported}: an end of the selected edge meets {} faces",
                    at.len()
                )
                .into()),
            }
        };
        let (s0, s1) = (side(run.front())?, side(run.back())?);
        let FilletWithSide {
            simple_fillet:
                SimpleFillet {
                    face0,
                    face1,
                    fillet,
                },
            side0,
            side1,
        } = fillet_with_side(
            &shell[i0],
            &shell[i1],
            edge.id(),
            Some(&shell[s0]),
            Some(&shell[s1]),
            radius,
            tol,
        )
        .ok_or("truck-shapeops `fillet_with_side` returned None")?;
        let (Some(side0), Some(side1)) = (side0, side1) else {
            return Err("truck-shapeops `fillet_with_side` trimmed no side face".into());
        };
        shell[i0] = face0;
        shell[i1] = face1;
        shell[s0] = side0;
        shell[s1] = side1;
        shell.push(fillet);
    }
    Solid::try_new(vec![shell]).map_err(|e| {
        format!("the shell `fillet_with_side` leaves is not a solid in truck-topology: {e}").into()
    })
}
