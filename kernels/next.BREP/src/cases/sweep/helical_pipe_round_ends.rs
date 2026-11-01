//! The helical pipe of `sweep/helical-pipe` (radius 1, along the helix of radius 10 about the z
//! axis rising 5 a turn, from (10, 0, 0) for three right-handed turns) with each end closed by a
//! half ball of radius 1: the pipe united with a ball of radius 1 at each end of the helix, whose
//! other half lies inside the pipe.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_sphere_brep, BooleanOperation, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let pipe = super::helical_pipe::pipe()?;
    let up = p(0.0, 0.0, 1.0);
    let start = make_sphere_brep(p(10.0, 0.0, 0.0), 1.0, up)?;
    let end = make_sphere_brep(p(10.0, 0.0, 15.0), 1.0, up)?;
    Ok(boolean_operation_nary(&[pipe, start, end], BooleanOperation::Union)?)
}
