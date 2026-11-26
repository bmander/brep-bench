//! A 40 by 30 by 6 plate, all twelve edges filleted to radius 3 in one operation.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 6.0)?;
    let mut middles = Vec::new();
    for y in [0.0, 30.0] {
        for z in [0.0, 6.0] { middles.push(p(20.0, y, z)); }
    }
    for x in [0.0, 40.0] {
        for z in [0.0, 6.0] { middles.push(p(x, 15.0, z)); }
        for y in [0.0, 30.0] { middles.push(p(x, y, 3.0)); }
    }
    Ok(fillet_edges(&plate, &middles, None, 3.0, false, None)?)
}
