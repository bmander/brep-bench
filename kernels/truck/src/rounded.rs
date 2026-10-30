//! The geometry types a solid with a rounded edge is built in.
//!
//! truck-shapeops' fillet (`fillet_with_side`) makes geometry of its own (a rolling-ball surface,
//! curves on it, curves where it meets a side face) that truck-modeling's `Curve` and `Surface`
//! cannot hold. So, as truck-shapeops' own fillet tests do, a solid to be rounded is built in
//! types declared here with truck's derive macros; truck-modeling's `primitive::cuboid` and
//! `builder` are generic and build in them directly.
use truck_geometry::prelude::*;

#[derive(
    Clone,
    Debug,
    ParametricCurve,
    BoundedCurve,
    Cut,
    SearchNearestParameterD1,
    SearchParameterD1,
    ParameterDivision1D,
    Invertible,
)]
pub enum Curve {
    Line(Line<Point3>),
    Nurbs(NurbsCurve<Vector4>),
    Parametric(PCurve<BSplineCurve<Point2>, Box<Surface>>),
    Intersection(IntersectionCurve<Box<Curve>, Box<Surface>, Box<Surface>>),
}

impl ToSameGeometry<Curve> for Line<Point3> {
    fn to_same_geometry(&self) -> Curve { Curve::Line(*self) }
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

impl ToSameGeometry<Surface> for Plane {
    fn to_same_geometry(&self) -> Surface { Surface::Plane(*self) }
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

/// The tolerance a fillet's surface is approximated to, relative to the size of the solid
/// rounded (`size`).
const FILLET_TOL: f64 = 1e-4;

/// The tolerance to pass to `fillet_with_side` when rounding an edge of `s`.
pub fn fillet_tolerance(s: &Shell) -> f64 { FILLET_TOL * size_of_vertices(s.vertex_iter()) }

pub fn kind(s: &Surface) -> &'static str {
    match s {
        Surface::Plane(_) => "plane",
        Surface::Nurbs(_) => "bspline",
        Surface::Fillet(_) | Surface::Processor(_) => "other",
    }
}

/// The diagonal of the bounding box of the solid's vertices.
pub fn size(s: &Solid) -> f64 { size_of_vertices(s.vertex_iter()) }

fn size_of_vertices(vertices: impl Iterator<Item = Vertex>) -> f64 {
    let b: BoundingBox<Point3> = vertices.map(|v| v.point()).collect();
    b.diameter()
}

/// `shell` with every face that is stored back to front restated forward: the same boundary
/// read the other way, on its surface turned over, made with truck-topology's public
/// `Face::new`. The solid is the same.
///
/// `fillet_with_side` trims a side face stored inverted into an open wire and panics. A solid
/// swept by `builder::tsweep` stores the face it starts from inverted, so a shell made that way
/// goes through this before it is rounded. `primitive::cuboid` stores every face forward, so
/// `fillet/box-one-edge` has no need of it.
#[allow(dead_code)]
pub fn restated_forward(shell: &Shell) -> Shell {
    shell
        .face_iter()
        .map(|f| match f.orientation() {
            true => f.clone(),
            false => Face::new(f.boundaries(), f.oriented_surface()),
        })
        .collect()
}
