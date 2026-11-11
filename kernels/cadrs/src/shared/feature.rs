//! What the cadrs implementations of the feature cases share: extrudes and revolves whose end is
//! "up to" something rather than a given depth or angle.
use crate::bench::{unit, V3};
use crate::Refusal;
use cadrs_kernel::{
    Axis, BodyId, DefaultKernel, ExtrudeEnd, ExtrudeSpec, Kernel, Profile, RevolveEnd, RevolveSpec,
};

/// `profile` extruded along `dir` to `end`, the bodies of `scene` the ones "Up to next"
/// considers.
pub fn extrude_to(
    k: &mut DefaultKernel,
    profile: &Profile,
    dir: V3,
    end: ExtrudeEnd,
    scene: &[BodyId],
) -> Result<BodyId, Refusal> {
    Ok(k.extrude_with(profile, &ExtrudeSpec::up_to(unit(dir), end, scene))?.first()?)
}

/// `profile` revolved about `axis` (right-handed) to `end`, the bodies of `scene` the ones "Up to
/// next" considers.
pub fn revolve_to(
    k: &mut DefaultKernel,
    profile: &Profile,
    axis: Axis,
    end: RevolveEnd,
    scene: &[BodyId],
) -> Result<BodyId, Refusal> {
    Ok(k.revolve_with(profile, &RevolveSpec::up_to(axis, end, scene))?.first()?)
}
