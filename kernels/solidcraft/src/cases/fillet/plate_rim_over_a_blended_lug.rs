use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = block((-40.0, -25.0, 0.0), (40.0, 25.0, 13.0))?;
    let corners = [v3(40.0, 25.0, 6.5), v3(-40.0, 25.0, 6.5), v3(-40.0, -25.0, 6.5), v3(40.0, -25.0, 6.5)];
    let plate = solidcraft_kernel::fillet(&plate, &corners, 10.0)?;
    let lugged = union(&plate, &zcyl(0.0, 25.0, 9.0, 0.0, 13.0)?)?;
    // the two uprights where the lug's side meets the north side, blended
    let blended = solidcraft_kernel::fillet(&lugged, &[v3(9.0, 25.0, 6.5), v3(-9.0, 25.0, 6.5)], 8.0)?;
    // the bottom rim, picked on the south side and followed all round
    let rim = solidcraft_kernel::tangent_chain(&blended, &[v3(0.0, -25.0, 0.0)])?;
    solidcraft_kernel::fillet(&blended, &rim, 2.0)
}
