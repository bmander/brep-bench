//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all
//! eight of its edges rounded to radius 3/2 in one fillet. Each edge is selected by its middle.
//!
//! The pyramid is the kernel's pyramid on a polygon (`make_pyramid_on_polygon`): the rectangle's
//! four corners and the apex.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_pyramid_on_polygon, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let base = [p(-10.0, -6.0, 0.0), p(10.0, -6.0, 0.0), p(10.0, 6.0, 0.0), p(-10.0, 6.0, 0.0)];
    let pyramid = make_pyramid_on_polygon(&base, p(0.0, 0.0, 12.0))?;
    let mut middles = vec![p(0.0, -6.0, 0.0), p(10.0, 0.0, 0.0), p(0.0, 6.0, 0.0),
        p(-10.0, 0.0, 0.0)];
    for (x, y) in [(5.0, 3.0), (-5.0, 3.0), (-5.0, -3.0), (5.0, -3.0)] {
        middles.push(p(x, y, 6.0));
    }
    Ok(fillet_edges(&pyramid, &middles, None, 1.5, false, None)?)
}
