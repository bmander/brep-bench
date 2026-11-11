//! A face gear hobbed whole by a tapered single-start worm: its thread a rack's trapezoid carried
//! along a conical helix, each section in its half-plane through the axis.
//!
//! Refused: cad-kernel screws a section about an axis at a constant radius (`build::screwed`,
//! `build::carried`'s screw, turn and slide), and has no sweep that carries one outward as it
//! screws, so the tapered thread itself is not built; and its arrangement cuts the sweep from a
//! blank under a turn seen from another turn only for a convex solid of revolution.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no tapered thread (it screws a section only at a constant radius, \
         `build::screwed`), and cuts a sweep under a turn seen from another turn only for a convex \
         solid of revolution"
        .into())
}
