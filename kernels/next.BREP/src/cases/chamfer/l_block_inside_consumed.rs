//! The L through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) of the xz plane, extruded
//! along +y from y = 0 to y = 10, its inside corner, the line x = 3, z = 3, chamfered with
//! distance 7 on both faces, adding material: the chamfer is the plane through the faces' far
//! edges, (10, y, 3) and (3, y, 10), and consumes both.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 3.0),
        p(3.0, 0.0, 3.0), p(3.0, 0.0, 10.0), p(0.0, 0.0, 10.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 1.0, 0.0), 10.0)?;
    // A right-angled edge: the chamfer's distance is its setback on both faces.
    Ok(fillet_edges(&block, &[p(3.0, 5.0, 3.0)], None, 7.0, true, None)?)
}
