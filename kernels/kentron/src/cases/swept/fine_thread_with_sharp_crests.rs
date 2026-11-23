//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar
//! carried along a fine screw: the box 4.5 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 0.45, screwed about the z axis,
//! right-handed, rising 1 a turn, from a turn of −180° to +7380°. Everything the bar passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, so the
//! groove is built as elementary geometry gives it: screwed into itself, it is its section at
//! z = 0 screwed up the rod, and that section is what the bar's outline sweeps turning about the
//! z axis while the bar holds z = 0 (`shared::swept::rod_screw_cut_by_bar`). The screw starts
//! with the bar wholly below the rod and ends with it wholly above.
use crate::shared::swept::rod_screw_cut_by_bar;
use crate::Refusal;
use kentron::math::Point2;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = vec![Point2::new(4.5, -8.0), Point2::new(12.0, -8.0), Point2::new(12.0, 8.0), Point2::new(4.5, 8.0)];
    rod_screw_cut_by_bar(m, 5.0, 20.0, outline, 0.9, 1.0)
}
