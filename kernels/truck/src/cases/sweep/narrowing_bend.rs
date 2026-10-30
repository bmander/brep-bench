//! "A hollow square duct that narrows as it bends through a quarter turn. Its centreline is an arc
//! of radius 30 in the xy plane about the z axis, from (30, 0, 0) to (0, 30, 0). At every point of
//! the arc the section lies in the vertical plane through the z axis and that point. The section's
//! outer half-width h runs linearly with the angle turned, from 9 at the start to 6 at the end:
//! at an angle θ from the xz plane, h = 9 − 3·θ/90°. [The hole is the square of half-width
//! h − 2.] The two ends are flat."
//!
//! truck cannot build it. truck-modeling's builder sweeps a section only unchanged, by translation
//! (`tsweep`) and by rotation (`rsweep`, `cone`); it has no sweep whose section changes along the
//! path. Here the section shrinks as it turns, so each corner traces a spiral, its distance from
//! the axis and its height linear in the angle: no curve truck-geometry has (line, B-spline,
//! NURBS) is such a spiral, and the loft `try_wire_homotopy` joins two sections only by straight
//! lines, which would make a different solid.
use crate::{Built, Refused};

pub fn build() -> Built {
    Err(Refused(
        "truck-modeling sweeps a section only unchanged, by translation (`tsweep`) or rotation \
         (`rsweep`); this section shrinks as it turns, its corners tracing spirals that no truck \
         curve (line, B-spline, NURBS) represents, and `try_wire_homotopy` lofts only along \
         straight lines"
            .into(),
    ))
}
