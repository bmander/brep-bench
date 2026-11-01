//! The rod of radius 5 about the z axis from z = 0 to z = 20, less everything a wheel of two beads
//! passes through as it slides without turning 40 up the z axis. The beads are tori on one axis
//! parallel to y through (11.5, 0, −9): the first centred there, of radii 6 and 2, the second
//! centred 1.5 further along the axis, of radii 6 and 1.5; the wheel is their union.
//!
//! next.BREP has no sweep of a solid. What a union sweeps is the union of what its parts sweep,
//! and each bead's sweep along a line follows by elementary geometry
//! (`solids::sweep::torus_slid_up`): the bead with its hole filled at each end, and between them
//! its outline slid along the line. Each is cut from the rod.
use crate::solids::p;
use crate::solids::sweep::torus_slid_up;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_cylinder_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    operands.extend(torus_slid_up(p(11.5, 0.0, -9.0), 6.0, 2.0, 40.0)?);
    operands.extend(torus_slid_up(p(11.5, 1.5, -9.0), 6.0, 1.5, 40.0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
