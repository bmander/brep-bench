//! The half of the cylinder of radius 5 about the z axis from z = 0 to z = 10 where y ≥ 0, every
//! one of its eight edges rounded to radius 1 (the two half-circle rims, the two straight edges
//! where the round face meets the flat one, the four where the flat face meets the ends), three
//! rounds meeting at each of the four corners.
//!
//! The half disk (the arc from 0° to 180° closed by its diameter) sketched on the Top plane and
//! extruded 10; its edges, but for any seam, rounded in one Fillet feature, radius 1, as in
//! `fillet/box-all-edges`.
use crate::bench::{arc, extrude, p2};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let half = Loop::new(vec![arc(p2(0.0, 0.0), 5.0, 0.0, 180.0), Curve2::line(p2(-5.0, 0.0), p2(5.0, 0.0))]);
    let bar = extrude(k, &Profile::of(Plane::xy(0.0), vec![half]), 10.0)?;
    let edges = k.edges_but_seams(bar)?;
    fillet(k, bar, &edges, 1.0)
}
