//! What a torus about the z axis (middle circle 6, tube 2) fills as it slides 15. along x without
//! turning, farther than its hole is wide.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{at, e, region, rod, stadium, torus, unite_all};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. A torus slid in its own plane is the tube of radius 2
    // about what its middle circle sweeps: the flat stadium within 6 of the slide's segment, so
    // the slab of that region 2 thick, the torus at each end, and a rod of radius 2 along each of
    // the stadium's straight sides
    let d = 15.;
    let stadium = region(&[stadium((0., 0.), (d, 0.), 6.)])?;
    let slab = extrude::extrude(&Frame::world(), &stadium, -2., 2.).map_err(e)?;
    // each joined to what is built so far, which it overlaps
    let pieces = [
        torus(&at([0., 0., 0.]), 6., 2.),
        torus(&at([d, 0., 0.]), 6., 2.),
        rod([0., -6., 0.], [d, -6., 0.], 2.),
        rod([0., 6., 0.], [d, 6., 0.], 2.),
    ];
    unite_all(slab, pieces)
}
