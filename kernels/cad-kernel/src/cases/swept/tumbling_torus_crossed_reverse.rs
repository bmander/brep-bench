//! The reverse member of a crossed pair of torus sweeps. See cases/swept/tumbling-torus-crossed-reverse.md for the continuous solid.
//!
//! Swept by the kernel's road for a ring under a motion no constant twist takes
//! (`swept::closed::ring`, through `crate::tumble`), which builds the half-turn, full-turn and thin
//! tumbling tori; it refuses this one at run time in cad-kernel 0.21.0 (that road, #942, is still
//! in progress): it cannot read the ring's contact curves at an instant of the roll as the
//! first instant's (`Unbuilt::Curved`).
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

/// The sweep: its torus, its two turns and its roll.
pub fn tumble() -> Tumble {
    Tumble {
        centre: [3., 0., 2.],
        hub: 2., tube: 0.22, ratio: 0.7,
        lean: 45., heading: 20., height: 0.7071,
        observer: -1., roll: [-100., 100.],
    }
}

pub fn build() -> Result<Brep, String> {
    tumble().build()
}
