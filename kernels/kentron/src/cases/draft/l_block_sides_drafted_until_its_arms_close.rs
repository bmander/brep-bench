//! An L-block, the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded from z = 0 to z = 8, its six sides drafted 30° with the pull direction +z about the
//! neutral plane z = 0, inward as they rise: they meet in ridges over each arm, below the top,
//! which is gone.
//!
//! kentron's `draft` (`shared::draft::outline_drafted`), which refuses it: it closes a part over
//! sides drafted until they meet only where each drafted plane bounds the whole part, and the
//! L's two inner sides do not.
use crate::shared::draft::outline_drafted;
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    outline_drafted(m, &outline, 8.0, 30.0)
}
