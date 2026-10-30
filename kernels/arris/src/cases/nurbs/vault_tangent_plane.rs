//! The block under the weighted barrel vault of `nurbs/groin-vault`, cut by a box whose floor is
//! the plane z = 16, which touches the vault along its crown, so the cut removes nothing.
use super::groin_vault::vault;
use crate::Refusal;
use arris::math::{Control, Point3};
use arris::ops::{cut, primitive_box};
use arris::topo::{Body, Model};

const HALF: f64 = 12.0;
const CROWN: f64 = 16.0;
/// How far the box reaches past the block on every side but its floor.
const OVERRUN: f64 = 4.0;

// workaround: no solid from a NURBS surface
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let block = vault(m)?;
    let reach = HALF + OVERRUN;
    let (lid, _) = primitive_box(
        m,
        Point3::new(-reach, -reach, CROWN),
        Point3::new(reach, reach, CROWN + 2.0 * OVERRUN),
        &Control::NONE,
    )?;
    let (cut_block, _) = cut(m, block, lid, &Control::NONE)?;
    Ok(cut_block)
}
