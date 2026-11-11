//! What the cadrs implementations of the geometry cases share: ellipses in sketches; closed
//! sections with conics and B-splines in them (a sketch's spline, `Curve2::Spline`, rational or
//! not: a parabola, a hyperbola or a quadratic Bézier exactly), extruded or revolved; B-spline
//! surfaces from their control nets, put in place of a face; the saddle drum.
use crate::bench::{p3, P3, V3};
use crate::Refusal;
use cadrs_kernel::native::{NurbsSurface, Surface};
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, FaceReplacement, Kernel, Loop, Pick, Plane, Profile};
use std::sync::Arc;

/// The arc of the ellipse about `center` with semi-axes `a` along the sketch's x and `b` along
/// its y, from parameter `start` degrees through `sweep` degrees.
pub fn ellipse_arc(center: (f64, f64), a: f64, b: f64, start: f64, sweep: f64) -> Curve2 {
    Curve2::ellipse_arc([center.0, center.1], a, b, start.to_radians(), sweep.to_radians())
}

/// A closed section on a plane, its curves in the plane's coordinates, one after another: a
/// sketch's region.
/// Sketch points from the cases' coordinate pairs.
fn points(pairs: &[(f64, f64)]) -> Vec<[f64; 2]> {
    pairs.iter().map(|&(x, y)| [x, y]).collect()
}

pub struct Section {
    plane: Plane,
    curves: Vec<Curve2>,
}

impl Section {
    pub fn on(plane: Plane) -> Self {
        Section { plane, curves: Vec::new() }
    }

    /// The segment from `a` to `b`: a sketch line.
    pub fn line(mut self, a: (f64, f64), b: (f64, f64)) -> Self {
        self.curves.push(Curve2::line([a.0, a.1], [b.0, b.1]));
        self
    }

    /// The Bézier curve with these control points and weights (empty: a polynomial curve): a
    /// sketch spline of one span, its knots clamped.
    pub fn bezier(mut self, poles: &[(f64, f64)], weights: &[f64]) -> Self {
        self.curves.push(Curve2::rational_bezier(&points(poles), weights));
        self
    }

    /// The conic arc from `a` to `b` whose end tangents meet at `m`, weighted `w` there (a
    /// rational quadratic: a parabola when `w` = 1, a hyperbola above 1, an ellipse below).
    pub fn conic(self, a: (f64, f64), m: (f64, f64), w: f64, b: (f64, f64)) -> Self {
        self.bezier(&[a, m, b], &[1.0, w, 1.0])
    }

    /// The B-spline of this degree on these knots, poles and weights (empty: a polynomial
    /// curve).
    pub fn spline(
        mut self,
        degree: usize,
        knots: &[f64],
        poles: &[(f64, f64)],
        weights: &[f64],
    ) -> Self {
        self.curves.push(Curve2::spline(degree, knots, &points(poles), weights));
        self
    }

    /// A whole closed curve: the periodic B-spline of this degree on these poles (taken in
    /// order, closing back on the first), its knots evenly spaced.
    pub fn periodic(mut self, degree: usize, poles: &[(f64, f64)]) -> Self {
        self.curves.push(Curve2::periodic(degree, &points(poles)));
        self
    }

    pub fn sketch(&self) -> cadrs_kernel::Profile {
        Profile::of(self.plane, vec![Loop::new(self.curves.clone())])
    }

    /// The solid the section's region sweeps turned a whole turn about the axis through `origin`
    /// along `dir` (cadrs's Revolve).
    pub fn turned(&self, k: &mut DefaultKernel, origin: P3, dir: V3) -> Result<BodyId, Refusal> {
        crate::bench::revolve(k, &self.sketch(), Axis::new(origin, dir), None)
    }

    /// The solid the section's region sweeps slid along `v` (cadrs's Extrude).
    pub fn slid(&self, k: &mut DefaultKernel, v: V3) -> Result<BodyId, Refusal> {
        crate::bench::extrude_along(k, &self.sketch(), v, v.norm())
    }
}

/// The section turned a whole turn about the z axis.
pub fn turned_about_z(k: &mut DefaultKernel, section: &Section) -> Result<BodyId, Refusal> {
    section.turned(k, p3(0.0, 0.0, 0.0), V3::z())
}

/// A control point `(x, y, z)` of weight `w`, homogeneous as cadrs's B-splines hold them.
pub fn pole(x: f64, y: f64, z: f64, w: f64) -> nalgebra::Vector4<f64> {
    nalgebra::Vector4::new(w * x, w * y, w * z, w)
}

/// The clamped knots of a Bézier of degree `p` on [0, 1].
pub fn bezier_knots(p: usize) -> Vec<f64> {
    let mut knots = vec![0.0; p + 1];
    knots.extend(vec![1.0; p + 1]);
    knots
}

/// The saddle z = 10 + xy/8 of `geometry/saddle-drum` as a B-spline surface: the bilinear patch
/// (degree 1 each way, non-rational) on the corners of the square −10 ≤ x, y ≤ 10 at the
/// saddle's heights there.
pub fn saddle() -> Result<Surface, Refusal> {
    let s = NurbsSurface::new(
        1,
        1,
        bezier_knots(1),
        bezier_knots(1),
        vec![
            vec![pole(-10.0, -10.0, 22.5, 1.0), pole(-10.0, 10.0, -2.5, 1.0)],
            vec![pole(10.0, -10.0, -2.5, 1.0), pole(10.0, 10.0, 22.5, 1.0)],
        ],
    )?;
    Ok(Surface::Nurbs(Arc::new(s)))
}

/// `body` with its face through `point` given the surface `surface` (cadrs's Replace face,
/// SMLib's local operation): the edges round it recomputed where the new surface meets the
/// faces next to it.
pub fn replace_face(
    k: &mut DefaultKernel,
    body: BodyId,
    point: P3,
    surface: Surface,
) -> Result<BodyId, Refusal> {
    let face = k.face_at(body, point)?;
    Ok(k.replace_faces(body, &[FaceReplacement::new(face, surface)])?.first()?)
}

/// The drum of `geometry/saddle-drum`: a circle of radius 8 about the z axis on the Top plane,
/// extruded 20 (past the saddle's highest point over it, 14), its top face replaced by the
/// saddle.
pub fn saddle_drum(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let drum = crate::bench::cylinder(k, p3(0.0, 0.0, 0.0), V3::z(), 8.0, 20.0)?;
    replace_face(k, drum, p3(0.0, 0.0, 20.0), saddle()?)
}
