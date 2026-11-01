//! A hollow square duct bent a quarter turn along the arc of radius 30 about the z axis from
//! (30, 0, 0) to (0, 30, 0): the 18 × 18 square with its 14 × 14 hole, standing in the xz plane at
//! x = 30, carried along the arc by the path's own motion (`SectionPlacement::Rigid`, the Sweep
//! feature's `pathAlign`), which turns it about the z axis. As the Sweep feature does with a hole,
//! the outer square and the hole are each swept along the path and the hole's sweep cut away.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation, make_arc, sweep_profile_along_chain, BooleanOperation, BooleanOptions,
    BrepSolid, SectionPlacement, SweepPath,
};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let arc = make_arc(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 30.0, 0.0, FRAC_PI_2)?;
    let path = SweepPath::new(vec![arc], vec![])?;
    let square = |h: f64| {
        polygon(&[p(30.0 - h, 0.0, -h), p(30.0 + h, 0.0, -h), p(30.0 + h, 0.0, h),
            p(30.0 - h, 0.0, h)])
    };
    let sweep = |h: f64| -> Result<BrepSolid, Refusal> {
        Ok(sweep_profile_along_chain(&square(h)?, &path, 0.0, None, None,
            SectionPlacement::Rigid, "")?)
    };
    let (outside, hole) = (sweep(9.0)?, sweep(7.0)?);
    Ok(boolean_operation(&outside, &hole, BooleanOperation::Subtract, &BooleanOptions::default())?)
}
