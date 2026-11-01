//! Everything the box from (0, 0, 0) to (10, 6, 4) passes through as it slides without turning
//! along the vector (8, 5, 3).
//!
//! next.BREP has no sweep of a solid. A box slid along a line is, by elementary geometry, the box
//! at each end and each of its faces slid along the line: the boxes are the kernel's boxes, each
//! face slid is its rectangle extruded along the vector, and the solid is their union.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, extrude_profile_brep, make_box_brep, BooleanOperation, BrepSolid,
    Vec3,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let slide = p(8.0, 5.0, 3.0);
    let (lo, hi) = (p(0.0, 0.0, 0.0), p(10.0, 6.0, 4.0));
    let mut parts = vec![
        make_box_brep(lo, 10.0, 6.0, 4.0)?,
        make_box_brep(slide, 10.0, 6.0, 4.0)?,
    ];
    // the corner of the box picked by one of the three coordinates of `lo` or `hi` each
    let corner = |x: bool, y: bool, z: bool| {
        let pick = |high: bool, l: f64, h: f64| if high { h } else { l };
        Vec3::new(pick(x, lo.x, hi.x), pick(y, lo.y, hi.y), pick(z, lo.z, hi.z))
    };
    for side in [false, true] {
        let faces = [
            [corner(side, false, false), corner(side, true, false), corner(side, true, true),
                corner(side, false, true)],
            [corner(false, side, false), corner(true, side, false), corner(true, side, true),
                corner(false, side, true)],
            [corner(false, false, side), corner(true, false, side), corner(true, true, side),
                corner(false, true, side)],
        ];
        for face in faces {
            parts.push(extrude_profile_brep(&polygon(&face)?, slide, slide.length())?);
        }
    }
    Ok(boolean_operation_nary(&parts, BooleanOperation::Union)?)
}
