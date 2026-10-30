//! What the cases share: the solids truck-modeling has no primitive for, built the way truck's
//! own examples build them, and the one rule every Boolean's tolerance follows.
use std::f64::consts::PI;
use truck_modeling::*;

/// The tolerance of every Boolean, relative to the size of the smaller operand: truck-shapeops
/// meshes both operands to it to find where they meet and fits the curves it finds to it. This is
/// the scale of truck's own example (0.05 on a unit cube). Which cases truck builds changes with
/// it in no orderly way; no other tolerance from 1e-2 down to 2e-4 builds more of them, and no
/// case chooses its own.
const BOOLEAN_TOL: f64 = 3e-2;

/// The tolerance to pass to `truck_shapeops::and` or `or` for the operands `a` and `b`.
pub fn boolean_tolerance(a: &Solid, b: &Solid) -> f64 { BOOLEAN_TOL * size(a).min(size(b)) }

/// The diagonal of the bounding box of the solid's vertices. (Not of points along its edges: a
/// truck intersection curve can panic where it is evaluated.)
pub fn size(s: &Solid) -> f64 {
    let b: BoundingBox<Point3> = s.vertex_iter().map(|v| v.point()).collect();
    b.diameter()
}

/// The direction every circle starts from, square to its axis: +x for an axis along y or z, +y
/// for one along x. A circle's seam (where its first and last arcs meet) lies that way.
fn seam_direction(axis: Vector3) -> Vector3 {
    let e = match axis.x.abs() <= axis.y.abs() && axis.x.abs() <= axis.z.abs() {
        true => Vector3::unit_x(),
        false => Vector3::unit_y(),
    };
    (e - axis * e.dot(axis)).normalize()
}

/// A solid cylinder: the disc of `radius` about `base`, square to `axis` (a unit vector), swept
/// `height` along it, as truck's `cylinder` example builds one. The circle runs counterclockwise
/// about `axis`, so the disc faces along it and the swept solid comes out right way out.
pub fn cylinder(base: Point3, axis: Vector3, radius: f64, height: f64) -> Solid {
    let rim = builder::vertex(base + seam_direction(axis) * radius);
    let circle: Wire = builder::rsweep(&rim, base, axis, Rad(2.0 * PI), 4);
    let disc = builder::try_attach_plane(&[circle]).expect("a circle lies in a plane");
    builder::tsweep(&disc, axis * height)
}

/// A ball, as truck's `sphere` example builds one: a meridian from pole to pole in three arcs (so
/// that no edge runs round the equator), turned once about the z axis by `builder::cone`, which
/// closes the poles. The meridian runs down, against the axis, or the ball comes out inside out.
pub fn ball(centre: Point3, radius: f64) -> Solid {
    let pole = builder::vertex(centre + Vector3::unit_z() * radius);
    let meridian: Wire = builder::rsweep(&pole, centre, Vector3::unit_y(), Rad(PI), 3);
    let shell = builder::cone(&meridian, Vector3::unit_z(), Rad(2.0 * PI), 4);
    Solid::new(vec![shell])
}

/// A torus about the z axis through `centre`, as truck's `torus` example builds one: the circle
/// of radius `tube` about the point `ring` out along +x, in the xz plane, starting at its top,
/// turned once about the z axis.
pub fn torus(centre: Point3, ring: f64, tube: f64) -> Solid {
    let middle = centre + Vector3::unit_x() * ring;
    let top = builder::vertex(middle + Vector3::unit_z() * tube);
    let section: Wire = builder::rsweep(&top, middle, Vector3::unit_y(), Rad(2.0 * PI), 4);
    Solid::new(vec![builder::rsweep(&section, centre, Vector3::unit_z(), Rad(2.0 * PI), 4)])
}
