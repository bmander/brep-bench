//! "A pipe of circular section, radius 1, following a helix about the z axis. The helix has radius
//! 10 and rises 5 per turn. It starts at (10, 0, 0), turns right-handed (from +x towards +y) and
//! makes three whole turns. The section stays square to the path. Each end is closed by a half
//! ball of radius 1, as a ball rolled along the helix would leave."
//!
//! truck cannot build it: the pipe itself is out of reach (see `helical_pipe`). truck-geometry has
//! no helical curve, and truck-modeling's builder sweeps only by translation (`tsweep`) and by
//! rotation (`rsweep`, `cone`), never along a path.
use crate::{Built, Refused};

pub fn build() -> Built {
    Err(Refused(
        "truck has no helical curve, and truck-modeling sweeps only by translation (`tsweep`) and \
         rotation (`rsweep`), never along a path"
            .into(),
    ))
}
