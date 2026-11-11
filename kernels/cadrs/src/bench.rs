//! What the cadrs implementations share beyond the kernel's own constructors (`Plane::xy`,
//! `Curve2::line`, `Loop::polygon`, `Profile::of`, `Axis::new`, and picking a face or an edge by
//! a point on it, `Pick::face_at`, which the cases call directly): the solids cadrs's Part
//! Studio makes from a sketch (an extrude, a revolve), the kernel's primitives, booleans and
//! rigid motions, each as the one body it makes.
//!
//! A block is a rectangle extruded and a cylinder a circle extruded, as a sketch-based modeller
//! such as Onshape makes them; a ball, a cone and a torus are the kernel's primitives
//! (`Kernel::primitive`), a torus whose tube is as wide as its ring or wider among them.
//!
//! Lengths are the cases' units (cadrs reads them as mm); angles here are in degrees, as the
//! cases give them, and turned into the kernel's radians.
use crate::Refusal;
use cadrs_kernel::{
    Axis, BodyId, BoolOp, Curve2, DefaultKernel, ExtrudeSpec, Kernel, Loop, Motion, Plane,
    Primitive, Profile, RevolveSpec, Transform,
};
use nalgebra::{Point2, Point3, Unit, Vector3};

pub type P2 = Point2<f64>;
pub type P3 = Point3<f64>;
pub type V3 = Vector3<f64>;

pub fn p2(x: f64, y: f64) -> P2 {
    P2::new(x, y)
}

pub fn p3(x: f64, y: f64, z: f64) -> P3 {
    P3::new(x, y, z)
}

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    V3::new(x, y, z)
}

pub fn unit(v: V3) -> Unit<V3> {
    Unit::new_normalize(v)
}

// Sketch curves (in the plane's coordinates).

/// The arc about `center` of `radius` from `start` degrees, sweeping `sweep` degrees
/// (counter-clockwise when positive).
pub fn arc(center: P2, radius: f64, start: f64, sweep: f64) -> Curve2 {
    Curve2::arc(center, radius, start.to_radians(), sweep.to_radians())
}

// Features.

/// `profile` extruded `depth` along its plane's normal (against it when negative).
pub fn extrude(k: &mut DefaultKernel, profile: &Profile, depth: f64) -> Result<BodyId, Refusal> {
    Ok(k.extrude_with(profile, &ExtrudeSpec::normal_to(profile, depth))?.first()?)
}

/// `profile` extruded `depth` along the direction `dir` (not in its plane).
pub fn extrude_along(
    k: &mut DefaultKernel,
    profile: &Profile,
    dir: V3,
    depth: f64,
) -> Result<BodyId, Refusal> {
    Ok(k.extrude_with(profile, &ExtrudeSpec::blind(unit(dir), depth))?.first()?)
}

/// `profile` extruded symmetrically: `depth` in all, half each side of its plane.
pub fn extrude_symmetric(
    k: &mut DefaultKernel,
    profile: &Profile,
    depth: f64,
) -> Result<BodyId, Refusal> {
    let spec = ExtrudeSpec::blind(profile.plane.normal, depth).both_ways();
    Ok(k.extrude_with(profile, &spec)?.first()?)
}

/// `profile` revolved about `axis` by `angle` degrees (right-handed), or a whole turn when
/// `None`.
pub fn revolve(
    k: &mut DefaultKernel,
    profile: &Profile,
    axis: Axis,
    angle: Option<f64>,
) -> Result<BodyId, Refusal> {
    Ok(k.revolve_with(profile, &RevolveSpec::solid(axis, angle.map(f64::to_radians)))?.first()?)
}

// Bodies made from sketches.

/// The block with opposite corners `min` and `max`: the rectangle on the plane z = min.z,
/// extruded up to max.z.
pub fn block(k: &mut DefaultKernel, min: P3, max: P3) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xy(min.z), vec![Loop::rectangle(min.x, min.y, max.x, max.y)]);
    extrude(k, &s, max.z - min.z)
}

/// The cylinder of `radius` whose base is centred on `base` and whose axis runs `length` along
/// `dir`: a circle on the plane through `base` square to `dir`, extruded.
pub fn cylinder(
    k: &mut DefaultKernel,
    base: P3,
    dir: V3,
    radius: f64,
    length: f64,
) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::square_to(base, dir), vec![Loop::circle([0.0, 0.0], radius)]);
    extrude(k, &s, length)
}

// The kernel's primitives.

/// The ball of `radius` about `center` (cadrs's sphere primitive), its poles on the z axis
/// through its centre and its seam toward +x.
pub fn ball(k: &mut DefaultKernel, center: P3, radius: f64) -> Result<BodyId, Refusal> {
    Ok(k.primitive(&Primitive::ball(center, radius))?.first()?)
}

/// The ball of `radius` about `center` (cadrs's sphere primitive), its poles along `axis` from
/// its centre and its seam toward `seam` from that axis.
pub fn ball_oriented(
    k: &mut DefaultKernel,
    center: P3,
    radius: f64,
    axis: V3,
    seam: V3,
) -> Result<BodyId, Refusal> {
    Ok(k.primitive(&Primitive::ball_with(center, radius, axis, seam))?.first()?)
}

/// The cone frustum on the axis from `base` along `dir`, `length` long, of radius `r0` at its
/// base and `r1` at its top (either may be 0; cadrs's cone primitive).
pub fn cone(
    k: &mut DefaultKernel,
    base: P3,
    dir: V3,
    r0: f64,
    r1: f64,
    length: f64,
) -> Result<BodyId, Refusal> {
    Ok(k.primitive(&Primitive::cone(base, dir, r0, r1, length))?.first()?)
}

/// The torus about the axis through `center` along `dir`, the circle through its tube's middle
/// of radius `major` and its tube of radius `minor` (cadrs's torus primitive: a ring, horn or
/// spindle torus).
pub fn torus(
    k: &mut DefaultKernel,
    center: P3,
    dir: V3,
    major: f64,
    minor: f64,
) -> Result<BodyId, Refusal> {
    Ok(k.primitive(&Primitive::torus(center, dir, major, minor))?.first()?)
}

// Booleans.

pub fn boolean(
    k: &mut DefaultKernel,
    op: BoolOp,
    target: BodyId,
    tools: &[BodyId],
) -> Result<BodyId, Refusal> {
    Ok(k.boolean(op, target, tools)?.first()?)
}

pub fn union(k: &mut DefaultKernel, a: BodyId, b: BodyId) -> Result<BodyId, Refusal> {
    boolean(k, BoolOp::Union, a, &[b])
}

pub fn cut(k: &mut DefaultKernel, a: BodyId, b: BodyId) -> Result<BodyId, Refusal> {
    boolean(k, BoolOp::Subtract, a, &[b])
}

pub fn common(k: &mut DefaultKernel, a: BodyId, b: BodyId) -> Result<BodyId, Refusal> {
    boolean(k, BoolOp::Intersect, a, &[b])
}

/// `body` less each of `tools`, one cut after another.
pub fn cut_each(k: &mut DefaultKernel, body: BodyId, tools: &[BodyId]) -> Result<BodyId, Refusal> {
    let mut b = body;
    for &t in tools {
        b = cut(k, b, t)?;
    }
    Ok(b)
}

/// `body` united with each of `tools`, one after another.
pub fn union_each(k: &mut DefaultKernel, body: BodyId, tools: &[BodyId]) -> Result<BodyId, Refusal> {
    let mut b = body;
    for &t in tools {
        b = union(k, b, t)?;
    }
    Ok(b)
}

// Rigid motions.

/// A copy of `body` moved by `v`.
pub fn moved(k: &mut DefaultKernel, body: BodyId, v: V3) -> Result<BodyId, Refusal> {
    Ok(k.transform(body, &Transform::translation(v.x, v.y, v.z))?.first()?)
}

/// A copy of `body` turned `angle` degrees (right-handed) about `axis`.
pub fn turned(k: &mut DefaultKernel, body: BodyId, axis: Axis, angle: f64) -> Result<BodyId, Refusal> {
    Ok(k.transform_motion(body, &Motion::rotation(&axis, angle.to_radians()))?.first()?)
}

/// A copy of `body` reflected in the plane through `point` with normal `normal`.
pub fn mirrored(k: &mut DefaultKernel, body: BodyId, point: P3, normal: V3) -> Result<BodyId, Refusal> {
    Ok(k.transform_motion(body, &Motion::reflection(point, unit(normal)))?.first()?)
}
