//! A block grooved by a square key carried along a spline.
//!
//! Through the kernel's sweep of a prism carried by a frame along a curve (`sweep_prism_along`):
//! the curve is the cubic Bézier (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0), the frame's up the
//! z axis; the key, drawn in the frame at the curve's start, is the rectangle −1.5 ≤ x ≤ 1.5,
//! −0.8 ≤ y ≤ 0.8 from z = −1.5 to 2.5. The motion is planar, so the kernel finds the plane region
//! the moving rectangle covers (its corners' paths, its sides' grazing points' paths and its end
//! positions, split where they meet) and extrudes it; cut from the block, the box from (−6, −6, −8)
//! to (36, 26, 0).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, sweep_prism_along, BooleanOperation, BrepSolid, FrameCurve};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 42.0, 32.0, 8.0)?;
    let path = FrameCurve {
        control: [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(20.0, 20.0, 0.0), p(30.0, 20.0, 0.0)],
        up: p(0.0, 0.0, 1.0),
    };
    let key = sweep_prism_along(&[[-1.5, -0.8], [1.5, -0.8], [1.5, 0.8], [-1.5, 0.8]], -1.5, 2.5, &path)?;
    Ok(boolean_operation_nary(&[block, key], BooleanOperation::Subtract)?)
}
