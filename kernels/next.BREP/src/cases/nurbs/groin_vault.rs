//! A groin vault: the common part of two blocks on the square −12 ≤ x, y ≤ 12 with weighted
//! barrel-vaulted tops (the block of `nurbs/vault-tangent-plane`), the second the first turned a
//! quarter turn about the z axis, so that the two arches cross at right angles.
use crate::solids::sweep::{block_under, moved, turn_z};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation, BooleanOperation, BooleanOptions, BrepSolid};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let first = block_under(&super::vault_tangent_plane::vault()?)?;
    let second = moved(&first, turn_z(FRAC_PI_2, p(0.0, 0.0, 0.0))?)?;
    Ok(boolean_operation(&first, &second, BooleanOperation::Intersect, &BooleanOptions::default())?)
}
