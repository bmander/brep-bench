//! The cone on the z axis from its base of radius 6 at z = 0 towards its apex at z = 12, its tip
//! rounded by the ball of radius 1 tangent to its side, shrunk by 1.5: every point at least 1.5
//! from its outside, SolidCraft's offset of the whole solid inward (`offset_solid`, rounded), the
//! moved faces that cross trimmed and merged.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_PI_2;

/// The ball-tipped cone, its half-section in x and z turned about the z axis: the base, the side
/// up to the circle of radius 2/√5 at z = 12 − 4/√5 where the ball touches it, the ball's arc
/// about (0, 12 − √5) on to the axis, and the axis back down.
pub fn ball_tipped_cone() -> R<Body> {
    let s5 = 5f64.sqrt();
    let centre = 12.0 - s5;
    // the side touches the ball at the angle atan2(1, 2) from +x towards +z
    let touch = 1f64.atan2(2.0);
    let (o, rim, tangent, tip) = (v2(0.0, 0.0), v2(6.0, 0.0), v2(2.0 / s5, 12.0 - 4.0 / s5), v2(0.0, centre + 1.0));
    let segs = vec![
        Seg2::Line { a: o, b: rim },
        Seg2::Line { a: rim, b: tangent },
        Seg2::Arc { center: v2(0.0, centre), radius: 1.0, start: touch, sweep: FRAC_PI_2 - touch },
        Seg2::Line { a: tip, b: o },
    ];
    revolve_z(Region2 { outer: Loop2 { segs }, holes: vec![] }, 360.0)
}

pub fn build() -> R<Body> {
    solidcraft_kernel::offset_solid(&ball_tipped_cone()?, -1.5, true)
}
