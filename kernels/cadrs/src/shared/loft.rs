//! What the cadrs implementations of the loft cases share: cadrs's solid Loft through sketched
//! sections, and the pieces of a loft cadrs's solid Loft cannot make (it takes no guides, and
//! pairs its sections' curves itself), built as Onshape's surfacing would: each wall a surface
//! Loft through curves (with guides, or to a point), each flat end a Fill, sewn into a solid.
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveLoftSpec, CurveSection, DefaultKernel, FillSpec, Kernel, LoftSection, LoftSpec, PathCurve, Plane, Profile};

/// The solid lofted through `sections` in order, with no end conditions.
pub fn loft(k: &mut DefaultKernel, sections: Vec<Profile>) -> Result<BodyId, Refusal> {
    Ok(k.loft_with(&LoftSpec::through(sections))?.first()?)
}

/// The sheet lofted through the curves `sections` in order (each a wire, or a point at either
/// end) along `guides`.
pub fn surface_loft(
    k: &mut DefaultKernel,
    sections: Vec<CurveSection>,
    guides: Vec<Vec<PathCurve>>,
) -> Result<BodyId, Refusal> {
    Ok(k.loft_curves(&CurveLoftSpec::new(sections, guides))?.first()?)
}

/// The flat sheet filling the polygon through `corners` on the plane z = `z` (its Fill).
pub fn flat(k: &mut DefaultKernel, z: f64, corners: &[(f64, f64)]) -> Result<BodyId, Refusal> {
    let corners: Vec<[f64; 2]> = corners.iter().map(|&(x, y)| [x, y]).collect();
    Ok(k.fill(&FillSpec::polygon(Plane::xy(z), &corners))?.first()?)
}

/// The solid the sheets `pieces` close, sewn along their edges.
pub fn sewn(k: &mut DefaultKernel, pieces: &[BodyId]) -> Result<BodyId, Refusal> {
    Ok(k.sew_solid(pieces, 1e-6)?.first()?)
}

/// The solid lofted through `sections` (each a sketch region, or a point at either end) along
/// `guides`, each from a corner of the first section to a corner of the last (cadrs's Loft with
/// guides).
pub fn guided_loft(
    k: &mut DefaultKernel,
    sections: Vec<LoftSection>,
    guides: Vec<Vec<PathCurve>>,
) -> Result<BodyId, Refusal> {
    Ok(k.loft_guided(&LoftSpec::solid(sections), &guides)?.first()?)
}
