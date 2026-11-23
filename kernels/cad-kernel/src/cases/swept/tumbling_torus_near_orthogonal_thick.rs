//! A thicker torus under the nearly perpendicular tumble. See cases/swept/tumbling-torus-near-orthogonal-thick.md for the continuous solid.
//!
//! Swept by the kernel's road for a ring under a motion no constant twist takes
//! (`swept::closed::ring`, through `crate::tumble`), which builds the half-turn, full-turn and thin
//! tumbling tori; it refuses this one at run time in cad-kernel 0.21.0 (that road, #942, is still
//! in progress): it fits a tube whose B-spline's pieces do not close
//! (`Unbuilt::Failed`).
use crate::tumble::Tumble;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Tumble {
        centre: [2.02, 0., 1.],
        hub: 2., tube: 0.16, ratio: 1.01,
        lean: 1., heading: 89., height: 0.,
        observer: 1., roll: [-180., 180.],
    }.build()
}
