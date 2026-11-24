//! The box from the origin to (40, 30, 10), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15), its top face then moved up by 6, to z = 16, the four
//! sides and the hole's wall running on up to meet it.
use crate::sketch::{block, cylinder, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::normal_sweep;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the drill runs past the floor and the top
    let drill = cylinder([20., 15., -1.], 5., 12.)?;
    let drilled = combined(&block([0., 0., 0.], [40., 30., 10.])?, &drill, Op::Cut, 0.)?;
    // cad-kernel moves no face of a solid. Every face beside the top (the four sides, the hole's
    // wall) is square to it, so carried on along the move they bound exactly what the top sweeps
    // through as it moves: the moved solid is the solid with the top's sweep along its normal
    // added.
    let top = faces_through(&drilled, &[[5., 5., 10.]])?[0];
    Ok(combined(&drilled, &normal_sweep(&drilled, top, [0., 6.])?, Op::Union, 0.)?)
}
