use super::vault_tangent_plane::vault;
use crate::solids::*;
use std::f64::consts::FRAC_PI_2;

/// The vaulted block of `nurbs/vault-tangent-plane` in common with itself turned a quarter turn
/// about the z axis. (The kernel declines the common part: its exact fallback for faces that
/// touch, here the two vaults at the crown's crossing, takes planar faces only.)
pub fn build() -> R<Body> {
    let first = vault()?;
    let turned = solidcraft_kernel::transform(&first, Vec3::ZERO, Vec3::ZERO, Vec3::Z, FRAC_PI_2)?;
    common(&first, &turned)
}
