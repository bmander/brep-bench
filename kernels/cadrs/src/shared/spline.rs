//! What the cadrs implementations of the spline cases share: a planar face bounded by curves a
//! sketch cannot hold (B-splines and rational Béziers in space), and the solids an Extrude and a
//! Revolve make of it.
//!
//! cadrs's sketch curves are lines, arcs, circles, ellipses and polynomial cubic Béziers. A
//! (rational) B-spline enters the kernel as a curve of a wire body (`create_wire`, as cadrs's
//! Part Studio passes a curve feature's curve), and the planar face it bounds is its Fill (a
//! planar chain is filled with its plane); a solid Extrude or Revolve then takes that face as its
//! input, as Onshape's do a face.
use crate::bench::{unit, V3};
use crate::Refusal;
use cadrs_kernel::{
    Axis, BodyId, DefaultKernel, ExtrudeSpec, FaceInput, Kernel, PathCurve, Pick, Plane, Profile,
    RevolveSpec,
};

/// The planar face filling the closed chain `curves` (joined end to end into a wire), as the
/// face input of a feature.
pub fn filled(k: &mut DefaultKernel, curves: &[PathCurve]) -> Result<FaceInput, Refusal> {
    Ok(k.planar_face(curves)?)
}

/// The solid `face` (on `plane`) makes extruded `depth` along `dir`.
pub fn extrude_face(
    k: &mut DefaultKernel,
    plane: Plane,
    face: FaceInput,
    dir: V3,
    depth: f64,
) -> Result<BodyId, Refusal> {
    let spec = ExtrudeSpec::blind(unit(dir), depth).with_faces(vec![face]);
    Ok(k.extrude_with(&Profile::new(plane, vec![]), &spec)?.first()?)
}

/// The solid `face` (on `plane`) makes revolved a whole turn about `axis`.
pub fn revolve_face(
    k: &mut DefaultKernel,
    plane: Plane,
    face: FaceInput,
    axis: Axis,
) -> Result<BodyId, Refusal> {
    let spec = RevolveSpec::solid(axis, None).with_faces(vec![face]);
    Ok(k.revolve_with(&Profile::new(plane, vec![]), &spec)?.first()?)
}
