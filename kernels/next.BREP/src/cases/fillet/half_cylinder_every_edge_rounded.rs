//! The half of the cylinder of radius 5 about the z axis from z = 0 to z = 10 where y ≥ 0, every
//! one of its six edges rounded to radius 1 in one fillet, which rounds each corner where three
//! rounds meet. Each edge is selected by its middle.
//!
//! The half cylinder is the half disk (the arc of radius 5 from (5, 0) over (0, 5) to (−5, 0) and
//! the chord back) extruded 10 along +z.
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut c = Chain::new();
    c.arc((0.0, 0.0), 5.0, deg(0.0), deg(180.0), true)?.line((-5.0, 0.0), (5.0, 0.0))?;
    let half = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 10.0)?;
    // the rims and chords at both ends, and the two straight edges where the round face meets the
    // flat one
    let mut middles = c.middles_at(0.0);
    middles.extend(c.middles_at(10.0));
    middles.extend([p(5.0, 0.0, 5.0), p(-5.0, 0.0, 5.0)]);
    Ok(fillet_edges(&half, &middles, None, 1.0, false, None)?)
}
