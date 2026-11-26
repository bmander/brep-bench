//! A T-block, the outline through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4)
//! and (0, 4) extruded from z = 0 to z = 8, its eight sides drafted 45° with the pull direction
//! +z about the neutral plane z = 0, inward as they rise: they meet in ridges over the bar and
//! the stem, below the top, which is gone.
//!
//! kentron's `draft` (`shared::draft::outline_drafted`), which refuses it: it closes a part over
//! sides drafted until they meet only where each drafted plane bounds the whole part, and the
//! T's inner sides do not.
use crate::shared::draft::outline_drafted;
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 4.0), (13.0, 4.0), (13.0, 16.0), (7.0, 16.0), (7.0, 4.0), (0.0, 4.0)];
    outline_drafted(m, &outline, 8.0, 45.0)
}
