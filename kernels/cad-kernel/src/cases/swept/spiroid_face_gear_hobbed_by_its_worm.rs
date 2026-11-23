//! A face gear hobbed whole by a tapered single-start worm: its thread a rack's trapezoid carried
//! along a conical helix, each section in its half-plane through the axis.
//!
//! Refused: the tapered thread itself is not built. cad-kernel's sweeps of a section carry it only
//! by a slide, a turn or a screw (`build::carried`'s `geom::Carry::{Line, Screw}`,
//! `build::screwed`, `build::revolve`), each at a constant radius from its axis; none carries a
//! section outward as it screws, and `build::lofted`/`build::sheets` take caller-made nets, which
//! would be geometry computed outside the kernel. Nor would the cut follow: the kernel's sweep cut
//! from a blank (`swept::closed::cut_from`) under a turn seen from another turn reads only a convex
//! solid of revolution (K2) or a tool all planes and lines (`swept::general`), and refuses a
//! thread's curved flanks (`Unbuilt::Curved`), as it does the constant-radius worm's in
//! `worm-wheel-hobbed-by-its-worm` and `helicon-face-gear-hobbed-by-its-worm`.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no tapered thread: it carries a section only by a slide, a turn or a screw \
         at a constant radius (`build::carried`, `build::screwed`); and its sweep cut from a blank \
         under a turn seen from another turn refuses a thread's curved flanks (`Unbuilt::Curved`)"
        .into())
}
