//! The ring sector 8 ≤ ρ ≤ 12, −1 ≤ z ≤ 1, 0° to 60° about the z axis, less everything a leaning
//! bar passes through as it turns rigidly about the z axis from −140° to +140°: in the plane y = 0
//! at the turn's start, its inner side the plane through (10, 0, 0) square to
//! n = (cos 20°, 0, −sin 20°), the bar running 3 beyond it along n, 2 either side of y = 0, and 3
//! either way along l = (sin 20°, 0, cos 20°).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the bar as a convex polyhedron, its
//! eight corners and six faces, turned about the z axis from −140° to +140°, asked for where it
//! lies in the ring sector (`sweep_solid_clipped`), and cut from it.
use crate::solids::p;
use crate::solids::swept::{box_tool, less_all, ring_sector, turn_about_z};
use crate::Refusal;
use brep_kernel::{sweep_solid_clipped, BrepSolid, SweepClip};

pub fn build() -> Result<BrepSolid, Refusal> {
    let lean = 20f64.to_radians();
    let (n, l) = (p(lean.cos(), 0.0, -lean.sin()), p(lean.sin(), 0.0, lean.cos()));
    let corner = p(10.0, -2.0, 0.0).add(l.scale(-3.0));
    let bar = box_tool(corner, n.scale(3.0), p(0.0, 4.0, 0.0), l.scale(6.0));
    let sector = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [8.0, 12.0],
        axial: [-1.0, 1.0],
        angle: Some([0.0, 60f64.to_radians()]),
    };
    let turn = turn_about_z((-140f64).to_radians(), 140f64.to_radians(), 0.0);
    let block = ring_sector(8.0, 12.0, -1.0, 1.0, 0.0, 60f64.to_radians())?;
    less_all(block, vec![sweep_solid_clipped(&bar, &turn, &sector)?])
}
