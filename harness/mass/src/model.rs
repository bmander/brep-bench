//! Reading the file: its units, points and frames, curves and surfaces.
use crate::curve::*;
use crate::geom::*;
use crate::py::*;
use crate::step::*;
use crate::surface::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub struct Model {
    pub entities: Entities,
    args: RefCell<HashMap<u64, Rc<Parts>>>,
    curves: RefCell<HashMap<u64, Arc<Curve>>>,
    surfaces: RefCell<HashMap<u64, Arc<Surface>>>,
    /// Millimetres per the file's unit of length.
    pub length: f64,
    /// Radians per the file's unit of plane angle.
    pub angle: f64,
}

/// An entity's arguments, as `Model.one` gives them: a simple entity's name and list, or (with no
/// name) a complex entity's parts.
pub struct One(Rc<Parts>);

impl One {
    pub fn name(&self) -> Option<&str> {
        if self.0.len() == 1 {
            Some(&self.0[0].0)
        } else {
            None
        }
    }

    /// The name as an f-string writes it.
    pub fn named(&self) -> &str {
        self.name().unwrap_or("None")
    }

    pub fn is(&self, name: &str) -> bool {
        self.name() == Some(name)
    }

    /// `a[i]`.
    pub fn at(&self, i: usize) -> R<&Val> {
        if self.0.len() == 1 {
            self.0[0].1.at(i)
        } else {
            Err(Err::Py("KeyError", i.to_string()))
        }
    }

    /// `len(a)`.
    pub fn len(&self) -> R<usize> {
        if self.0.len() == 1 {
            self.0[0].1.len()
        } else {
            Ok(self.0.len())
        }
    }
}

fn key_error<T>(name: &str) -> R<T> {
    Err(Err::Py("KeyError", str_repr(name)))
}

/// `next(...)` of nothing.
fn stop_iteration<T>() -> R<T> {
    Err(Err::Uncaught("StopIteration", String::new()))
}

impl Model {
    pub fn new(text: &[u8]) -> R<Model> {
        let mut m = Model {
            entities: parse(text),
            args: RefCell::new(HashMap::new()),
            curves: RefCell::new(HashMap::new()),
            surfaces: RefCell::new(HashMap::new()),
            length: 1.0,
            angle: 1.0,
        };
        m.units()?;
        Ok(m)
    }

    /// The number of the entity `r` names, if there can be one.
    fn id(&self, r: &Val) -> R<Option<u64>> {
        match r {
            Val::Ref(n) => Ok(Some(*n)),
            Val::Num(x) if *x >= 0.0 && *x == x.trunc() && *x < 1.8e19 => Ok(Some(*x as u64)),
            Val::List(_) => Err(Err::Py("TypeError", "unhashable type: 'list'".into())),
            _ => Ok(None),
        }
    }

    pub fn args_of(&self, id: u64) -> R<Rc<Parts>> {
        if let Some(a) = self.args.borrow().get(&id) {
            return Ok(a.clone());
        }
        let Some(e) = self.entities.get(id) else {
            return unsupported(format!("#{} is named but not defined", id));
        };
        let a = Rc::new(arguments(&e.text)?);
        self.args.borrow_mut().insert(id, a.clone());
        Ok(a)
    }

    pub fn args(&self, r: &Val) -> R<Rc<Parts>> {
        match self.id(r)? {
            Some(id) => self.args_of(id),
            None => unsupported(format!("#{} is named but not defined", r.str())),
        }
    }

    /// A simple entity's name and arguments.
    pub fn one(&self, r: &Val) -> R<One> {
        Ok(One(self.args(r)?))
    }

    fn units(&mut self) -> R<()> {
        for k in 0..self.entities.list.len() {
            let e = &self.entities.list[k];
            let id = e.id;
            let (length, angle) = (
                e.names.iter().any(|n| n == "LENGTH_UNIT"),
                e.names.iter().any(|n| n == "PLANE_ANGLE_UNIT"),
            );
            if length {
                self.length = self.unit(&Val::Ref(id), true)?;
            }
            if angle {
                self.angle = self.unit(&Val::Ref(id), false)?;
            }
        }
        Ok(())
    }

    fn unit(&self, r: &Val, length: bool) -> R<f64> {
        let a = self.args(r)?;
        if let Some(si) = part(&a, "SI_UNIT") {
            let prefix = if si.len()? >= 2 {
                let n = si.len()?;
                si.at(n - 2)?.clone()
            } else {
                si.at(0)?;
                Val::None
            };
            let scale = match &prefix {
                Val::None => 1.0,
                Val::List(_) => return Err(Err::Py("TypeError", "unhashable type: 'list'".into())),
                p => match p.as_str() {
                    Some("MILLI") => 1e-3,
                    Some("CENTI") => 1e-2,
                    Some("DECI") => 1e-1,
                    Some("KILO") => 1e3,
                    Some("MICRO") => 1e-6,
                    Some("NANO") => 1e-9,
                    _ => return unsupported(format!("a unit prefixed {}", prefix.str())),
                },
            };
            if length {
                return Ok(scale * 1000.0); // metres to millimetres
            }
            return Ok(scale);
        }
        if let Some(cb) = part(&a, "CONVERSION_BASED_UNIT") {
            let measure = cb.at(1)?;
            let ma = self.args(measure)?;
            for (k, v) in ma.iter() {
                if k.ends_with("MEASURE_WITH_UNIT") {
                    return Ok(v.at(0)?.float()? * self.unit(v.at(1)?, length)?);
                }
            }
        }
        unsupported(format!(
            "a unit of {} it does not know: #{}",
            if length { "length" } else { "angle" },
            r.str()
        ))
    }

    // -- points, frames

    /// A `CARTESIAN_POINT`'s coordinates as written, none scaled by the file's length unit: a
    /// point of a surface's parameters, in a pcurve.
    fn raw_point(&self, r: &Val) -> R<V3> {
        let a = self.one(r)?;
        if !a.is("CARTESIAN_POINT") {
            return unsupported(format!("a point {}", a.named()));
        }
        let mut c = [0.0; 3];
        for (i, x) in a.at(1)?.items()?.iter().enumerate() {
            if i < 3 {
                c[i] = x.float()?;
            }
        }
        Ok(c)
    }

    pub fn point(&self, r: &Val) -> R<V3> {
        let a = self.one(r)?;
        if a.is("CARTESIAN_POINT") {
            let mut c = [0.0; 3];
            for (i, x) in a.at(1)?.items()?.iter().enumerate() {
                let x = x.float()? * self.length;
                if i < 3 {
                    c[i] = x;
                }
            }
            return Ok(c);
        }
        if a.is("VERTEX_POINT") {
            return self.point(a.at(1)?);
        }
        unsupported(format!("#{} is not a point ({})", r.str(), a.named()))
    }

    fn direction(&self, r: &Val) -> R<V3> {
        let a = self.one(r)?;
        if !a.is("DIRECTION") {
            return unsupported(format!("#{} is not a direction ({})", r.str(), a.named()));
        }
        let mut c = [0.0; 3];
        for (i, x) in a.at(1)?.items()?.iter().enumerate() {
            let x = x.float()?;
            if i < 3 {
                c[i] = x;
            }
        }
        Ok(c)
    }

    fn frame(&self, r: &Val) -> R<Frame> {
        let a = self.one(r)?;
        if !a.is("AXIS2_PLACEMENT_3D") {
            return unsupported(format!("a placement {}", a.named()));
        }
        let o = self.point(a.at(1)?)?;
        let z = if !a.at(2)?.is_none() {
            unit(self.direction(a.at(2)?)?)?
        } else {
            [0.0, 0.0, 1.0]
        };
        let x = if a.len()? > 3 && !a.at(3)?.is_none() {
            self.direction(a.at(3)?)?
        } else if z[0].abs() < 0.9 {
            [1.0, 0.0, 0.0]
        } else {
            [0.0, 1.0, 0.0]
        };
        let x = unit(sub(x, mul(z, dot(x, z))))?;
        Ok(Frame {
            o,
            x,
            y: cross(z, x),
            z,
        })
    }

    // -- curves

    /// **The curve an edge runs along on a face's surface, where the file gives it**: of the edge's
    /// `SURFACE_CURVE` or `SEAM_CURVE` (`geom`), a `PCURVE` on `surface` itself, its curve (a
    /// B-spline or a line) in the surface's parameters, over the edge curve's own parameter: `(u,
    /// v, 0)` at `t`. None where there is none, or it is of another kind. A surface that comes near
    /// itself (a swept sheet folded back past its face) gives an edge's points two feet on it, and
    /// the curve says which is the face's.
    pub fn pcurve_on(&self, geom: &Val, surface: &Val) -> R<Option<Arc<Curve>>> {
        let a = self.args(geom)?;
        let Some((_, v)) = a
            .iter()
            .find(|(n, _)| matches!(n.as_str(), "SURFACE_CURVE" | "SEAM_CURVE"))
        else {
            return Ok(None);
        };
        let on = self.id(surface)?;
        for pc in v.at(2)?.items()? {
            let pa = self.args(pc)?;
            let Some(p) = part(&pa, "PCURVE") else {
                continue;
            };
            if self.id(p.at(1)?)? != on {
                continue;
            }
            let da = self.args(p.at(2)?)?;
            let Some(d) = part(&da, "DEFINITIONAL_REPRESENTATION") else {
                continue;
            };
            for item in d.at(1)?.items()? {
                let ca = self.args(item)?;
                if let Some(v) = part(&ca, "LINE") {
                    let v = v.unpack(3)?;
                    let va = self.one(&v[2])?;
                    if !va.is("VECTOR") {
                        continue;
                    }
                    let dir = self.one(va.at(1)?)?;
                    let mut d = [0.0; 3];
                    for (i, x) in dir.at(1)?.items()?.iter().enumerate() {
                        if i < 3 {
                            d[i] = x.float()?;
                        }
                    }
                    let m = va.at(2)?.float()?;
                    let o = self.raw_point(&v[1])?;
                    return Ok(Some(Arc::new(Curve::line(o, [d[0] * m, d[1] * m, d[2] * m]))));
                }
                if ca.iter().any(|(n, _)| n.starts_with("B_SPLINE_CURVE")) {
                    return Ok(Some(Arc::new(self.bspline_curve(&ca, false)?)));
                }
            }
        }
        Ok(None)
    }

    pub fn curve(&self, r: &Val) -> R<Arc<Curve>> {
        let id = self.id(r)?;
        if let Some(id) = id {
            if let Some(c) = self.curves.borrow().get(&id) {
                return Ok(c.clone());
            }
            if self.surfaces.borrow().contains_key(&id) {
                return Err(Err::Uncaught(
                    "AttributeError",
                    "a surface named as a curve".into(),
                ));
            }
        }
        let c = self.read_curve(r)?;
        if let Some(id) = id {
            self.curves.borrow_mut().insert(id, c.clone());
        }
        Ok(c)
    }

    fn read_curve(&self, r: &Val) -> R<Arc<Curve>> {
        let a = self.args(r)?;
        let l = self.length;
        if let Some(v) = part(&a, "LINE") {
            let v = v.unpack(3)?;
            let va = self.one(&v[2])?;
            if !va.is("VECTOR") {
                return unsupported(format!("a line's direction {}", va.named()));
            }
            let p = self.point(&v[1])?;
            let d = unit(self.direction(va.at(1)?)?)?;
            return Ok(Arc::new(Curve::line(p, mul(d, va.at(2)?.float()? * l))));
        }
        if let Some(v) = part(&a, "CIRCLE") {
            let v = v.unpack(3)?;
            let f = self.frame(&v[1])?;
            let r = v[2].float()? * l;
            return Ok(Arc::new(Curve::Conic { f, a: r, b: r }));
        }
        if let Some(v) = part(&a, "ELLIPSE") {
            let v = v.unpack(4)?;
            let f = self.frame(&v[1])?;
            return Ok(Arc::new(Curve::Conic {
                f,
                a: v[2].float()? * l,
                b: v[3].float()? * l,
            }));
        }
        if let Some(v) = part(&a, "PARABOLA") {
            let v = v.unpack(3)?;
            let f = self.frame(&v[1])?;
            return Ok(Arc::new(Curve::Open {
                f,
                parabola: true,
                a: v[2].float()? * l,
                b: 0.0,
            }));
        }
        if let Some(v) = part(&a, "HYPERBOLA") {
            let v = v.unpack(4)?;
            let f = self.frame(&v[1])?;
            return Ok(Arc::new(Curve::Open {
                f,
                parabola: false,
                a: v[2].float()? * l,
                b: v[3].float()? * l,
            }));
        }
        if let Some((_, v)) = a.iter().find(|(n, _)| {
            matches!(
                n.as_str(),
                "SURFACE_CURVE" | "SEAM_CURVE" | "INTERSECTION_CURVE"
            )
        }) {
            return self.curve(v.at(1)?);
        }
        if let Some(v) = part(&a, "TRIMMED_CURVE") {
            let basis = self.curve(v.at(1)?)?;
            return Ok(if !v.at(4)?.is("F") {
                basis
            } else {
                Arc::new(Curve::reversed(basis))
            });
        }
        if let Some(v) = part(&a, "POLYLINE") {
            let mut pts = Vec::new();
            for p in v.at(1)?.items()? {
                pts.push(self.point(p)?);
            }
            return Ok(Arc::new(Curve::Polyline(Polyline::new(pts)?)));
        }
        if a.iter().any(|(n, _)| {
            [
                "B_SPLINE_CURVE",
                "BEZIER_CURVE",
                "UNIFORM_CURVE",
                "QUASI_UNIFORM_CURVE",
            ]
            .iter()
            .any(|s| n.starts_with(s))
        }) {
            return Ok(Arc::new(self.bspline_curve(&a, true)?));
        }
        let mut names: Vec<&str> = a.iter().map(|(n, _)| n.as_str()).collect();
        names.sort();
        unsupported(format!("a curve {}", names.join("/")))
    }

    fn bspline_curve(&self, a: &Parts, scaled: bool) -> R<Curve> {
        let (p, pts, u, w);
        if a.len() == 1 {
            let (name, args) = &a[0];
            p = args.at(1)?.int()?;
            pts = args.at(2)?;
            u = if name == "B_SPLINE_CURVE_WITH_KNOTS" {
                knots(args.at(6)?, args.at(7)?)?
            } else {
                let n = pts.len()? as i64 - 1;
                implicit_knots(name, n, args.at(1)?.int()?)?
            };
            w = None;
        } else {
            let Some(base) = part(a, "B_SPLINE_CURVE") else {
                return key_error("B_SPLINE_CURVE");
            };
            p = base.at(0)?.int()?;
            pts = base.at(1)?;
            u = if let Some(k) = part(a, "B_SPLINE_CURVE_WITH_KNOTS") {
                knots(k.at(0)?, k.at(1)?)?
            } else {
                let Some((kind, _)) = a.iter().find(|(n, _)| {
                    matches!(
                        n.as_str(),
                        "BEZIER_CURVE" | "UNIFORM_CURVE" | "QUASI_UNIFORM_CURVE"
                    )
                }) else {
                    return stop_iteration();
                };
                implicit_knots(kind, pts.len()? as i64 - 1, p)?
            };
            w = match part(a, "RATIONAL_B_SPLINE_CURVE") {
                Some(r) => {
                    let mut w = Vec::new();
                    for x in r.at(0)?.items()? {
                        w.push(x.float()?);
                    }
                    Some(w)
                }
                None => None,
            };
        }
        let mut points = Vec::new();
        for q in pts.items()? {
            points.push(if scaled { self.point(q)? } else { self.raw_point(q)? });
        }
        Ok(Curve::BSpline(BSplineCurve::new(p, points, w, u)?))
    }

    // -- surfaces

    pub fn surface(&self, r: &Val) -> R<Arc<Surface>> {
        let id = self.id(r)?;
        if let Some(id) = id {
            if let Some(s) = self.surfaces.borrow().get(&id) {
                return Ok(s.clone());
            }
            if self.curves.borrow().contains_key(&id) {
                return Err(Err::Uncaught(
                    "AttributeError",
                    "a curve named as a surface".into(),
                ));
            }
        }
        let s = self.read_surface(r)?;
        if let Some(id) = id {
            self.surfaces.borrow_mut().insert(id, s.clone());
        }
        Ok(s)
    }

    fn read_surface(&self, r: &Val) -> R<Arc<Surface>> {
        let a = self.args(r)?;
        let l = self.length;
        if let Some(v) = part(&a, "PLANE") {
            return Ok(Arc::new(Surface::plane(self.frame(v.at(1)?)?)));
        }
        if let Some(v) = part(&a, "CYLINDRICAL_SURFACE") {
            let v = v.unpack(3)?;
            let f = self.frame(&v[1])?;
            return Ok(Arc::new(Surface::cylinder(f, v[2].float()? * l)));
        }
        if let Some(v) = part(&a, "CONICAL_SURFACE") {
            let v = v.unpack(4)?;
            let f = self.frame(&v[1])?;
            let r = v[2].float()? * l;
            return Ok(Arc::new(Surface::cone(f, r, v[3].float()? * self.angle)?));
        }
        if let Some(v) = part(&a, "SPHERICAL_SURFACE") {
            let v = v.unpack(3)?;
            let f = self.frame(&v[1])?;
            return Ok(Arc::new(Surface::sphere(f, v[2].float()? * l)));
        }
        if let Some(v) = part(&a, "DEGENERATE_TOROIDAL_SURFACE") {
            let v = v.unpack(5)?;
            let f = self.frame(&v[1])?;
            let major = v[2].float()? * l;
            return Ok(Arc::new(Surface::torus(
                f,
                major,
                v[3].float()? * l,
                Some(v[4].is("T")),
            )?));
        }
        if let Some(v) = part(&a, "TOROIDAL_SURFACE") {
            let v = v.unpack(4)?;
            let f = self.frame(&v[1])?;
            let major = v[2].float()? * l;
            return Ok(Arc::new(Surface::torus(f, major, v[3].float()? * l, None)?));
        }
        if let Some(v) = part(&a, "SURFACE_OF_REVOLUTION") {
            let v = v.unpack(3)?;
            let aa = self.one(&v[2])?;
            if !aa.is("AXIS1_PLACEMENT") {
                return unsupported(format!("an axis {}", aa.named()));
            }
            let axis = if !aa.at(2)?.is_none() {
                unit(self.direction(aa.at(2)?)?)?
            } else {
                [0.0, 0.0, 1.0]
            };
            let c = self.curve(&v[1])?;
            let a0 = self.point(aa.at(1)?)?;
            return Ok(Arc::new(Surface::revolution(c, a0, axis)?));
        }
        if let Some(v) = part(&a, "OFFSET_SURFACE") {
            let v = v.unpack(4)?;
            let b = self.surface(&v[1])?;
            return Ok(Arc::new(Surface::offset(b, v[2].float()? * l)));
        }
        if let Some(v) = part(&a, "SURFACE_OF_LINEAR_EXTRUSION") {
            let v = v.unpack(3)?;
            let va = self.one(&v[2])?;
            let c = self.curve(&v[1])?;
            let d = unit(self.direction(va.at(1)?)?)?;
            return Ok(Arc::new(Surface::extrusion(
                c,
                mul(d, va.at(2)?.float()? * l),
            )?));
        }
        if a.iter().any(|(n, _)| {
            [
                "B_SPLINE_SURFACE",
                "BEZIER_SURFACE",
                "UNIFORM_SURFACE",
                "QUASI_UNIFORM_SURFACE",
            ]
            .iter()
            .any(|s| n.starts_with(s))
        }) {
            return Ok(Arc::new(self.bspline_surface(&a)?));
        }
        let mut names: Vec<&str> = a.iter().map(|(n, _)| n.as_str()).collect();
        names.sort();
        unsupported(format!("a surface {}", names.join("/")))
    }

    fn bspline_surface(&self, a: &Parts) -> R<Surface> {
        let (pu, pv, grid, u, v, w);
        let first_row = |grid: &Val| -> R<i64> { Ok(grid.at(0)?.len()? as i64 - 1) };
        if a.len() == 1 {
            let (name, args) = &a[0];
            pu = args.at(1)?.int()?;
            pv = args.at(2)?.int()?;
            grid = args.at(3)?;
            if name == "B_SPLINE_SURFACE_WITH_KNOTS" {
                u = knots(args.at(8)?, args.at(10)?)?;
                v = knots(args.at(9)?, args.at(11)?)?;
            } else {
                u = implicit_knots(name, grid.len()? as i64 - 1, pu)?;
                v = implicit_knots(name, first_row(grid)?, pv)?;
            }
            w = None;
        } else {
            let Some(base) = part(a, "B_SPLINE_SURFACE") else {
                return key_error("B_SPLINE_SURFACE");
            };
            pu = base.at(0)?.int()?;
            pv = base.at(1)?.int()?;
            grid = base.at(2)?;
            if let Some(k) = part(a, "B_SPLINE_SURFACE_WITH_KNOTS") {
                u = knots(k.at(0)?, k.at(2)?)?;
                v = knots(k.at(1)?, k.at(3)?)?;
            } else {
                let Some((kind, _)) = a.iter().find(|(n, _)| {
                    matches!(
                        n.as_str(),
                        "BEZIER_SURFACE" | "UNIFORM_SURFACE" | "QUASI_UNIFORM_SURFACE"
                    )
                }) else {
                    return stop_iteration();
                };
                u = implicit_knots(kind, grid.len()? as i64 - 1, pu)?;
                v = implicit_knots(kind, first_row(grid)?, pv)?;
            }
            w = match part(a, "RATIONAL_B_SPLINE_SURFACE") {
                Some(r) => {
                    let mut w = Vec::new();
                    for row in r.at(0)?.items()? {
                        let mut ws = Vec::new();
                        for x in row.items()? {
                            ws.push(x.float()?);
                        }
                        w.push(ws);
                    }
                    Some(w)
                }
                None => None,
            };
        }
        let mut pts = Vec::new();
        for row in grid.items()? {
            let mut ps = Vec::new();
            for q in row.items()? {
                ps.push(self.point(q)?);
            }
            pts.push(ps);
        }
        Surface::bspline(pu, pv, pts, w, u, v)
    }
}

/// A B-spline's knots from their multiplicities (`expand`).
fn knots(mults: &Val, knots: &Val) -> R<Vec<f64>> {
    let (mults, knots) = (mults.items()?, knots.items()?);
    let mut u = Vec::new();
    for (m, k) in mults.iter().zip(knots) {
        let k = k.float()?;
        let m = m.int()?;
        for _ in 0..m.max(0) {
            u.push(k);
        }
    }
    Ok(u)
}
