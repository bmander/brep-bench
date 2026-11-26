//! The plus-shaped plate on the outline (3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9),
//! (3, 9), (3, 6), (0, 6), (0, 3), (3, 3), 4 thick, shrunk by 1: its convex edges sharp, its four
//! inside corners' edges concave quarter cylinders.
use springrep::topo::Brep;

use crate::shared::offset::shrunk;
use crate::shared::shell::prism;

pub fn build() -> Result<Brep, String> {
    let outline = [
        [3., 0.], [6., 0.], [6., 3.], [9., 3.], [9., 6.], [6., 6.],
        [6., 9.], [3., 9.], [3., 6.], [0., 6.], [0., 3.], [3., 3.],
    ];
    let plate = prism(&outline, &["wall"; 12], 4., "plate")?;
    shrunk(&plate, 1.)
}
