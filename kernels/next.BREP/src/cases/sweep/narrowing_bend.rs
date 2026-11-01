//! A hollow square duct narrowing round a quarter bend of radius 30 about the z axis, from
//! (30, 0, 0) to (0, 30, 0): its section, in the vertical plane through the z axis, the square of
//! half-width h = 9 − 3·θ/90° centred on the arc, with a hole of half-width h − 2.
//!
//! next.BREP has no sweep with a scaling law. Its loft along a guide that turns its sections with
//! the guide (`loft_profile_brep_guided_frame`, the Loft feature's `rotateToGuide`) carries the
//! start section into the end one, blending the two linearly in the guide's parameter in the
//! guide's turning frame; that is the duct when the guide's parameter runs with the angle turned.
//! The kernel's arc is rational, its parameter not proportional to the angle, so the guide is the
//! kernel's own helix fit (`fit_helix_curve`) with no rise: the quarter circle of radius 30
//! sampled and parameterized uniformly in angle. The outside and the hole are each lofted along
//! it, and the hole cut away, as the Loft feature does with a hole.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, fit_helix_curve, loft_profile_brep_guided_frame, BooleanOperation,
    BooleanOptions, BrepSolid, NurbsCurve, Vec3,
};

/// The square of half-width `h` about the arc's point in the radial direction `out`.
fn square(out: Vec3, h: f64) -> Result<Vec<NurbsCurve>, String> {
    let at = |r: f64, z: f64| out.scale(r).add(p(0.0, 0.0, z));
    polygon(&[at(30.0 - h, -h), at(30.0 + h, -h), at(30.0 + h, h), at(30.0 - h, h)])
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let guide = fit_helix_curve(p(0.0, 0.0, 0.0), z, Some(x), 30.0, 30.0, 0.0, 0.25, 0.0, false)?;
    let loft = |start: f64, end: f64| -> Result<BrepSolid, Refusal> {
        Ok(loft_profile_brep_guided_frame(&[square(x, start)?, square(y, end)?], &guide, None)?)
    };
    let (outside, hole) = (loft(9.0, 6.0)?, loft(7.0, 4.0)?);
    Ok(boolean_operation(&outside, &hole, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
