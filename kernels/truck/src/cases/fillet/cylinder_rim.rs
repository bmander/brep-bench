//! "A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Round the circular edge around its
//! top face to radius 1."
//!
//! truck cannot build it. truck draws a circle as two edges or more (truck-topology's `Edge` takes
//! no closed edge: `Edge::try_new(&v, &v, …)` is `SameVertex`), so the rim is a chain of arcs, each
//! running smoothly into the next. truck-shapeops rounds one edge at a time (`fillet_with_side`);
//! rounding one arc leaves the next one sharp, and its fillet along a chain of edges is commented
//! out in its source.
use crate::{Built, Refused};

pub fn build() -> Built {
    Err(Refused(
        "truck-shapeops rounds only a single edge (`fillet_with_side`), and the rim is a chain of \
         arcs (truck-topology takes no closed edge); its fillet along a chain of edges is \
         unsupported (commented out in truck-shapeops)"
            .into(),
    ))
}
