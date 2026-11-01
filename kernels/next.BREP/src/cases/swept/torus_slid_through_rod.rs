//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a wheel passes
//! through as it slides without turning along the z axis: a torus with its axis parallel to y,
//! the circle through its tube's centres of radius 6 and its tube of radius 2, its centre moving
//! from (11.5, 0, −9) to (11.5, 0, 29).
//!
//! next.BREP has no sweep of a solid; the wheel's sweep along a line follows by elementary
//! geometry (`solids::sweep::torus_slid_up`): the wheel with its hole filled at each end, and
//! between them its outline slid along the line. Each is cut from the rod.
use crate::solids::p;
use crate::solids::sweep::torus_slid_up;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_cylinder_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    operands.extend(torus_slid_up(p(11.5, 0.0, -9.0), 6.0, 2.0, 38.0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
