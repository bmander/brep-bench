//! The box from the origin to (40, 30, 8), drilled right through along z by two holes of radius 3
//! about the vertical lines through (15, 15) and (25, 15), both walls then offset 3 into the plate,
//! away from their axes, to cylinders of radius 6 that overlap: one opening, no wall between.
use crate::sketch::{block, cylinder, faces_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::offset::normal_sweep;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the drills run past the floor and the top
    let mut plate = block([0., 0., 0.], [40., 30., 8.])?;
    for x in [15., 25.] {
        let drill = cylinder([x, 15., -1.], 3., 10.)?;
        plate = combined(&plate, &drill, Op::Cut, 0.)?;
    }
    // cad-kernel offsets no single face of a solid. The top and the floor are square to each
    // wall, so the widened plate is the plate less what each wall sweeps through as it moves: its
    // sweep along its normal, which points toward its axis, taken the other way, from 3 into the
    // plate back to the wall. The two sweeps overlap between the holes, so cutting both leaves no
    // wall there: each wall is trimmed where it meets the other.
    let walls = faces_through(&plate, &[[12., 15., 4.], [28., 15., 4.]])?;
    let sweeps =
        walls.iter().map(|&w| normal_sweep(&plate, w, [-3., 0.])).collect::<Result<Vec<_>, _>>()?;
    for sweep in &sweeps {
        plate = combined(&plate, sweep, Op::Cut, 0.)?;
    }
    Ok(plate)
}
