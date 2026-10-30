//! The block under the weighted barrel vault of `nurbs/groin-vault`, cut by a box whose floor is
//! the plane z = 16, which touches the vault along its crown, so the cut removes nothing.
use super::groin_vault::vault;
use vcad_kernel::Solid;

const HALF: f64 = 12.0;
const CROWN: f64 = 16.0;
/// How far the box reaches past the block on every side but its floor.
const OVERRUN: f64 = 4.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let block = vault()?;
    let reach = HALF + OVERRUN;
    let lid = Solid::cube(2.0 * reach, 2.0 * reach, 2.0 * OVERRUN).translate(-reach, -reach, CROWN);
    Ok(block.try_difference(&lid)?)
}
