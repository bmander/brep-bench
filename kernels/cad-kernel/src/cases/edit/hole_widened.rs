//! The box from the origin to (40, 30, 8), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15), the hole's wall then offset 2 into the plate, away
//! from its axis, to the cylinder of radius 7.
use crate::sketch::{block, cylinder, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::normal_sweep;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the drill runs past the floor and the top
    let drill = cylinder([20., 15., -1.], 5., 10.)?;
    let plate = combined(&block([0., 0., 0.], [40., 30., 8.])?, &drill, Op::Cut, 0.)?;
    // cad-kernel offsets no single face of a solid. The faces beside the wall, the top and the
    // floor, are square to it, so the widened plate is the plate less what the wall sweeps
    // through as it moves: its sweep along its normal, which points out of the material toward
    // the axis, taken the other way, from 2 into the plate back to the wall.
    let wall = faces_through(&plate, &[[25., 15., 4.]])?[0];
    Ok(combined(&plate, &normal_sweep(&plate, wall, [-2., 0.])?, Op::Cut, 0.)?)
}
