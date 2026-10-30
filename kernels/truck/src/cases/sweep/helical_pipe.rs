//! "A pipe of circular section, radius 1, following a helix about the z axis. The helix has radius
//! 10 and rises 5 per turn. It starts at (10, 0, 0), turns right-handed (from +x towards +y) and
//! makes three whole turns. The section stays square to the path, and the pipe's ends are flat."
//!
//! truck cannot build it. truck-geometry has no helical curve, and truck-modeling's builder sweeps
//! only by translation (`tsweep`) and by rotation (`rsweep`, `cone`); it has no sweep along a path,
//! and no sweep that turns and rises at once.
use crate::{Built, Refused};

pub fn build() -> Built {
    Err(Refused(
        "truck has no helical curve, and truck-modeling sweeps only by translation (`tsweep`) and \
         rotation (`rsweep`), never along a path"
            .into(),
    ))
}
