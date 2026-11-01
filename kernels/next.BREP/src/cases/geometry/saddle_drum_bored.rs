//! The drum of `geometry/saddle-drum` (radius 8, floor z = 0, top the bilinear B-spline saddle
//! z = 10 + xy/8 on (−10, −10, 22.5), (10, −10, −2.5), (−10, 10, −2.5), (10, 10, 22.5)), bored
//! through along its axis by a cylinder of radius 3 running past the floor and the saddle.
use crate::solids::geometry::under_saddle;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_cylinder_brep, BooleanOperation, BooleanOptions, BrepSolid,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let options = BooleanOptions::default();
    let block = under_saddle()?;
    let drum = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 8.0, 30.0)?;
    let drum = boolean_operation(&block, &drum, BooleanOperation::Intersect, &options)?;
    let bore = make_cylinder_brep(p(0.0, 0.0, -1.0), p(0.0, 0.0, 1.0), 3.0, 32.0)?;
    Ok(boolean_operation(&drum, &bore, BooleanOperation::Subtract, &options)?)
}
