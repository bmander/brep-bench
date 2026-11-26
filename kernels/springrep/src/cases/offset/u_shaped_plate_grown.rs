//! The U-shaped plate on the outline (0, 0), (9, 0), (9, 9), (6, 9), (6, 3), (3, 3), (3, 9),
//! (0, 9), 2 thick, grown by 1: its convex edges rounded, the slot's two inside corners' edges
//! sharp.
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::shell::prism;

pub fn build() -> Result<Brep, String> {
    let outline = [
        [0., 0.], [9., 0.], [9., 9.], [6., 9.], [6., 3.], [3., 3.], [3., 9.], [0., 9.],
    ];
    let plate = prism(&outline, &["wall"; 8], 2., "plate")?;
    grown(&plate, 1.)
}
